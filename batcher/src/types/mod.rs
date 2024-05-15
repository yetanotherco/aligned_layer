use std::fmt::Debug;
use bytes::Bytes;
use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    #[default]
    SP1,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Task {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub public_input: Vec<u8>,
    pub verification_key: Vec<u8>,
    pub quorum_numbers: Vec<u8>,
    pub quorum_threshold_percentages: Vec<u8>,
    pub fee: u64,
}


impl IsMerkleTreeBackend for Task {
    type Node = Bytes;
    type Data = Task;

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        let leaf_bytes = bincode::serialize(leaf).expect("Failed to serialize leaf");
        let mut hasher = Sha3_256::new();
        hasher.update(&leaf_bytes);
        hasher.finalize().to_vec().into()
    }

    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Sha3_256::new();
        hasher.update(child_1);
        hasher.update(child_2);
        hasher.finalize().to_vec().into()
    }
}
