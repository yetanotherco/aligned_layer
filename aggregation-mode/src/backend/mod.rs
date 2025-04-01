pub mod config;
pub mod fetcher;
mod merkle_tree;
pub mod queue;
mod s3;
mod types;

use crate::zk::{
    aggregator::{self, AggregatedProof, ProgramInput, ProofAggregationError},
    backends::sp1::SP1AggregationInput,
    Proof, ZKVMEngine,
};
use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{PendingTransactionError, ProviderBuilder},
    rpc::types::TransactionReceipt,
    signers::local::LocalSigner,
};
use config::Config;
use fetcher::ProofsFetcher;
use merkle_tree::compute_proofs_merkle_root;
use queue::ProofsQueue;
use sp1_sdk::HashableKey;
use std::{str::FromStr, time::Duration};
use tracing::{error, info, warn};
use types::{AlignedProofAggregationService, AlignedProofAggregationServiceContract};

#[derive(Debug)]
enum AggregatedProofSubmissionError {
    Aggregation(ProofAggregationError),
    SendBlobTransaction,
    SendVerifyAggregatedProofTransaction(alloy::contract::Error),
    ReceiptError(PendingTransactionError),
}

pub struct ProofAggregator {
    engine: ZKVMEngine,
    submit_proof_every_secs: u64,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    fetcher: ProofsFetcher,
    queue: ProofsQueue,
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
        let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
        let proof_aggregation_service: AlignedProofAggregationService::AlignedProofAggregationServiceInstance<(), alloy::providers::fillers::FillProvider<alloy::providers::fillers::JoinFill<alloy::providers::fillers::JoinFill<alloy::providers::Identity, alloy::providers::fillers::JoinFill<alloy::providers::fillers::GasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::BlobGasFiller, alloy::providers::fillers::JoinFill<alloy::providers::fillers::NonceFiller, alloy::providers::fillers::ChainIdFiller>>>>, alloy::providers::fillers::WalletFiller<EthereumWallet>>, alloy::providers::RootProvider>> = AlignedProofAggregationService::new(
            Address::from_str(&config.proof_aggregation_service_address)
                .expect("Address to be correct"),
            provider,
        );
        let fetcher = ProofsFetcher::new(config);

        Self {
            engine: ZKVMEngine::SP1,
            submit_proof_every_secs: config.submit_proofs_every_secs,
            proof_aggregation_service,
            queue: ProofsQueue::new(config.max_proofs_in_queue),
            fetcher,
        }
    }

    pub async fn start(&mut self) {
        info!(
            "Starting proof aggregator service, configured to run every {}",
            self.submit_proof_every_secs
        );

        loop {
            tokio::time::sleep(Duration::from_secs(self.submit_proof_every_secs)).await;
            info!("About to aggregate and submit proof to be verified on chain");
            let res = self.aggregate_and_submit_proofs_on_chain().await;

            match res {
                Ok(()) => {
                    info!(
                        "Finished iteration, next aggregated proof is in {} seconds",
                        self.submit_proof_every_secs
                    );
                }
                Err(err) => {
                    error!("Error while aggregating and submitting proofs: {:?}", err);
                    if let Err(err) = self.set_aggregated_proof_as_missed().await {
                        error!("Error while marking proof as failed: {:?}", err);
                    };
                }
            };
        }
    }

    async fn aggregate_and_submit_proofs_on_chain(
        &mut self,
    ) -> Result<(), AggregatedProofSubmissionError> {
        self.fetcher.fetch(&mut self.queue).await;
        let proofs = self.queue.clear();

        if proofs.len() == 0 {
            warn!("No proofs in queue, skipping iteration...");
            return Ok(());
        }

        let (merkle_root, leaves) = compute_proofs_merkle_root(&proofs);
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

        let blob_tx_hash = self.send_blob_transaction(leaves).await?;
        self.send_proof_to_verify_on_chain(&blob_tx_hash, output.proof)
            .await?;

        Ok(())
    }

    async fn send_proof_to_verify_on_chain(
        &self,
        blob_tx_hash: &[u8; 32],
        aggregated_proof: AggregatedProof,
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        match aggregated_proof {
            AggregatedProof::SP1(proof) => {
                let res = self
                    .proof_aggregation_service
                    .verify(
                        blob_tx_hash.into(),
                        proof.vk().bytes32_raw().into(),
                        proof.proof.public_values.to_vec().into(),
                        proof.proof.bytes().into(),
                    )
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

    // TODO
    async fn send_blob_transaction(
        &self,
        leaves: Vec<[u8; 32]>,
    ) -> Result<[u8; 32], AggregatedProofSubmissionError> {
        Ok([0u8; 32])
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
