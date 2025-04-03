pub mod config;
pub mod fetcher;
mod merkle_tree;
mod s3;
mod types;

use crate::zk::{
    aggregator::{self, AggregatedProof, ProgramInput, ProofAggregationError},
    backends::sp1::SP1AggregationInput,
    Proof, ZKVMEngine,
};
use alloy::{
    consensus::{Blob, BlobTransactionSidecar},
    eips::eip4844::BYTES_PER_BLOB,
    hex,
    network::EthereumWallet,
    primitives::{Address, FixedBytes},
    providers::{PendingTransactionError, ProviderBuilder},
    rpc::types::TransactionReceipt,
    signers::local::LocalSigner,
};
use config::Config;
use fetcher::{ProofsFetcher, ProofsFetcherError};
use merkle_tree::compute_proofs_merkle_root;
use sp1_sdk::HashableKey;
use std::str::FromStr;
use tracing::{error, info, warn};
use types::{AlignedProofAggregationService, AlignedProofAggregationServiceContract};

#[derive(Debug)]
pub enum AggregatedProofSubmissionError {
    Aggregation(ProofAggregationError),
    BuildingBlobCommitment,
    BuildingBlobProof,
    BuildingBlobVersionedHash,
    SendVerifyAggregatedProofTransaction(alloy::contract::Error),
    ReceiptError(PendingTransactionError),
    FetchingProofs(ProofsFetcherError),
}

pub struct ProofAggregator {
    engine: ZKVMEngine,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    fetcher: ProofsFetcher,
}

impl ProofAggregator {
    pub fn new(config: &Config) -> Self {
        let rpc_url = config.eth_rpc_url.parse().expect("correct url");
        let signer = LocalSigner::decrypt_keystore(
            config.ecdsa.private_key_store_path.clone(),
            config.ecdsa.private_key_store_password.clone(),
        )
        .expect("Correct keystore signer");
        let wallet = EthereumWallet::from(signer);
        let rpc_provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
        let proof_aggregation_service: AlignedProofAggregationService::AlignedProofAggregationServiceInstance<(), alloy::providers::fillers::FillProvider<alloy::providers::fillers::JoinFill<alloy::providers::fillers::JoinFill<alloy::providers::Identity, alloy::providers::fillers::JoinFill<alloy::providers::fillers::GasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::BlobGasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::NonceFiller, alloy::providers::fillers::ChainIdFiller>>>>, alloy::providers::fillers::WalletFiller<EthereumWallet>>, alloy::providers::RootProvider>> = AlignedProofAggregationService::new(
            Address::from_str(&config.proof_aggregation_service_address)
                .expect("Address to be correct"),
            rpc_provider,
        );
        let fetcher = ProofsFetcher::new(config);

        Self {
            engine: ZKVMEngine::SP1,
            proof_aggregation_service,
            fetcher,
        }
    }

    pub async fn start(&mut self) {
        info!("Starting proof aggregator service",);

        info!("About to aggregate and submit proof to be verified on chain");
        let res = self.aggregate_and_submit_proofs_on_chain().await;

        match res {
            Ok(()) => {
                info!("Process finished successfully");
            }
            Err(err) => {
                error!("Error while aggregating and submitting proofs: {:?}", err);
                info!("About to set aggregated proof as missed");
                if let Err(err) = self.set_aggregated_proof_as_missed().await {
                    error!("Error while marking proof as failed: {:?}", err);
                };
                info!("Proofs set as missed");
            }
        }
    }

