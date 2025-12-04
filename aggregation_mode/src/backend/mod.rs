pub mod config;
pub mod fetcher;
mod merkle_tree;
mod retry;
mod s3;
mod types;

use crate::aggregators::{AlignedProof, ProofAggregationError, ZKVMEngine};

use alloy::{
    consensus::{BlobTransactionSidecar, EnvKzgSettings, EthereumTxEnvelope, TxEip4844WithSidecar},
    eips::{eip4844::BYTES_PER_BLOB, eip7594::BlobTransactionSidecarEip7594, Encodable2718},
    hex,
    network::EthereumWallet,
    primitives::Address,
    providers::{PendingTransactionError, Provider, ProviderBuilder},
    rpc::types::TransactionReceipt,
    signers::local::LocalSigner,
};
use config::Config;
use ethers::types::U256;
use ethers::utils::parse_ether;
use fetcher::{ProofsFetcher, ProofsFetcherError};
use merkle_tree::compute_proofs_merkle_root;
use risc0_ethereum_contracts::encode_seal;
use std::thread::sleep;
use std::{str::FromStr, time::Duration};
use tracing::{error, info, warn};
use types::{AlignedProofAggregationService, AlignedProofAggregationServiceContract, RPCProvider};

#[derive(Debug)]
pub enum AggregatedProofSubmissionError {
    BuildingBlobCommitment,
    BuildingBlobProof,
    BuildingBlobVersionedHash,
    Risc0EncodingSeal(String),
    SendVerifyAggregatedProofTransaction(String),
    ReceiptError(PendingTransactionError),
    FetchingProofs(ProofsFetcherError),
    ZKVMAggregation(ProofAggregationError),
    BuildingMerkleRoot,
    MerkleRootMisMatch,
    GasPriceError(String),
}

pub struct ProofAggregator {
    engine: ZKVMEngine,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    fetcher: ProofsFetcher,
    config: Config,
    rpc_provider: RPCProvider,
}

impl ProofAggregator {
    pub fn new(config: Config) -> Self {
        let rpc_url: reqwest::Url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
        let signer = LocalSigner::decrypt_keystore(
            config.ecdsa.private_key_store_path.clone(),
            config.ecdsa.private_key_store_password.clone(),
        )
        .expect("Keystore signer should be `cast wallet` compliant");
        let wallet = EthereumWallet::from(signer);

        // Check if the monthly budget is non-negative to avoid runtime errors later
        let monthly_budget_in_wei = parse_ether(config.monthly_budget_eth)
            .expect("Monthly budget must be a non-negative value");

        info!("Monthly budget set to {} wei", monthly_budget_in_wei);

        let rpc_provider = ProviderBuilder::new().connect_http(rpc_url.clone());

        let signed_rpc_provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

        let proof_aggregation_service = AlignedProofAggregationService::new(
            Address::from_str(&config.proof_aggregation_service_address)
                .expect("AlignedProofAggregationService address should be valid"),
            signed_rpc_provider.clone(),
        );

        let engine =
            ZKVMEngine::from_env().expect("AGGREGATOR env variable to be set to one of sp1|risc0");
        let fetcher = ProofsFetcher::new(&config);

        Self {
            engine,
            proof_aggregation_service,
            fetcher,
            config,
            rpc_provider,
        }
    }

    pub async fn start(&mut self) {
        info!("Starting proof aggregator service");

        info!("About to aggregate and submit proof to be verified on chain");
        let res = self.aggregate_and_submit_proofs_on_chain().await;

        match res {
            Ok(()) => {
                self.config
                    .update_last_aggregated_block(self.fetcher.get_last_aggregated_block())
                    .unwrap();
                info!("Process finished successfully");
            }
            Err(err) => {
                error!("Error while aggregating and submitting proofs: {:?}", err);
            }
        }
    }

