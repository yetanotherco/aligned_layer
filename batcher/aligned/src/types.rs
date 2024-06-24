use aligned_batcher_lib::types::{BatchInclusionData, VerificationDataCommitment};
use lambdaworks_crypto::merkle_tree::proof::Proof;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AlignedVerificationData {
    pub verification_data_commitment: VerificationDataCommitment,
    pub batch_merkle_root: [u8; 32],
    pub batch_inclusion_proof: Proof<[u8; 32]>,
    pub index_in_batch: usize,
}

impl AlignedVerificationData {
    pub fn new(
        verification_data_commitment: &VerificationDataCommitment,
        inclusion_data: &BatchInclusionData,
    ) -> Self {
        let batch_merkle_root = inclusion_data.batch_merkle_root;
        let batch_inclusion_proof = &inclusion_data.batch_inclusion_proof;
        let index_in_batch = inclusion_data.index_in_batch;

        Self {
            verification_data_commitment: verification_data_commitment.clone(),
            batch_merkle_root,
            batch_inclusion_proof: batch_inclusion_proof.clone(),
            index_in_batch,
        }
    }
}
