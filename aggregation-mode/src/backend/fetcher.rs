use std::str::FromStr;

use super::{
    config::Config,
    types::{AlignedLayerServiceManager, AlignedLayerServiceManagerContract},
};
use crate::{
    backend::s3::get_aligned_batch_from_s3,
    zk::{backends::sp1::SP1Proof, Proof},
};
use aligned_sdk::core::types::ProvingSystemId;
use alloy::{primitives::Address, providers::ProviderBuilder};
use tracing::{error, info};

#[derive(Debug)]
pub enum ProofsFetcherError {
    QueryingLogs,
}

pub struct ProofsFetcher {
    aligned_service_manager: AlignedLayerServiceManagerContract,
}

impl ProofsFetcher {
    pub fn new(config: &Config) -> Self {
        let rpc_url = config.eth_rpc_url.parse().expect("correct url");
        let provider = ProviderBuilder::new().on_http(rpc_url);
        let aligned_service_manager = AlignedLayerServiceManager::new(
            Address::from_str(&config.aligned_service_manager_address)
                .expect("Address to be correct"),
            provider,
        );

        Self {
            aligned_service_manager,
        }
    }

    pub async fn fetch(&self) -> Result<Vec<Proof>, ProofsFetcherError> {
        info!("Fetching proofs from batch logs");
        // Subscribe to NewBatch event from AlignedServiceManager
        let logs = self
            .aligned_service_manager
            .NewBatchV3_filter()
            .from_block(0)
            .query()
            .await
            .map_err(|_| ProofsFetcherError::QueryingLogs)?;

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
            let proofs_to_add: Vec<Proof> = data
                .into_iter()
                .filter_map(|p| match p.proving_system {
                    ProvingSystemId::SP1 => {
                        let elf = p.vm_program_code?;
                        let proof = bincode::deserialize(&p.proof).ok()?;
                        let sp1_proof = SP1Proof { proof, elf };

                        Some(Proof::SP1(sp1_proof))
                    }
                    _ => None,
                })
                .collect();

            info!("SP1 proofs filtered, total proofs to add {}", proofs.len());

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
}