    async fn aggregate_and_submit_proofs_on_chain(
        &mut self,
    ) -> Result<(), AggregatedProofSubmissionError> {
        let proofs = self
            .fetcher
            .fetch(self.engine.clone(), self.config.total_proofs_limit)
            .await
            .map_err(AggregatedProofSubmissionError::FetchingProofs)?;

        if proofs.is_empty() {
            warn!("No proofs collected, skipping aggregation...");
            return Ok(());
        }

        info!("Proofs fetched, constructing merkle root...");
        let (merkle_tree, leaves) = compute_proofs_merkle_root(&proofs)
            .ok_or(AggregatedProofSubmissionError::BuildingMerkleRoot)?;
        let merkle_root = merkle_tree.root;
        info!("Merkle root constructed: 0x{}", hex::encode(merkle_root));

        info!("Starting proof aggregation program...");
        let (aggregated_proof, zkvm_merkle_root) = self
            .engine
            .aggregate_proofs(proofs, self.config.proofs_per_chunk)
            .map_err(AggregatedProofSubmissionError::ZKVMAggregation)?;
        info!("Proof aggregation program finished");

        info!("Starting Merkle root verification: comparing ZKVM output with off-VM computation");
        if zkvm_merkle_root != merkle_root {
            error!(
                "Merkle root mismatch detected: ZKVM = {zkvm_merkle_root:?}, off-VM = {merkle_root:?}"
            );
            return Err(AggregatedProofSubmissionError::MerkleRootMisMatch);
        }
        info!("Merkle root verification successful: roots match");

        info!("Constructing blob...");
        let (blob, blob_versioned_hash) = self.construct_blob(leaves).await?;
        info!(
            "Blob constructed, versioned hash: {}",
            hex::encode(blob_versioned_hash)
        );

        // Iterate until we can send the proof on-chain
        let mut time_elapsed = Duration::from_secs(24 * 3600);

        loop {
            // We add 24 hours because the proof aggregator runs once a day, so the time elapsed
            // should be considered over a 24h period.

            let gas_price = match self.rpc_provider.get_gas_price().await {
                Ok(price) => Ok(price),
                Err(e1) => Err(AggregatedProofSubmissionError::GasPriceError(
                    e1.to_string(),
                )),
            }?;

            if self.should_send_proof_to_verify_on_chain(
                time_elapsed,
                self.config.monthly_budget_eth,
                gas_price.into(),
            ) {
                break;
            } else {
                info!("Skipping sending proof to ProofAggregationService contract due to budget/time constraints.");
            }

            // Sleep for 3 minutes (15 blocks) before re-evaluating
            let time_to_sleep = Duration::from_secs(180);
            time_elapsed += time_to_sleep;
            sleep(time_to_sleep);
        }

        info!("Sending proof to ProofAggregationService contract...");
        let receipt = self
            .send_proof_to_verify_on_chain(blob, blob_versioned_hash, aggregated_proof)
            .await?;
        info!(
            "Proof sent and verified, tx hash {:?}",
            receipt.transaction_hash
        );

        Ok(())
    }

    fn max_to_spend_in_wei(time_elapsed: Duration, monthly_eth_budget: f64) -> U256 {
        const SECONDS_PER_MONTH: u64 = 30 * 24 * 60 * 60;

        // Note: this expect is safe because in case it was invalid, should have been caught at startup
        let monthly_budget_in_wei = parse_ether(monthly_eth_budget)
            .expect("The monthly budget should be a non-negative value");

        let elapsed_seconds = U256::from(time_elapsed.as_secs());

        let budget_available_per_second_in_wei = monthly_budget_in_wei / SECONDS_PER_MONTH;

        budget_available_per_second_in_wei * elapsed_seconds
    }

    /// Decides whether to send the aggregated proof to be verified on-chain based on
    /// time elapsed since last submission and monthly ETH budget.
    /// We make a linear function with the eth to spend this month and the time elapsed since last submission.
    /// If eth to spend / elapsed time is over the linear function, we skip the submission.
    fn should_send_proof_to_verify_on_chain(
        &self,
        time_elapsed: Duration,
        monthly_eth_budget: f64,
        network_gas_price: U256,
    ) -> bool {
        // We assume a fixed gas cost of 300,000 for each of the 2 transactions
        const ON_CHAIN_COST_IN_GAS_UNITS: u64 = 600_000u64;

        let on_chain_cost_in_gas: U256 = U256::from(ON_CHAIN_COST_IN_GAS_UNITS);
        let max_to_spend_in_wei = Self::max_to_spend_in_wei(time_elapsed, monthly_eth_budget);

        let expected_cost_in_wei = network_gas_price * on_chain_cost_in_gas;

        expected_cost_in_wei <= max_to_spend_in_wei
    }

