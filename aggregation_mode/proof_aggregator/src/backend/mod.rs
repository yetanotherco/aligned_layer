pub mod config;
mod db;
pub mod fetcher;
mod merkle_tree;
mod types;

use crate::{
    aggregators::{AlignedProof, ProofAggregationError, ZKVMEngine},
    backend::db::{Db, DbError},
};

use alloy::{
    consensus::{BlobTransactionSidecar, EnvKzgSettings, EthereumTxEnvelope, TxEip4844WithSidecar},
    eips::{eip4844::BYTES_PER_BLOB, eip7594::BlobTransactionSidecarEip7594, Encodable2718},
    hex,
    network::EthereumWallet,
    primitives::{utils::parse_ether, Address, U256},
    providers::{PendingTransactionError, Provider, ProviderBuilder},
    rpc::types::TransactionReceipt,
    signers::local::LocalSigner,
};
use config::Config;
use fetcher::{ProofsFetcher, ProofsFetcherError};
use merkle_tree::compute_proofs_merkle_root;
use risc0_ethereum_contracts::encode_seal;
use sqlx::types::Uuid;
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
    StoringMerklePaths(DbError),
    GasPriceError(String),
}

pub struct ProofAggregator {
    engine: ZKVMEngine,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    fetcher: ProofsFetcher,
    config: Config,
    rpc_provider: RPCProvider,
    sp1_chunk_aggregator_vk_hash_bytes: [u8; 32],
    risc0_chunk_aggregator_image_id_bytes: [u8; 32],
    db: Db,
}

impl ProofAggregator {
    pub async fn new(config: Config) -> Self {
        let rpc_url: reqwest::Url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
        let signer = LocalSigner::decrypt_keystore(
            config.ecdsa.private_key_store_path.clone(),
            config.ecdsa.private_key_store_password.clone(),
        )
        .expect("Keystore signer should be `cast wallet` compliant");
        let wallet = EthereumWallet::from(signer);

        // Check if the monthly budget is non-negative to avoid runtime errors later
        let _monthly_budget_in_wei = parse_ether(&config.monthly_budget_eth.to_string())
            .expect("Monthly budget must be a non-negative value");

        info!("Monthly budget set to {} eth", config.monthly_budget_eth);

        let rpc_provider = ProviderBuilder::new().connect_http(rpc_url.clone());

        let signed_rpc_provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

        let proof_aggregation_service = AlignedProofAggregationService::new(
            Address::from_str(&config.proof_aggregation_service_address)
                .expect("AlignedProofAggregationService address should be valid"),
            signed_rpc_provider.clone(),
        );

        let engine =
            ZKVMEngine::from_env().expect("AGGREGATOR env variable to be set to one of sp1|risc0");

        let db = Db::try_new(&config.db_connection_url)
            .await
            .expect("To connect to db");

        let fetcher = ProofsFetcher::new(db.clone());

        let sp1_chunk_aggregator_vk_hash_bytes: [u8; 32] =
            hex::decode(&config.sp1_chunk_aggregator_vk_hash)
                .expect("Failed to decode SP1 chunk aggregator VK hash")
                .try_into()
                .expect("SP1 chunk aggregator VK hash must be 32 bytes");

        let risc0_chunk_aggregator_image_id_bytes: [u8; 32] =
            hex::decode(&config.risc0_chunk_aggregator_image_id)
                .expect("Failed to decode Risc0 chunk aggregator image id")
                .try_into()
                .expect("Risc0 chunk aggregator image id must be 32 bytes");

        Self {
            engine,
            proof_aggregation_service,
            fetcher,
            config,
            rpc_provider,
            sp1_chunk_aggregator_vk_hash_bytes,
            risc0_chunk_aggregator_image_id_bytes,
            db,
        }
    }

    pub async fn start(&mut self) {
        info!("Starting proof aggregator service");

        info!("About to aggregate and submit proof to be verified on chain");
        let res = self.aggregate_and_submit_proofs_on_chain().await;

        match res {
            Ok(()) => {
                info!("Process finished successfully");
            }
            Err(err) => {
                error!("Error while aggregating and submitting proofs: {:?}", err);
            }
        }
    }

