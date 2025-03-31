use std::{str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{PendingTransactionError, ProviderBuilder},
    rpc::types::TransactionReceipt,
    signers::local::PrivateKeySigner,
};
use merkle_tree::compute_proofs_merkle_root;
use sp1_sdk::HashableKey;
use tracing::{error, info};
use types::{AlignedProofAggregationService, AlignedProofAggregationServiceContract};

use crate::zk::{
    aggregator::{self, AggregatedProof, ProgramInput, ProofAggregationError},
    backends::sp1::SP1AggregationInput,
    Proof, VerificationError, ZKVMEngine,
};

mod merkle_tree;
mod types;

#[derive(Debug)]
pub enum ProofQueueError {
    QueueMaxCapacity,
    InvalidProof(VerificationError),
}

#[derive(Debug)]
enum AggregatedProofSubmissionError {
    Aggregation(ProofAggregationError),
    SendBlobTransaction,
    SendVerifyAggregatedProofTransaction(alloy::contract::Error),
    GettingReceiptVerifyAggregatedProofTransaction(PendingTransactionError),
}

pub struct ProofAggregator {
    engine: ZKVMEngine,
    submit_proof_every_secs: u64,
    max_proofs_in_queue: u16,
    proofs_queue: Vec<Proof>,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
}

pub struct Config {
    pub rpc_url: String,
    pub private_key: String,
}

impl ProofAggregator {
    pub fn new(config: Config) -> Self {
        let rpc_url = config.rpc_url.parse().expect("correct url");
        let signer = PrivateKeySigner::from_str(&config.private_key).expect("valid string");
        let wallet = EthereumWallet::from(signer);
        let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
        let proof_aggregation_service =
            AlignedProofAggregationService::new(Address::default(), provider);

        Self {
            engine: ZKVMEngine::SP1,
            submit_proof_every_secs: 10,
            max_proofs_in_queue: 2,
            proofs_queue: vec![],
            proof_aggregation_service,
        }
    }

    pub async fn start(&mut self) {
        info!("Starting proof aggregator service");
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
                    self.set_aggregated_proof_as_missed().await;
                }
            };
        }
    }

    pub fn add_proof(&mut self, proof: Proof, elf: &[u8]) -> Result<(), ProofQueueError> {
        if let Err(err) = proof.verify(elf) {
            return Err(ProofQueueError::InvalidProof(err));
        };

        if self.proofs_queue.len() as u16 >= self.max_proofs_in_queue {
            return Err(ProofQueueError::QueueMaxCapacity);
        }

        self.proofs_queue.push(proof);

        Ok(())
    }

    async fn aggregate_and_submit_proofs_on_chain(
        &mut self,
    ) -> Result<(), AggregatedProofSubmissionError> {
        let proofs = self
            .proofs_queue
            .drain(0..self.proofs_queue.len())
            .collect::<Vec<_>>();

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
                        proof.vk.bytes32_raw().into(),
                        proof.proof.public_values.to_vec().into(),
                        proof.proof.bytes().into(),
                    )
                    .send()
                    // sign with aggregator wallet
                    .await
                    .map_err(
                        AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction,
                    )?;

                res.get_receipt().await.map_err(
                    AggregatedProofSubmissionError::GettingReceiptVerifyAggregatedProofTransaction,
                )
            }
        }
    }

    async fn send_blob_transaction(
        &self,
        leaves: Vec<[u8; 32]>,
    ) -> Result<[u8; 32], AggregatedProofSubmissionError> {
        Ok([0u8; 32])
    }

    async fn set_aggregated_proof_as_missed(&self) {}
}