    async fn aggregate_and_submit_proofs_on_chain(
        &mut self,
    ) -> Result<(), AggregatedProofSubmissionError> {
        let proofs = self
            .fetcher
            .fetch()
            .await
            .map_err(AggregatedProofSubmissionError::FetchingProofs)?;

        if proofs.len() == 0 {
            warn!("No proofs collected, skipping aggregation...");
            return Ok(());
        }

        info!("Proofs fetched, constructing merkle root...");
        let (merkle_root, leaves) = compute_proofs_merkle_root(&proofs);
        info!("Merkle root constructed: {}", hex::encode(merkle_root));

        info!("Starting proof aggregation program...");
        let output = match self.engine {
            ZKVMEngine::SP1 => {
                // only SP1 compressed proofs are supported
                let proofs = proofs
                    .into_iter()
                    .filter_map(|proof| match proof {
                        Proof::SP1(proof) => Some(proof),
                    })
                    .collect();

                let input = SP1AggregationInput {
                    proofs,
                    merkle_root,
                };

                aggregator::aggregate_proofs(ProgramInput::SP1(input))
                    .map_err(AggregatedProofSubmissionError::Aggregation)?
            }
        };
        info!("Proof aggregation program finished");

        info!("Constructing blob...");
        let (blob, blob_versioned_hash) = self.construct_blob(leaves).await?;
        info!(
            "Blob constructed, versioned hash: {}",
            hex::encode(blob_versioned_hash)
        );

        info!("Sending proof to ProofAggregationService contract...");
        let receipt = self
            .send_proof_to_verify_on_chain(blob, blob_versioned_hash, output.proof)
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
        aggregated_proof: AggregatedProof,
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        match aggregated_proof {
            AggregatedProof::SP1(proof) => {
                let res = self
                    .proof_aggregation_service
                    .verify(
                        blob_versioned_hash.into(),
                        proof.vk().bytes32_raw().into(),
                        proof.proof.public_values.to_vec().into(),
                        proof.proof.bytes().into(),
                    )
                    .sidecar(blob)
                    .send()
                    .await
                    .map_err(
                        AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction,
                    )?;

                res.get_receipt()
                    .await
                    .map_err(AggregatedProofSubmissionError::ReceiptError)
            }
        }
    }

    async fn construct_blob(
        &self,
        leaves: Vec<[u8; 32]>,
    ) -> Result<(BlobTransactionSidecar, [u8; 32]), AggregatedProofSubmissionError> {
        let data: Vec<u8> = leaves.iter().flat_map(|arr| arr.iter().copied()).collect();
        let mut blob_data: [u8; BYTES_PER_BLOB] = [0u8; BYTES_PER_BLOB];

        for (i, byte) in data.iter().enumerate() {
            blob_data[i] = *byte;
        }

        // calculate kzg commitments for blob
        let settings = c_kzg::ethereum_kzg_settings();
        let blob = c_kzg::Blob::new(blob_data);
        let commitment = c_kzg::KzgCommitment::blob_to_kzg_commitment(&blob, settings)
            .map_err(|_| AggregatedProofSubmissionError::BuildingBlobCommitment)?;
        let proof =
            c_kzg::KzgProof::compute_blob_kzg_proof(&blob, &commitment.to_bytes(), settings)
                .map_err(|_| AggregatedProofSubmissionError::BuildingBlobProof)?;

        // convert to alloy types
        let blob = Blob::from_slice(&blob_data);
        let commitment: FixedBytes<48> = FixedBytes::from_slice(commitment.to_bytes().as_slice());
        let proof: FixedBytes<48> = FixedBytes::from_slice(proof.to_bytes().as_slice());

        let blob = BlobTransactionSidecar::new(vec![blob], vec![commitment], vec![proof]);
        let blob_versioned_hash = blob
            .versioned_hash_for_blob(0)
            .ok_or(AggregatedProofSubmissionError::BuildingBlobVersionedHash)?
            .0;

        Ok((blob, blob_versioned_hash))
    }

    async fn set_aggregated_proof_as_missed(
        &self,
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        let res = self
            .proof_aggregation_service
            .markCurrentAggregatedProofAsMissed()
            .send()
            .await
            .map_err(AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction)?;

        res.get_receipt()
            .await
            .map_err(AggregatedProofSubmissionError::ReceiptError)
    }
}