    async fn send_proof_to_verify_on_chain(
        &self,
        blob: BlobTransactionSidecar,
        blob_versioned_hash: [u8; 32],
        aggregated_proof: AlignedProof,
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        let tx_req = match aggregated_proof {
            AlignedProof::SP1(proof) => self
                .proof_aggregation_service
                .verifySP1(
                    blob_versioned_hash.into(),
                    proof.proof_with_pub_values.public_values.to_vec().into(),
                    proof.proof_with_pub_values.bytes().into(),
                )
                .sidecar(blob)
                .into_transaction_request(),
            AlignedProof::Risc0(proof) => {
                let encoded_seal = encode_seal(&proof.receipt).map_err(|e| {
                    AggregatedProofSubmissionError::Risc0EncodingSeal(e.to_string())
                })?;
                self.proof_aggregation_service
                    .verifyRisc0(
                        blob_versioned_hash.into(),
                        encoded_seal.into(),
                        proof.receipt.journal.bytes.into(),
                    )
                    .sidecar(blob)
                    .into_transaction_request()
            }
        };

        let provider = self.proof_aggregation_service.provider();
        let envelope = provider
            .fill(tx_req)
            .await
            .map_err(Self::send_verify_aggregated_proof_err)?
            .try_into_envelope()
            .map_err(Self::send_verify_aggregated_proof_err)?;
        let tx: EthereumTxEnvelope<TxEip4844WithSidecar<BlobTransactionSidecarEip7594>> = envelope
            .try_into_pooled()
            .map_err(Self::send_verify_aggregated_proof_err)?
            .try_map_eip4844(|tx| {
                tx.try_map_sidecar(|sidecar| sidecar.try_into_7594(EnvKzgSettings::Default.get()))
            })
            .map_err(Self::send_verify_aggregated_proof_err)?;

        let encoded_tx = tx.encoded_2718();
        let pending_tx = provider
            .send_raw_transaction(&encoded_tx)
            .await
            .map_err(Self::send_verify_aggregated_proof_err)?;

        let receipt = pending_tx
            .get_receipt()
            .await
            .map_err(Self::send_verify_aggregated_proof_err)?;

        Ok(receipt)
    }

    fn send_verify_aggregated_proof_err<E: ToString>(err: E) -> AggregatedProofSubmissionError {
        AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
    }

