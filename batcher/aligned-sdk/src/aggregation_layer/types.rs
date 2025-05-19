use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use sha3::{Digest, Keccak256};

use crate::beacon::BeaconClientError;

#[derive(Debug)]
pub enum AggregationModeVerificationData {
    SP1 {
        vk: [u8; 32],
        public_inputs: Vec<u8>,
    },
    Risc0 {
        image_id: [u8; 32],
        public_inputs: Vec<u8>,
    },
}

impl AggregationModeVerificationData {
    pub fn program_id(&self) -> [u8; 32] {
        match self {
            Self::Risc0 { image_id, .. } => *image_id,
            Self::SP1 { vk, .. } => *vk,
        }
    }

    pub fn public_inputs(&self) -> &Vec<u8> {
        match self {
            Self::Risc0 { public_inputs, .. } => public_inputs,
            Self::SP1 { public_inputs, .. } => public_inputs,
        }
    }

    pub fn commitment(&self) -> [u8; 32] {
        match self {
            AggregationModeVerificationData::SP1 { vk, public_inputs } => {
                let mut hasher = Keccak256::new();
                hasher.update(vk);
                hasher.update(public_inputs);
                hasher.finalize().into()
            }
            AggregationModeVerificationData::Risc0 {
                image_id,
                public_inputs,
            } => {
                let mut hasher = Keccak256::new();
                hasher.update(image_id);
                hasher.update(public_inputs);
                hasher.finalize().into()
            }
        }
    }
}

// We use a newtype wrapper around `[u8; 32]` because Rust's orphan rule
// prevents implementing a foreign trait (`IsMerkleTreeBackend`) for a foreign type (`[u8; 32]`).
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Hash32(pub [u8; 32]);

// Note:
// We define a version of the backend that takes the leaves as hashed data
// since the user may not have access to the proofs that he didn't submit
// The original MerkleTreeBackend is defined in three locations
// - aggregation_mode/src/aggregators/mod.rs
// - aggregation_mode/src/aggregators/risc0_aggregator.rs
// - aggregation_mode/src/aggregators/sp1_aggregator.rs
// The definition on aggregator/mod.rs supports taking proofs from both Risc0 and SP1
// Hashes of all implementations should match
impl IsMerkleTreeBackend for Hash32 {
    type Data = Hash32;
    type Node = [u8; 32];

    /// We don't have to hash the data, as the blob already contains the proof commitments (which represent the merkle leaves)
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

#[derive(Debug, Clone)]
pub enum ProofVerificationAggModeError {
    ProvingSystemNotSupportedInAggMode,
    EthereumProviderError(String),
    BeaconClient(BeaconClientError),
    EventDecoding,
    MerkleTreeConstruction,
}
