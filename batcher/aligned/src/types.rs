use aligned_batcher_lib::types::{
    BatchInclusionData, VerificationCommitmentBatch, VerificationData, VerificationDataCommitment,
};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{Signer, Wallet},
    types::Signature,
};
use lambdaworks_crypto::merkle_tree::{
    merkle::MerkleTree, proof::Proof, traits::IsMerkleTreeBackend,
};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessage {
    pub verification_data: VerificationData,
    pub signature: Signature,
}

impl ClientMessage {
    pub async fn new(verification_data: VerificationData, wallet: Wallet<SigningKey>) -> Self {
        let verification_data_str = serde_json::to_string(&verification_data).unwrap();
        let hashed_leaf = VerificationCommitmentBatch::hash_one(verification_data.into());
        let signature = wallet.sign_message(&verification_data_str).await.unwrap();

        ClientMessage {
            verification_data,
            signature,
        }
    }
}
