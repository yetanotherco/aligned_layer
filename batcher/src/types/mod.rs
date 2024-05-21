use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    SP1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationData {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub public_input: Option<Vec<u8>>,
    pub verification_key: Option<Vec<u8>>,
    pub vm_program_code: Option<Vec<u8>>
}

pub fn get_proving_system_from_str(proving_system: &str) -> ProvingSystemId {
    match proving_system {
        "GnarkPlonkBls12_381" => ProvingSystemId::GnarkPlonkBls12_381,
        "GnarkPlonkBn254" => ProvingSystemId::GnarkPlonkBn254,
        "Groth16Bn254" => ProvingSystemId::Groth16Bn254,
        "SP1" => ProvingSystemId::SP1,
        _ => panic!("Invalid proving system: {}\nAvailable prooving systems:\n GnarkPlonkBls12_381\n GnarkPlonkBn254\n Groth16Bn254\n SP1", proving_system),
    }
}
