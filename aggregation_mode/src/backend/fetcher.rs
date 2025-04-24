use std::str::FromStr;

use super::{
    config::Config,
    types::{AlignedLayerServiceManager, AlignedLayerServiceManagerContract, RPCProvider},
};
use crate::{
    aggregators::{
        risc0_aggregator::Risc0ProofReceiptAndImageId, sp1_aggregator::SP1ProofWithPubValuesAndElf,
        AlignedProof, ZKVMEngine,
    },
    backend::s3::get_aligned_batch_from_s3,
};
use aligned_sdk::core::types::ProvingSystemId;
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use risc0_zkvm::Receipt;
use tracing::{error, info};

#[derive(Debug)]
pub enum ProofsFetcherError {
    GetLogs(String),
    GetBlockNumber(String),
}

pub struct ProofsFetcher {
    rpc_provider: RPCProvider,
    aligned_service_manager: AlignedLayerServiceManagerContract,
    last_aggregated_block: u64,
}

impl ProofsFetcher {
    pub fn new(config: &Config) -> Self {
        let rpc_url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
        let rpc_provider = ProviderBuilder::new().on_http(rpc_url);
        let aligned_service_manager = AlignedLayerServiceManager::new(
            Address::from_str(&config.aligned_service_manager_address)
                .expect("AlignedProofAggregationService address should be valid"),
            rpc_provider.clone(),
        );

        let last_aggregated_block = config.get_last_aggregated_block().unwrap();

        Self {
            rpc_provider,
            aligned_service_manager,
            last_aggregated_block,
        }
    }

    pub async fn fetch(
        &mut self,
        engine: ZKVMEngine,
    ) -> Result<Vec<AlignedProof>, ProofsFetcherError> {
        // Get current block
        let current_block = self
            .rpc_provider
            .get_block_number()
            .await
            .map_err(|e| ProofsFetcherError::GetBlockNumber(e.to_string()))?;

        if current_block < self.last_aggregated_block {
            return Err(ProofsFetcherError::GetBlockNumber(
                "Invalid last processed block".to_string(),
            ));
        }

        info!(
            "Fetching proofs from batch logs starting from block number {} upto {}",
            self.last_aggregated_block, current_block
        );

        // Subscribe to NewBatch event from AlignedServiceManager
        let logs = self
            .aligned_service_manager
            .NewBatchV3_filter()
            .from_block(self.last_aggregated_block)
            .to_block(current_block)
            .query()
            .await
            .map_err(|e| ProofsFetcherError::GetLogs(e.to_string()))?;

        info!("Logs collected {}", logs.len());

        // Update last processed block after collecting logs
        self.last_aggregated_block = current_block;

        let mut proofs = vec![];

        for (batch, _) in logs {
            info!(
                "New batch submitted, about to process. Batch merkle root {}...",
                batch.batchMerkleRoot
            );

            // Download batch proofs from s3
            let data = match get_aligned_batch_from_s3(batch.batchDataPointer).await {
                Ok(data) => data,
                Err(err) => {
                    error!("Error while downloading proofs from s3. Err {:?}", err);
                    continue;
                }
            };

            info!("Data downloaded from S3, number of proofs {}", data.len());

            // Filter compatible proofs to be aggregated and push to queue
            let proofs_to_add: Vec<AlignedProof> = match engine {
                ZKVMEngine::SP1 => data
                    .into_iter()
                    .filter_map(|p| match p.proving_system {
                        ProvingSystemId::SP1 => {
                            let elf = p.vm_program_code?;
                            let proof_with_pub_values = bincode::deserialize(&p.proof).ok()?;
                            let sp1_proof = SP1ProofWithPubValuesAndElf {
                                proof_with_pub_values,
                                elf,
                            };

                            Some(AlignedProof::SP1(sp1_proof.into()))
                        }

                        _ => None,
                    })
                    .collect(),
                ZKVMEngine::RISC0 => data
                    .into_iter()
                    .filter_map(|p| match p.proving_system {
                        ProvingSystemId::Risc0 => {
                            let mut image_id = [0u8; 32];
                            image_id.copy_from_slice(p.vm_program_code?.as_slice());
                            let public_inputs = p.pub_input?;
                            let inner_receipt: risc0_zkvm::InnerReceipt =
                                bincode::deserialize(&p.proof).ok()?;

                            let receipt = Receipt::new(inner_receipt, public_inputs);
                            let risc0_proof = Risc0ProofReceiptAndImageId { image_id, receipt };

                            Some(AlignedProof::Risc0(risc0_proof.into()))
                        }
                        _ => None,
                    })
                    .collect(),
            };

            info!(
                "{} Proofs filtered, compatible proofs found {}",
                engine,
                proofs_to_add.len()
            );

            // try to add them to the queue
            for proof in proofs_to_add {
                if let Err(err) = proof.verify() {
                    error!("Could not add proof, verification failed: {:?}", err);
                    continue;
                };

                proofs.push(proof);
            }
        }

        Ok(proofs)
    }

    pub fn get_last_aggregated_block(&self) -> u64 {
        self.last_aggregated_block
    }
}