    /// ### Blob capacity
    ///
    /// As dictated in [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844), each blob can hold:
    ///
    /// - `FIELD_ELEMENTS_PER_BLOB = 4096`
    /// - `BYTES_PER_FIELD_ELEMENT = 32`
    ///
    /// This gives a total theoretical capacity of:
    ///
    /// `FIELD_ELEMENTS_PER_BLOB * BYTES_PER_FIELD_ELEMENT = 4096 * 32 = 131072 bytes`
    ///
    /// However, this full capacity isn't usable due to the encoding of KZG commitments to elliptic curve points.
    /// Specifically:
    ///
    /// - Ethereum uses the BLS12-381 curve, whose scalar field modulus is slightly less than `2^256`
    ///   (closer to `2^255`).
    /// - Therefore, 32-byte field elements can't represent all 256-bit values.
    /// - To ensure values are within the field modulus, we **pad with a leading `0x00` byte**,
    ///   effectively capping values below the modulus.
    /// - This reduces the usable payload to **31 bytes per field element**.
    ///
    /// So, the _actual usable capacity_ per blob is:
    ///
    /// `4096 * 31 = 126976 bytes`
    ///
    /// Meaning that we can send as much as 126976 / 32 = 3968 proofs per blob
    async fn construct_blob(
        &self,
        leaves: Vec<[u8; 32]>,
    ) -> Result<(BlobTransactionSidecar, [u8; 32]), AggregatedProofSubmissionError> {
        let data: Vec<u8> = leaves.iter().flat_map(|arr| arr.iter().copied()).collect();
        let mut blob_data: [u8; BYTES_PER_BLOB] = [0u8; BYTES_PER_BLOB];

        // We pad the data with 0x0 byte every 31 bytes so that the field elements
        // constructed from the bytes are less than BLS_MODULUS.
        //
        // See https://github.com/ethereum/consensus-specs/blob/86fb82b221474cc89387fa6436806507b3849d88/specs/deneb/polynomial-commitments.md#bytes_to_bls_field
        let mut offset = 0;
        for chunk in data.chunks(31) {
            blob_data[offset] = 0x00;
            let start = offset + 1;
            let end = start + chunk.len();
            blob_data[start..end].copy_from_slice(chunk);
            offset += 32;
        }

        // calculate kzg commitments for blob

        // This parameter is the optimal balance between performance and memory usage to load the trusted setup
        // Source: https://github.com/ethereum/c-kzg-4844?tab=readme-ov-file#precompute
        let settings = c_kzg::ethereum_kzg_settings(8);
        let blob = c_kzg::Blob::new(blob_data);
        let commitment = settings
            .blob_to_kzg_commitment(&blob)
            .map_err(|_| AggregatedProofSubmissionError::BuildingBlobCommitment)?;
        let proof = settings
            .compute_blob_kzg_proof(&blob, &commitment.to_bytes())
            .map_err(|_| AggregatedProofSubmissionError::BuildingBlobProof)?;

        let blob = BlobTransactionSidecar::from_kzg(
            vec![blob],
            vec![commitment.to_bytes()],
            vec![proof.to_bytes()],
        );
        let blob_versioned_hash = blob
            .versioned_hash_for_blob(0)
            .ok_or(AggregatedProofSubmissionError::BuildingBlobVersionedHash)?
            .0;

        Ok((blob, blob_versioned_hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::config::Config;

    fn make_aggregator() -> ProofAggregator {
        // Set the AGGREGATOR env variable to "sp1" or "risc0" as it's needed by ProofAggregator::new
        std::env::set_var("AGGREGATOR", "sp1");

        let current_dir = env!("CARGO_MANIFEST_DIR");

        // These config values are taken from config-files/config-proof-aggregator.yaml
        let config = Config {
            eth_rpc_url: "http://localhost:8545".to_string(),
            eth_ws_url: "ws://localhost:8545".to_string(),
            max_proofs_in_queue: 1000,
            proof_aggregation_service_address: "0xcbEAF3BDe82155F56486Fb5a1072cb8baAf547cc"
                .to_string(),
            aligned_service_manager_address: "0x851356ae760d987E095750cCeb3bC6014560891C"
                .to_string(),
            // Use a path relative to the crate so tests work both locally and in CI
            last_aggregated_block_filepath: format!(
                "{current_dir}/../config-files/proof-aggregator.last_aggregated_block.json"
            ),
            ecdsa: config::ECDSAConfig {
                private_key_store_path: format!(
                    "{current_dir}/../config-files/anvil.proof-aggregator.ecdsa.key.json"
                ),
                private_key_store_password: "".to_string(),
            },
            proofs_per_chunk: 512,
            total_proofs_limit: 3968,
            monthly_budget_eth: 15.0,
        };

        ProofAggregator::new(config)
    }

    #[test]
    fn test_should_send_proof_to_verify_on_chain_updated_cases() {
        let aggregator = make_aggregator();

        let gas_price_20gwei: U256 = U256::from(20_000_000_000u64);

        // With 0 seconds elapsed and 1 ETH budget, we cannot send the proof
        assert!(!aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(0),
            1.0,
            gas_price_20gwei,
        ));

        // After 24 hours and 1 ETH monthly budget, we can send the proof
        assert!(aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(24 * 3600),
            1.0,
            gas_price_20gwei,
        ));

        // After 24 hours and a very low budget, we cannot send the proof
        assert!(!aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(24 * 3600),
            0.00325,
            U256::from(0_200_000_000u64),
        ));

        // After 27 hours with the same budget as before, we can send the proof
        assert!(aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(27 * 3600),
            0.00325,
            U256::from(0_200_000_000u64),
        ));

        // After 30 days but with a very low budget, we cannot send the proof
        assert!(!aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(30 * 24 * 3600),
            0.001,
            gas_price_20gwei,
        ));

        // After 15 days, a moderate budget and a high gas price, we can still send the proof
        assert!(aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(15 * 24 * 3600),
            10.0,
            U256::from(2_000_000_000_000u64),
        ));

        // After 30 days and a medium budget, we cannot send the proof
        assert!(!aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(30 * 24 * 3600),
            0.012,
            gas_price_20gwei,
        ));

        // After 2 days and a reasonable budget, we can send the proof
        assert!(aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(2 * 24 * 3600),
            5.0,
            gas_price_20gwei,
        ));

        // After 10 days and a medium budget with a very high gas price, we cannot send the proof
        assert!(!aggregator.should_send_proof_to_verify_on_chain(
            Duration::from_secs(10 * 24 * 3600),
            2.0,
            U256::from(100_000_000_000_000u64),
        ));
    }
}
