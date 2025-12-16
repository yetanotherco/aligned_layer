use crate::{
    aggregators::{
        risc0_aggregator::Risc0ProofReceiptAndImageId, sp1_aggregator::SP1ProofWithPubValuesAndVk,
        AlignedProof, ZKVMEngine,
    },
    backend::db::{Db, DbError},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sqlx::types::Uuid;
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

    pub async fn fetch_pending_proofs(
        &self,
        engine: ZKVMEngine,
        limit: i64,
    ) -> Result<(Vec<AlignedProof>, Vec<Uuid>), ProofsFetcherError> {
        let tasks = self
            .db
            .get_pending_tasks_and_mark_them_as_processing(engine.proving_system_id() as i32, limit)
            .await
            .map_err(ProofsFetcherError::Query)?;

        let (proofs_to_aggregate, tasks_id): (Vec<AlignedProof>, Vec<Uuid>) = match engine {
            ZKVMEngine::SP1 => {
                let pairs: Vec<(AlignedProof, Uuid)> = tasks
                    .into_par_iter()
                    .filter_map(|task| {
                        let vk = bincode::deserialize(&task.program_commitment).ok()?;
                        let proof_with_pub_values = bincode::deserialize(&task.proof).ok()?;

                        match SP1ProofWithPubValuesAndVk::new(proof_with_pub_values, vk) {
                            Ok(proof) => Some((AlignedProof::SP1(proof.into()), task.task_id)),
                            Err(err) => {
                                error!("Could not add proof, verification failed: {:?}", err);
                                None
                            }
                        }
                    })
                    .collect();

                pairs.into_iter().unzip()
            }
            ZKVMEngine::RISC0 => {
                let pairs: Vec<(AlignedProof, Uuid)> = tasks
                    .into_par_iter()
                    .filter_map(|task| {
                        let mut image_id = [0u8; 32];
                        image_id.copy_from_slice(&task.program_commitment);
                        // we are inside a for_each callback so it returns for this particular iteration only
                        let receipt = bincode::deserialize(&task.proof).ok()?;

                        let risc0_proof = Risc0ProofReceiptAndImageId::new(image_id, receipt);

                        match risc0_proof {
                            Ok(proof) => Some((AlignedProof::Risc0(proof.into()), task.task_id)),
                            Err(err) => {
                                error!("Could not add proof, verification failed: {:?}", err);
                                None
                            }
                        }
                    })
                    .collect();

                pairs.into_iter().unzip()
            }
        };

        info!(
            "{} Proofs filtered, compatible proofs found {}",
            engine,
            proofs_to_aggregate.len()
        );

        Ok((proofs_to_aggregate, tasks_id))
    }
}
