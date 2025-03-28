use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SP1CompressedProof {
    vk: Vec<u8>,
    public_inputs: Vec<Vec<u8>>,
}
