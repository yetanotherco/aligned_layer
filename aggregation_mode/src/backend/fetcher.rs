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
use aligned_sdk::common::types::ProvingSystemId;
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use rayon::prelude::*;
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

    /// Retrieves batches from the aligned fast mode since the last processed block,
    /// filtering for proofs compatible with the specified zkVM engine.
    pub async fn fetch(
        &mut self,
        engine: ZKVMEngine,
        limit: u16,
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

        let mut proofs = vec![];

        for (batch, log) in logs {
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
                    .into_par_iter()
                    .filter_map(|p| {
                        if p.proving_system != ProvingSystemId::SP1 {
                            return None;
                        };

                        let elf = p.vm_program_code?;
                        let proof_with_pub_values = bincode::deserialize(&p.proof).ok()?;
                        let sp1_proof =
                            SP1ProofWithPubValuesAndElf::new(proof_with_pub_values, elf);

                        match sp1_proof {
                            Ok(proof) => Some(AlignedProof::SP1(proof.into())),
                            Err(err) => {
                                error!("Could not add proof, verification failed: {:?}", err);
                                None
                            }
                        }
                    })
                    .collect(),
                ZKVMEngine::RISC0 => data
                    .into_par_iter()
                    .filter_map(|p| {
                        if p.proving_system != ProvingSystemId::Risc0 {
                            return None;
                        };

                        let mut image_id = [0u8; 32];
                        image_id.copy_from_slice(p.vm_program_code?.as_slice());
                        let public_inputs = p.pub_input?;
                        let inner_receipt: risc0_zkvm::InnerReceipt =
                            bincode::deserialize(&p.proof).ok()?;

                        let receipt = Receipt::new(inner_receipt, public_inputs);
                        let risc0_proof = Risc0ProofReceiptAndImageId::new(image_id, receipt);

                        match risc0_proof {
                            Ok(proof) => Some(AlignedProof::Risc0(proof.into())),
                            Err(err) => {
                                error!("Could not add proof, verification failed: {:?}", err);
                                None
                            }
                        }
                    })
                    .collect(),
            };

            info!(
                "{} Proofs filtered, compatible proofs found {}",
                engine,
                proofs_to_add.len()
            );

            if (proofs.len() + proofs_to_add.len()) > (limit as usize) {
                let log_block_number = log.block_number.unwrap();
                info!(
                    "Limit of {} proofs reached, stopping at block number {}, which is {} from current block",
                    limit, log_block_number, current_block - log_block_number
                );
                // Update last processed block to this log block number
                // So the next aggregation starts at this block
                self.last_aggregated_block = log_block_number;
                return Ok(proofs);
            }

            proofs.extend(proofs_to_add);
        }

        // Update last processed block after collecting logs
        self.last_aggregated_block = current_block;

        Ok(proofs)
    }

    pub fn get_last_aggregated_block(&self) -> u64 {
        self.last_aggregated_block
    }
}
