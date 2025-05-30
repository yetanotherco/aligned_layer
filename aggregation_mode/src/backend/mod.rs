pub mod config;
pub mod fetcher;
mod merkle_tree;
mod s3;
mod types;

use crate::aggregators::{AlignedProof, ProofAggregationError, ZKVMEngine};

use alloy::{
    consensus::BlobTransactionSidecar,
    eips::eip4844::BYTES_PER_BLOB,
    hex,
    network::EthereumWallet,
    primitives::Address,
    providers::{PendingTransactionError, ProviderBuilder},
    rpc::types::TransactionReceipt,
    signers::local::LocalSigner,
};
use config::Config;
use fetcher::{ProofsFetcher, ProofsFetcherError};
use merkle_tree::compute_proofs_merkle_root;
use risc0_ethereum_contracts::encode_seal;
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
        let rpc_provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
        let proof_aggregation_service = AlignedProofAggregationService::new(
            Address::from_str(&config.proof_aggregation_service_address)
                .expect("AlignedProofAggregationService address should be valid"),
            rpc_provider,
        );

        let engine =
            ZKVMEngine::from_env().expect("AGGREGATOR env variable to be set to one of sp1|risc0");
        let fetcher = ProofsFetcher::new(&config);

        Self {
            engine,
            proof_aggregation_service,
            fetcher,
            config,
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
        let receipt = self
            .send_proof_to_verify_on_chain(blob, blob_versioned_hash, aggregated_proof)
            .await?;
        info!(
            "Proof sent and verified, tx hash {:?}",
            receipt.transaction_hash
        );

        Ok(())
    }

    async fn send_proof_to_verify_on_chain(
        &self,
        blob: BlobTransactionSidecar,
        blob_versioned_hash: [u8; 32],
        aggregated_proof: AlignedProof,
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        let res = match aggregated_proof {
            AlignedProof::SP1(proof) => {
                self.proof_aggregation_service
                    .verifySP1(
                        blob_versioned_hash.into(),
                        proof.proof_with_pub_values.public_values.to_vec().into(),
                        proof.proof_with_pub_values.bytes().into(),
                    )
                    .sidecar(blob)
                    .send()
                    .await
            }
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
                    .send()
                    .await
            }
        }
        .map_err(AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction)?;

        res.get_receipt()
            .await
            .map_err(AggregatedProofSubmissionError::ReceiptError)
    }

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
        let settings = c_kzg::ethereum_kzg_settings();
        let blob = c_kzg::Blob::new(blob_data);
        let commitment = c_kzg::KzgCommitment::blob_to_kzg_commitment(&blob, settings)
            .map_err(|_| AggregatedProofSubmissionError::BuildingBlobCommitment)?;
        let proof =
            c_kzg::KzgProof::compute_blob_kzg_proof(&blob, &commitment.to_bytes(), settings)
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
