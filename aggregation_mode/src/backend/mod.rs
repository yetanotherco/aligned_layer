pub mod config;
pub mod fetcher;
mod merkle_tree;
mod retry;
mod s3;
mod types;

use crate::aggregators::{AlignedProof, ProofAggregationError, ZKVMEngine};

use alloy::{
    eips::eip4844::BYTES_PER_BLOB,
    hex,
    network::EthereumWallet,
    primitives::Address,
    providers::{PendingTransactionError, ProviderBuilder},
    signers::local::LocalSigner,
};
use config::Config;
use ethrex_common::{
    types::{BlobsBundle, Fork},
    H256,
};
use ethrex_l2_rpc::signer::LocalSigner as EthrexLocalSigner;
use ethrex_rpc::{clients::Overrides, EthClient};
use ethrex_sdk::{build_generic_tx, calldata::encode_calldata, send_generic_transaction};
use fetcher::{ProofsFetcher, ProofsFetcherError};
use merkle_tree::compute_proofs_merkle_root;
use risc0_ethereum_contracts::encode_seal;
use secp256k1::SecretKey;
use std::str::FromStr;
use tracing::{error, info, warn};
use types::{AlignedProofAggregationService, AlignedProofAggregationServiceContract};

#[derive(Debug)]
pub enum AggregatedProofSubmissionError {
    BuildingBlobCommitment,
    BuildingBlobProof,
    BuildingBlobVersionedHash,
    Risc0EncodingSeal(String),
    SendVerifyAggregatedProofTransaction(alloy::contract::Error),
    ReceiptError(PendingTransactionError),
    FetchingProofs(ProofsFetcherError),
    ZKVMAggregation(ProofAggregationError),
    BuildingMerkleRoot,
    MerkleRootMisMatch,
}

pub struct ProofAggregator {
    engine: ZKVMEngine,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    fetcher: ProofsFetcher,
    config: Config,
    ethrex_eth_client: EthClient,
    ethrex_signer: ethrex_l2_rpc::signer::Signer,
}

impl ProofAggregator {
    pub fn new(config: Config) -> Self {
        let rpc_url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
        let signer = LocalSigner::decrypt_keystore(
            config.ecdsa.private_key_store_path.clone(),
            config.ecdsa.private_key_store_password.clone(),
        )
        .expect("Keystore signer should be `cast wallet` compliant");
        let wallet = EthereumWallet::from(signer);
        let rpc_provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);
        let proof_aggregation_service = AlignedProofAggregationService::new(
            Address::from_str(&config.proof_aggregation_service_address)
                .expect("AlignedProofAggregationService address should be valid"),
            rpc_provider,
        );

        let engine =
            ZKVMEngine::from_env().expect("AGGREGATOR env variable to be set to one of sp1|risc0");
        let fetcher = ProofsFetcher::new(&config);
        let ethrex_eth_client = ethrex_rpc::EthClient::new(&config.eth_rpc_url).unwrap();
        let secret_key = SecretKey::from_str(&config.ecdsa.private_key).unwrap();
        let ethrex_signer =
            ethrex_l2_rpc::signer::Signer::Local(EthrexLocalSigner::new(secret_key));

        Self {
            engine,
            proof_aggregation_service,
            fetcher,
            config,
            ethrex_eth_client,
            ethrex_signer,
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

        info!("Sending proof to ProofAggregationService contract...");
        let tx_hash = self
            .send_proof_to_verify_on_chain(blob, blob_versioned_hash, aggregated_proof)
            .await?;
        info!("Proof sent and verified, tx hash {:?}", tx_hash);

        Ok(())
    }

    async fn send_proof_to_verify_on_chain(
        &mut self,
        blob_bundle: BlobsBundle,
        blob_versioned_hash: [u8; 32],
        aggregated_proof: AlignedProof,
    ) -> Result<H256, AggregatedProofSubmissionError> {
        // TODO: see how to get this
        self.ethrex_eth_client.maximum_allowed_max_fee_per_blob_gas = Some(1);

        match aggregated_proof {
            AlignedProof::SP1(proof) => {
                let calldata = encode_calldata(
                    "verifySP1(bytes32,bytes,bytes)",
                    &[
                        ethrex_l2_common::calldata::Value::FixedBytes(
                            blob_versioned_hash.to_vec().into(),
                        ),
                        ethrex_l2_common::calldata::Value::Bytes(
                            proof.proof_with_pub_values.public_values.to_vec().into(),
                        ),
                        ethrex_l2_common::calldata::Value::Bytes(
                            proof.proof_with_pub_values.bytes().into(),
                        ),
                    ],
                )
                .expect("Calldata to be valid");

                let tx = build_generic_tx(
                    &self.ethrex_eth_client,
                    ethrex_common::types::TxType::EIP4844,
                    self.proof_aggregation_service.address().0 .0.into(),
                    self.ethrex_signer.address(),
                    calldata.into(),
                    Overrides {
                        blobs_bundle: Some(blob_bundle),
                        ..Default::default()
                    },
                )
                .await
                .expect("Tx to be built correctly");

                let tx_hash =
                    send_generic_transaction(&self.ethrex_eth_client, tx, &self.ethrex_signer)
                        .await
                        .expect("Transaction to be sent");

                Ok(tx_hash)
            }
            AlignedProof::Risc0(proof) => {
                let encoded_seal = encode_seal(&proof.receipt).map_err(|e| {
                    AggregatedProofSubmissionError::Risc0EncodingSeal(e.to_string())
                })?;

                let calldata = encode_calldata(
                    "verifyRisc0(bytes32,bytes,bytes)",
                    &[
                        ethrex_l2_common::calldata::Value::FixedBytes(
                            blob_versioned_hash.to_vec().into(),
                        ),
                        ethrex_l2_common::calldata::Value::Bytes(encoded_seal.into()),
                        ethrex_l2_common::calldata::Value::Bytes(
                            proof.receipt.journal.bytes.into(),
                        ),
                    ],
                )
                .expect("Calldata to be valid");

                let tx = build_generic_tx(
                    &self.ethrex_eth_client,
                    ethrex_common::types::TxType::EIP4844,
                    self.proof_aggregation_service.address().0 .0.into(),
                    self.ethrex_signer.address(),
                    calldata.into(),
                    Overrides {
                        blobs_bundle: Some(blob_bundle),
                        ..Default::default()
                    },
                )
                .await
                .expect("Tx to be built correctly");

                let tx_hash =
                    send_generic_transaction(&self.ethrex_eth_client, tx, &self.ethrex_signer)
                        .await
                        .expect("Transaction to be sent");

                Ok(tx_hash)
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
    ) -> Result<(BlobsBundle, [u8; 32]), AggregatedProofSubmissionError> {
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
        let blobs_bundle = BlobsBundle::create_from_blobs(&vec![blob_data], Fork::Osaka).unwrap();
        let blob_versioned_hash = blobs_bundle.generate_versioned_hashes()[0];

        Ok((blobs_bundle, blob_versioned_hash.0))
    }
}
