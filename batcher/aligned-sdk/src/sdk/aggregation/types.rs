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

impl IsMerkleTreeBackend for Hash32 {
    type Data = Hash32;
    type Node = [u8; 32];

    /// We don't have to hash the data, as the blob already contains the proof commitments (which represent the merkle leaves)
    fn hash_data(leaf: &Self::Data) -> Self::Node {
        leaf.0
    }

    /// Leaves don't have to be hashed as the blob already contains the proof commitments (which represent the merkle leaves)
    fn hash_leaves(leaves: &[Self::Data]) -> Vec<Self::Node> {
        leaves.iter().map(|l| l.0).collect()
    }

    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        hasher.update(child_1);
        hasher.update(child_2);
        hasher.finalize().into()
    }
}

#[derive(Debug)]
pub enum ProofVerificationAggModeError {
    ProvingSystemNotSupportedInAggMode,
    EthereumProviderError(String),
    BeaconClient(BeaconClientError),
    ProofNotFoundInLogs,
    EventDecoding,
    MerkleTreeConstruction,
    MerkleTreeProofVerification,
}
