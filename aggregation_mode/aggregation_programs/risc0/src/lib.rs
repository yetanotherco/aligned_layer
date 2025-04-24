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

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub proofs_image_id_and_pub_inputs: Vec<Risc0ImageIdAndPubInputs>,
}
