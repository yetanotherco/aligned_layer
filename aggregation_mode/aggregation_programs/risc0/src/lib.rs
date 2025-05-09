use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

#[derive(Serialize, Deserialize)]
pub struct Risc0ImageIdAndPubInputs {
    pub image_id: [u8; 32],
    pub public_inputs: Vec<u8>,
}

impl Risc0ImageIdAndPubInputs {
    pub fn commitment(&self) -> [u8; 32] {
        let mut hasher = Keccak::v256();
        for &word in &self.image_id {
            hasher.update(&word.to_be_bytes());
        }
        hasher.update(&self.public_inputs);

        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        hash
    }
}

// Note: this MerkleTreeBackend is defined in three locations
// - aggregation_mode/src/aggregators/mod.rs
// - aggregation_mode/src/aggregators/risc0_aggregator.rs
// - aggregation_mode/src/aggregators/sp1_aggregator.rs
// All 3 implementations should match
// The definition on aggregator/mod.rs supports taking proofs from both Risc0 and SP1,
// Additionally, a version that takes the leaves as already hashed data is defined on:
// - batcher/aligned-sdk/src/sdk/aggregation.rs
// This one is used in the SDK since,
// the user may not have access to the proofs that he didn't submit
impl IsMerkleTreeBackend for Risc0ImageIdAndPubInputs {
    type Data = Risc0ImageIdAndPubInputs;
    type Node = [u8; 32];

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        leaf.commitment()
    }

    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak::v256();
        hasher.update(child_1);
        hasher.update(child_2);

        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        hash
    }
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub proofs_image_id_and_pub_inputs: Vec<Risc0ImageIdAndPubInputs>,
}
