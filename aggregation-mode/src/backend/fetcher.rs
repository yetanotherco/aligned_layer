use std::{str::FromStr, sync::Arc};

use super::{
    config::Config,
    queue::ProofsQueue,
    s3::S3Client,
    types::{AlignedLayerServiceManager, AlignedLayerServiceManagerContract},
};
use crate::zk::{
    backends::sp1::{vk_from_elf, SP1Proof},
    Proof,
};
use aligned_sdk::core::types::ProvingSystemId;
use alloy::{
    primitives::Address,
    providers::{ProviderBuilder, WsConnect},
};
use futures_util::stream::StreamExt;
use tokio::sync::Mutex;
use tracing::{error, info};

/// This services is in charge of:
/// 1. Listens to aligned new batch task
/// 2. Downloads proofs from S3 bucket
/// 3. Filter supported proofs to be aggregated
/// 4. Push the proofs to the queue
pub struct ProofsFetcher {
    aligned_service_manager: AlignedLayerServiceManagerContract,
    s3: S3Client,
    queue: Arc<Mutex<ProofsQueue>>,
}

impl ProofsFetcher {
    pub async fn new(config: &Config, queue: Arc<Mutex<ProofsQueue>>) -> Self {
        let ws = WsConnect::new(&config.eth_ws_url);
        let provider = ProviderBuilder::new()
            .on_ws(ws)
            .await
            .expect("Successful connection");

        let aligned_service_manager = AlignedLayerServiceManager::new(
            Address::from_str(&config.aligned_service_manager_address)
                .expect("Address to be correct"),
            provider,
        );

        let s3 = S3Client::new(config.bucket_name.clone(), None).await;

        Self {
            aligned_service_manager,
            s3,
            queue,
        }
    }

    pub async fn start(&self) {
        // Subscribe to NewBatch event from AlignedServiceManager
        let event_sub = self
            .aligned_service_manager
            .NewBatch_filter()
            .subscribe()
            .await
            .expect("To subscribe to event");
        let mut stream = event_sub.into_stream();

        while let Some(log) = stream.next().await {
            let Ok(log) = log else {
                continue;
            };

            // Download batch proofs from s3
            let Ok(data) = self.s3.get_aligned_batch(log.0.batchDataPointer).await else {
                error!("Error while downloading proofs from s3");
                continue;
            };

            // Filter SP1 compressed proofs to and push to queue to be aggregated
            let proofs: Vec<(Proof, Vec<u8>)> = data
                .into_iter()
                .filter_map(|p| match p.proving_system {
                    ProvingSystemId::SP1 => {
                        let elf = p.vm_program_code?;
                        let proof = bincode::deserialize(&p.proof).ok()?;
                        let sp1_proof = SP1Proof {
                            proof,
                            vk: vk_from_elf(&elf),
                        };

                        Some((Proof::SP1(sp1_proof), elf))
                    }
                    _ => None,
                })
                .collect();

            // try to add them to the queue
            let mut queue_lock = self.queue.lock().await;
            for (proof, elf) in proofs {
                match queue_lock.add_proof(proof, &elf) {
                    Ok(_) => info!(
                        "New proof added to queue, current length {}",
                        queue_lock.proofs().len()
                    ),
                    Err(e) => error!("Could not add proof, reason: {:?}", e),
                };
            }
        }
    }
}
