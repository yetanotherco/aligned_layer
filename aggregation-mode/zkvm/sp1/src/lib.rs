use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Serialize, Deserialize)]
pub struct SP1ProofInput {
    pub vk: [u32; 8],
    pub public_inputs: Vec<u8>,
}

impl SP1ProofInput {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        for &word in &self.vk {
            hasher.update(word.to_le_bytes());
        }
        hasher.update(&self.public_inputs);
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize)]
pub enum ProofInput {
    SP1Compressed(SP1ProofInput),
}

impl ProofInput {
    pub fn hash(&self) -> [u8; 32] {
        match self {
            ProofInput::SP1Compressed(proof) => proof.hash(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub proofs: Vec<ProofInput>,
    pub merkle_root: [u8; 32],
}