    // TODO: on failure, mark proofs as pending again
    async fn aggregate_and_submit_proofs_on_chain(
        &mut self,
    ) -> Result<(), AggregatedProofSubmissionError> {
        let (proofs, tasks_id) = self
            .fetcher
            .fetch_pending_proofs(self.engine.clone(), self.config.total_proofs_limit as i64)
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

        // We start on 24 hours because the proof aggregator runs once a day, so the time elapsed
        // should be considered over a 24h period.
        let mut time_elapsed = Duration::from_secs(24 * 3600);

        // Iterate until we can send the proof on-chain
        loop {
            // Fetch gas price from network
            let gas_price = self
                .rpc_provider
                .get_gas_price()
                .await
                .map_err(|e| AggregatedProofSubmissionError::GasPriceError(e.to_string()))?;

            if Self::should_send_proof_to_verify_on_chain(
                time_elapsed,
                self.config.monthly_budget_eth,
                U256::from(gas_price),
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

        // Retry in case of failure
        let receipt = self
            .send_proof_to_verify_on_chain_retryable(blob, blob_versioned_hash, aggregated_proof)
            .await?;
        info!(
            "Proof sent and verified, tx hash {:?}",
            receipt.transaction_hash
        );

        info!("Storing merkle paths for each task...",);
        let mut merkle_paths_for_tasks: Vec<(Uuid, Vec<u8>)> = vec![];
        for (idx, task_id) in tasks_id.into_iter().enumerate() {
            let Some(proof) = merkle_tree.get_proof_by_pos(idx) else {
                warn!("Proof not found for task id {task_id}");
                continue;
            };
            let proof_bytes = proof
                .merkle_path
                .iter()
                .flat_map(|e| e.to_vec())
                .collect::<Vec<_>>();

            merkle_paths_for_tasks.push((task_id, proof_bytes))
        }
        self.db
            .insert_tasks_merkle_path_and_mark_them_as_verified(merkle_paths_for_tasks)
            .await
            .map_err(AggregatedProofSubmissionError::StoringMerklePaths)?;
        info!("Merkle path inserted sucessfully",);

        Ok(())
    }

    fn max_to_spend_in_wei(time_elapsed: Duration, monthly_eth_budget: f64) -> U256 {
        const SECONDS_PER_MONTH: u64 = 30 * 24 * 60 * 60;

        // Note: this expect is safe because in case it was invalid, should have been caught at startup
        let monthly_budget_in_wei = parse_ether(&monthly_eth_budget.to_string())
            .expect("The monthly budget should be a non-negative value");

        let elapsed_seconds = U256::from(time_elapsed.as_secs());

        let budget_available_per_second_in_wei =
            monthly_budget_in_wei / U256::from(SECONDS_PER_MONTH);

        budget_available_per_second_in_wei * elapsed_seconds
    }

    /// Decides whether to send the aggregated proof to be verified on-chain based on
    /// time elapsed since last submission and monthly ETH budget.
    /// We make a linear function with the eth to spend this month and the time elapsed since last submission.
    /// If eth to spend / elapsed time is over the linear function, we skip the submission.
    fn should_send_proof_to_verify_on_chain(
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

    async fn send_proof_to_verify_on_chain_retryable(
        &self,
        blob: BlobTransactionSidecar,
        blob_versioned_hash: [u8; 32],
        aggregated_proof: AlignedProof,
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        match send_proof_to_verify_on_chain(
            blob.clone(),
            blob_versioned_hash,
            aggregated_proof.clone(),
            self.proof_aggregation_service.clone(),
            self.sp1_chunk_aggregator_vk_hash_bytes,
            self.risc0_chunk_aggregator_image_id_bytes,
        )
        .await
        {
            Ok(tx_receipt) => Ok(tx_receipt),
            Err(err) => {
                tracing::error!("Failed to send proof to be verified on chain: {err:?}");

                retry_function(
                    || {
                        send_proof_to_verify_on_chain(
                            blob.clone(),
                            blob_versioned_hash,
                            aggregated_proof.clone(),
                            self.proof_aggregation_service.clone(),
                            self.sp1_chunk_aggregator_vk_hash_bytes,
                            self.risc0_chunk_aggregator_image_id_bytes,
                        )
                    },
                    ETHEREUM_CALL_MIN_RETRY_DELAY,
                    ETHEREUM_CALL_BACKOFF_FACTOR,
                    ETHEREUM_CALL_MAX_RETRIES,
                    ETHEREUM_CALL_MAX_RETRY_DELAY,
                )
                .await
                .map_err(|e| {
                    error!("Could't get nonce: {:?}", e);
                    e.inner()
                })
            }
        }
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

async fn send_proof_to_verify_on_chain(
    blob: BlobTransactionSidecar,
    blob_versioned_hash: [u8; 32],
    aggregated_proof: AlignedProof,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    sp1_chunk_aggregator_vk_hash_bytes: [u8; 32],
    risc0_chunk_aggregator_image_id_bytes: [u8; 32],
) -> Result<TransactionReceipt, RetryError<AggregatedProofSubmissionError>> {
    let tx_req = match aggregated_proof {
        AlignedProof::SP1(proof) => proof_aggregation_service
            .verifyAggregationSP1(
                blob_versioned_hash.into(),
                proof.proof_with_pub_values.public_values.to_vec().into(),
                proof.proof_with_pub_values.bytes().into(),
                sp1_chunk_aggregator_vk_hash_bytes.into(),
            )
            .sidecar(blob)
            .into_transaction_request(),
        AlignedProof::Risc0(proof) => {
            let encoded_seal = encode_seal(&proof.receipt)
                .map_err(|e| AggregatedProofSubmissionError::Risc0EncodingSeal(e.to_string()))
                .map_err(RetryError::Transient)?;
            proof_aggregation_service
                .verifyAggregationRisc0(
                    blob_versioned_hash.into(),
                    encoded_seal.into(),
                    proof.receipt.journal.bytes.into(),
                    risc0_chunk_aggregator_image_id_bytes.into(),
                )
                .sidecar(blob)
                .into_transaction_request()
        }
    };

    let provider = proof_aggregation_service.provider();
    let envelope = provider
        .fill(tx_req)
        .await
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?
        .try_into_envelope()
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;
    let tx: EthereumTxEnvelope<TxEip4844WithSidecar<BlobTransactionSidecarEip7594>> = envelope
        .try_into_pooled()
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?
        .try_map_eip4844(|tx| {
            tx.try_map_sidecar(|sidecar| sidecar.try_into_7594(EnvKzgSettings::Default.get()))
        })
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;

    let encoded_tx = tx.encoded_2718();
    let pending_tx = provider
        .send_raw_transaction(&encoded_tx)
        .await
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;

    let receipt = pending_tx
        .get_receipt()
        .await
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;

    Ok(receipt)
}

use backon::ExponentialBuilder;
use backon::Retryable;
use std::future::Future;

#[derive(Debug)]
pub enum RetryError<E> {
    Transient(E),
    Permanent(E),
}

impl<E: std::fmt::Display> std::fmt::Display for RetryError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RetryError::Transient(e) => write!(f, "{}", e),
            RetryError::Permanent(e) => write!(f, "{}", e),
        }
    }
}

impl<E> RetryError<E> {
    pub fn inner(self) -> E {
        match self {
            RetryError::Transient(e) => e,
            RetryError::Permanent(e) => e,
        }
    }
}

impl<E: std::fmt::Display> std::error::Error for RetryError<E> where E: std::fmt::Debug {}

pub const ETHEREUM_CALL_MIN_RETRY_DELAY: u64 = 500; // milliseconds
pub const ETHEREUM_CALL_MAX_RETRIES: usize = 5;
pub const ETHEREUM_CALL_BACKOFF_FACTOR: f32 = 2.0;
pub const ETHEREUM_CALL_MAX_RETRY_DELAY: u64 = 60; // seconds

/// Supports retries only on async functions. See: https://docs.rs/backon/latest/backon/#retry-an-async-function
/// Runs with `jitter: false`.
pub async fn retry_function<FutureFn, Fut, T, E>(
    function: FutureFn,
    min_delay: u64,
    factor: f32,
    max_times: usize,
    max_delay: u64,
) -> Result<T, RetryError<E>>
where
    Fut: Future<Output = Result<T, RetryError<E>>>,
    FutureFn: FnMut() -> Fut,
{
    let backoff = ExponentialBuilder::default()
        .with_min_delay(Duration::from_millis(min_delay))
        .with_max_times(max_times)
        .with_factor(factor)
        .with_max_delay(Duration::from_secs(max_delay));

    function
        .retry(backoff)
        .sleep(tokio::time::sleep)
        .when(|e| matches!(e, RetryError::Transient(_)))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_send_proof_to_verify_on_chain_updated_cases() {
        // The should_send_proof_to_verify_on_chain function returns true when:
        // gas_price * 600_000 <= (seconds_elapsed) * (monthly_eth_budget / (30 * 24 * 60 * 60))

        const BUDGET_PER_MONTH_IN_ETH: f64 = 0.15;
        const ONE_DAY_SECONDS: u64 = 24 * 60 * 60;
        let gas_price = U256::from(1_000_000_000u64); // 1 Gwei

        // Case 1: Base case -> should return true
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000058 ETH/hour * 24 hours = 0.005 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(ProofAggregator::should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            BUDGET_PER_MONTH_IN_ETH,              // 0.15 ETH monthly budget
            gas_price,                            // 1 Gwei gas price
        ));

        // Case 2: Slightly Increased Gas Price -> should return false
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 8 Gwei
        // Max to spend: 0.000000058 ETH/hour * 24 hours = 0.005 ETH
        // Expected cost: 600,000 * 8 Gwei = 0.0048 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(ProofAggregator::should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            BUDGET_PER_MONTH_IN_ETH,              // 0.15 ETH monthly budget
            U256::from(8_000_000_000u64),         // 8 Gwei gas price
        ));

