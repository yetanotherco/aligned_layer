use std::time::Duration;

use alloy::{
    primitives::Address,
    providers::{PendingTransactionError, ProviderBuilder},
    rpc::types::TransactionReceipt,
};
use sp1_sdk::HashableKey;
use tracing::{error, info};
use types::{AlignedProofAggregationService, AlignedProofAggregationServiceContract};

use crate::zk::{
    aggregator::{self, AggregatedProof, ProgramInput, ProofAggregationError},
    Proof, ZKVMEngine,
};

mod types;

#[derive(Debug)]
pub enum ProofQueueError {
    QueueMaxCapacity,
    InvalidProof,
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

impl ProofAggregator {
    // TODO read .yaml config file
    pub fn new(rpc_url: &str) -> Self {
        let rpc_url = rpc_url.parse().expect("correct url");
        let provider = ProviderBuilder::new().on_http(rpc_url);
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

    pub fn add_proof(&mut self, proof: Proof) -> Result<(), ProofQueueError> {
        if proof.verify().is_err() {
            return Err(ProofQueueError::InvalidProof);
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
        // TODO build merkle tree and pass as input

        let proofs = self
            .proofs_queue
            .drain(0..self.proofs_queue.len())
            .collect::<Vec<_>>();

        let leaves: Vec<[u8; 32]> = vec![];
        let merkle_root = [0u8; 32];

        let output = match self.engine {
            ZKVMEngine::SP1 => {
                // convert proofs to sp1 input format
                let proofs = proofs
                    .iter()
                    .map(|proof| match proof {
                        Proof::SP1(proof) => sp1_aggregator::Proof::SP1Compressed(
                            sp1_aggregator::SP1CompressedProof {
                                public_inputs: proof.proof.public_values.to_vec(),
                                vk: proof.verifying_key().bytes32().as_bytes().to_vec(),
                            },
                        ),
                    })
                    .collect();
                let input = sp1_aggregator::Input { proofs };

                // clean proof queue
                self.proofs_queue = vec![];

                aggregator::aggregate_proofs(ProgramInput::SP1(input))
                    .map_err(AggregatedProofSubmissionError::Aggregation)?
            }
        };

        let receipt = self.send_blob_transaction(leaves).await?;
        self.send_proof_to_verify_on_chain(&receipt.transaction_hash.0, output.proof)
            .await?;

        Ok(())
    }

    // TODO send blob + contract transaction
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
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        Err(AggregatedProofSubmissionError::SendBlobTransaction)
    }

    async fn set_aggregated_proof_as_missed(&self) {}
}
