use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Serialize, Deserialize)]
pub struct SP1VkAndPubInputs {
    pub vk: [u32; 8],
    pub public_inputs: Vec<u8>,
}

impl SP1VkAndPubInputs {
    pub fn commitment(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        for &word in &self.vk {
            hasher.update(word.to_be_bytes());
        }
        hasher.update(&self.public_inputs);
        hasher.finalize().into()
    }
}

// Note: this MerkleTreeBackend is defined in three locations
// - aggregation_mode/src/aggregators/mod.rs
// - aggregation_mode/src/aggregators/risc0_aggregator.rs and
// - aggregation_mode/src/aggregators/sp1_aggregator.rs
// All 3 implementations should match
// The definition on aggregator/mod.rs supports taking proofs from both Risc0 and SP1,
// Additionally, a version that takes the leaves as already hashed data is defined on:
// - crates/sdk/src/aggregation_layer/types.rs
// This one is used in the SDK since
// the user may not have access to the proofs that they didn't submit
impl IsMerkleTreeBackend for SP1VkAndPubInputs {
    type Data = SP1VkAndPubInputs;
    type Node = [u8; 32];

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        leaf.commitment()
    }

    /// Computes a commutative Keccak256 hash, ensuring H(a, b) == H(b, a).
    ///
    /// See: https://docs.openzeppelin.com/contracts/5.x/api/utils#Hashes
    ///
    /// Source: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/Hashes.sol#L17-L19
    ///
    /// Compliant with OpenZeppelin's `processProofCalldata` function from MerkleProof.sol.
    ///
    /// See: https://docs.openzeppelin.com/contracts/5.x/api/utils#MerkleProof
    ///
    /// Source: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/MerkleProof.sol#L114-L128
    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        if child_1 < child_2 {
            hasher.update(child_1);
            hasher.update(child_2);
        } else {
            hasher.update(child_2);
            hasher.update(child_1);
        }
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Hash32(pub [u8; 32]);

// Note: this MerkleTreeBackend is defined in three locations
// - aggregation_mode/src/aggregators/mod.rs
// - aggregation_mode/src/aggregators/risc0_aggregator.rs and
// - aggregation_mode/src/aggregators/sp1_aggregator.rs
// All 3 implementations should match
// The definition on aggregator/mod.rs supports taking proofs from both Risc0 and SP1,
// Additionally, a version that takes the leaves as already hashed data is defined on:
// - batcher/aligned-sdk/src/sdk/aggregation.rs
// This one is used in the SDK since
// the user may not have access to the proofs that they didn't submit
impl IsMerkleTreeBackend for Hash32 {
    type Data = Hash32;
    type Node = [u8; 32];

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        leaf.0
    }

    /// Computes a commutative Keccak256 hash, ensuring H(a, b) == H(b, a).
    ///
    /// See: https://docs.openzeppelin.com/contracts/5.x/api/utils#Hashes
    ///
    /// Source: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/Hashes.sol#L17-L19
    ///
    /// Compliant with OpenZeppelin's `processProofCalldata` function from MerkleProof.sol.
    ///
    /// See: https://docs.openzeppelin.com/contracts/5.x/api/utils#MerkleProof
    ///
    /// Source: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/MerkleProof.sol#L114-L128
    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        if child_1 < child_2 {
            hasher.update(child_1);
            hasher.update(child_2);
        } else {
            hasher.update(child_2);
            hasher.update(child_1);
        }
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserProofsAggregatorInput {
    pub proofs_vk_and_pub_inputs: Vec<SP1VkAndPubInputs>,
}

#[derive(Serialize, Deserialize)]
pub struct ChunkAggregatorInput {
    pub proofs_and_leaves_commitment: Vec<(SP1VkAndPubInputs, Vec<[u8; 32]>)>,
}
