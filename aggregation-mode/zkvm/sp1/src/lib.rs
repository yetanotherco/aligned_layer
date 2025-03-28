use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct SP1CompressedProof {
    vk: Vec<u8>,
    pub public_inputs: Vec<u8>,
}

impl SP1CompressedProof {
    pub fn vk(&self) -> [u32; 8] {
        assert!(self.vk.len() >= 32, "vk must be at least 32 bytes long");

        let mut bytes = [0_32; 8];

        for (i, chunk) in self.vk.chunks_exact(4).enumerate() {
            bytes[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        bytes
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.vk);
        hasher.update(&self.public_inputs);
        hasher.finalize().into()
    }
}
