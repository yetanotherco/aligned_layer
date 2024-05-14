use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    SP1,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    proving_system: ProvingSystemId,
    proof: Vec<u8>,
    public_input: Vec<u8>,
    verification_key: Vec<u8>,
    quorum_numbers: Vec<u8>,
    quorum_threshold_percentages: Vec<u8>,
    fee: u64,
}
