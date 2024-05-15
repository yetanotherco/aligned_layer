use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    SP1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub public_input: Vec<u8>,
    pub verification_key: Vec<u8>,
    pub quorum_numbers: Vec<u8>,
    pub quorum_threshold_percentages: Vec<u8>,
    pub fee: u64,
}
