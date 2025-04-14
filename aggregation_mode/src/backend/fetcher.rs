use std::str::FromStr;

use super::{
    config::Config,
    types::{AlignedLayerServiceManager, AlignedLayerServiceManagerContract, RPCProvider},
};
use crate::{
    aggregators::{sp1_aggregator::SP1ProofWithPubValuesAndElf, AlignedProof},
    backend::s3::get_aligned_batch_from_s3,
};
use aligned_sdk::core::types::ProvingSystemId;
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use tracing::{error, info};

#[derive(Debug)]
pub enum ProofsFetcherError {
    GetLogs(String),
    GetBlockNumber(String),
}

pub struct ProofsFetcher {
    rpc_provider: RPCProvider,
    aligned_service_manager: AlignedLayerServiceManagerContract,
    fetch_from_secs_ago: u64,
    block_time_secs: u64,
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

        Self {
            rpc_provider,
            aligned_service_manager,
            fetch_from_secs_ago: config.fetch_logs_from_secs_ago,
            block_time_secs: config.block_time_secs,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<AlignedProof>, ProofsFetcherError> {
        let from_block = self.get_block_number_to_fetch_from().await?;
        info!(
            "Fetching proofs from batch logs starting from block number {}",
            from_block
        );
        // Subscribe to NewBatch event from AlignedServiceManager
        let logs = self
            .aligned_service_manager
            .NewBatchV3_filter()
            .from_block(from_block)
            .query()
            .await
            .map_err(|e| ProofsFetcherError::GetLogs(e.to_string()))?;

        info!("Logs collected {}", logs.len());

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

            // Filter SP1 compressed proofs to and push to queue to be aggregated
            let proofs_to_add: Vec<AlignedProof> = data
                .into_iter()
                .filter_map(|p| match p.proving_system {
                    ProvingSystemId::SP1 => {
                        let elf = p.vm_program_code?;
                        let proof_with_pub_values = bincode::deserialize(&p.proof).ok()?;
                        let sp1_proof = SP1ProofWithPubValuesAndElf {
                            proof_with_pub_values,
                            elf,
                        };

                        Some(AlignedProof::SP1(sp1_proof))
                    }
                    _ => None,
                })
                .collect();

            info!(
                "SP1 proofs filtered, total proofs to add {}",
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

    async fn get_block_number_to_fetch_from(&self) -> Result<u64, ProofsFetcherError> {
        let block_number = self
            .rpc_provider
            .get_block_number()
            .await
            .map_err(|e| ProofsFetcherError::GetBlockNumber(e.to_string()))?;

        let number_of_blocks_in_the_past = self.fetch_from_secs_ago / self.block_time_secs;

        Ok(block_number.saturating_sub(number_of_blocks_in_the_past))
    }
}