        // Case 3: Increased Gas Price -> should return false
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 10 Gwei
        // Max to spend: 0.000000058 ETH/hour * 24 hours = 0.005 ETH
        // Expected cost: 600,000 * 10 Gwei = 0.006 ETH
        // Expected cost > Max to spend, so we cannot send the proof
        assert!(!ProofAggregator::should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            BUDGET_PER_MONTH_IN_ETH,              // 0.15 ETH monthly budget
            U256::from(10_000_000_000u64),        // 10 Gwei gas price
        ));

        // Case 4: Slightly Reduced Time Elapsed -> should return true
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 2 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000058 ETH/hour * 3 hours = 0.000625 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(ProofAggregator::should_send_proof_to_verify_on_chain(
            Duration::from_secs(3 * 3600), // 3 hours
            BUDGET_PER_MONTH_IN_ETH,       // 0.15 ETH monthly budget
            gas_price,                     // 1 Gwei gas price
        ));

        // Case 5: Reduced Time Elapsed -> should return false
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 1.2 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000058 ETH/hour * 1.2 hours = 0.00025 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost > Max to spend, so we cannot send the proof
        assert!(!ProofAggregator::should_send_proof_to_verify_on_chain(
            Duration::from_secs_f64(1.2 * 3600.0), // 1.2 hours
            BUDGET_PER_MONTH_IN_ETH,               // 0.15 ETH monthly budget
            gas_price,                             // 1 Gwei gas price
        ));

        // Case 6: Slightly Reduced Monthly Budget -> should return true
        // Monthly Budget: 0.1 ETH -> 0.0033 ETH per day -> 0.000000038 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000038 ETH/hour * 24 hours = 0.0032832 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(ProofAggregator::should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            0.1,                                  // 0.1 ETH monthly budget
            gas_price,                            // 1 Gwei gas price
        ));

        // Case 7: Decreased Monthly Budget -> should return false
        // Monthly Budget: 0.01 ETH -> 0.00033 ETH per day -> 0.0000000038 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.0000000038 ETH/hour * 24 hours = 0.00032832 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost > Max to spend, so we cannot send the proof
        assert!(!ProofAggregator::should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            0.01,                                 // 0.01 ETH monthly budget
            gas_price,                            // 1 Gwei gas price
        ));
    }
}
