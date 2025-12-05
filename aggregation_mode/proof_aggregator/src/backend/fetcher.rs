use crate::{
    aggregators::{
        risc0_aggregator::Risc0ProofReceiptAndImageId, sp1_aggregator::SP1ProofWithPubValuesAndVk,
        AlignedProof, ZKVMEngine,
    },
    backend::db::{Db, DbError},
};
use rayon::prelude::*;
use tracing::{error, info};

#[derive(Debug)]
pub enum ProofsFetcherError {
    Query(DbError),
}

pub struct ProofsFetcher {
    db: Db,
}

impl ProofsFetcher {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn query(
        &self,
        engine: ZKVMEngine,
        limit: i64,
    ) -> Result<Vec<AlignedProof>, ProofsFetcherError> {
        let tasks = self
            .db
            .get_pending_tasks_and_mark_them_as_processed(engine.proving_system_id() as i64, limit)
            .await
            .map_err(ProofsFetcherError::Query)?;

        let proofs_to_aggregate: Vec<AlignedProof> = match engine {
            ZKVMEngine::SP1 => tasks
                .into_par_iter()
                .filter_map(|task| {
                    let vk = bincode::deserialize(&task.program_commitment).ok()?;
                    let proof_with_pub_values = bincode::deserialize(&task.proof).ok()?;
                    let sp1_proof = SP1ProofWithPubValuesAndVk::new(proof_with_pub_values, vk);

                    match sp1_proof {
                        Ok(proof) => Some(AlignedProof::SP1(proof.into())),
                        Err(err) => {
                            error!("Could not add proof, verification failed: {:?}", err);
                            None
                        }
                    }
                })
                .collect(),
            ZKVMEngine::RISC0 => tasks
                .into_par_iter()
                .filter_map(|task| {
                    let mut image_id = [0u8; 32];
                    image_id.copy_from_slice(&task.program_commitment);
                    let receipt: risc0_zkvm::Receipt = bincode::deserialize(&task.proof).ok()?;

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
            proofs_to_aggregate.len()
        );

        Ok(proofs_to_aggregate)
    }
}
