use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Serialize, Deserialize)]
pub struct SP1VkAndPubInputs {
    pub vk: [u32; 8],
    pub public_inputs: Vec<u8>,
}

impl SP1VkAndPubInputs {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        for &word in &self.vk {
            hasher.update(word.to_be_bytes());
        }
        hasher.update(&self.public_inputs);
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize)]
pub enum ProofDataInput {
    SP1Compressed(SP1VkAndPubInputs),
}

impl ProofDataInput {
    pub fn hash(&self) -> [u8; 32] {
        match self {
            ProofDataInput::SP1Compressed(proof_data) => proof_data.hash(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub proofs_data: Vec<ProofDataInput>,
    pub merkle_root: [u8; 32],
}
