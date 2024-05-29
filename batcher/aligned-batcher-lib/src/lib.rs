use alloy_primitives::Address;
use anyhow::anyhow;
use lazy_static::lazy_static;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use sp1_sdk::ProverClient;
use gnark::verify_gnark;

mod gnark;

lazy_static! {
    static ref SP1_PROVER_CLIENT: ProverClient = ProverClient::new();
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    #[default]
    SP1,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VerificationData {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub pub_input: Option<Vec<u8>>,
    pub verification_key: Option<Vec<u8>>,
    pub vm_program_code: Option<Vec<u8>>,
    pub proof_generator_addr: Address,
}

impl VerificationData {
    pub fn verify(&self) -> bool {
        match self.proving_system {
            ProvingSystemId::SP1 => {
                if let Some(elf) = &self.vm_program_code {
                    return verify_sp1_proof(self.proof.as_slice(), elf.as_slice());
                }
                warn!("Trying to verify SP1 proof but ELF was not provided. Returning false");
                false
            }

            ProvingSystemId::GnarkPlonkBls12_381
            | ProvingSystemId::GnarkPlonkBn254
            | ProvingSystemId::Groth16Bn254 => {
                let vk = &self
                    .verification_key
                    .as_ref()
                    .expect("Verification key is required");

                let pub_input = &self.pub_input.as_ref().expect("Public input is required");
                let is_valid = verify_gnark(&self.proving_system, &self.proof, pub_input, vk);
                debug!("Gnark proof is valid: {}", is_valid);
                is_valid
            }
        }
    }
}

fn verify_sp1_proof(proof: &[u8], elf: &[u8]) -> bool {
    let (_pk, vk) = SP1_PROVER_CLIENT.setup(elf);
    if let Ok(proof) = bincode::deserialize(proof) {
        return SP1_PROVER_CLIENT.verify(&proof, &vk).is_ok();
    }

    false
}

pub fn parse_proving_system(proving_system: &str) -> anyhow::Result<ProvingSystemId> {
    match proving_system {
        "GnarkPlonkBls12_381" => Ok(ProvingSystemId::GnarkPlonkBls12_381),
        "GnarkPlonkBn254" => Ok(ProvingSystemId::GnarkPlonkBn254),
        "Groth16Bn254" => Ok(ProvingSystemId::Groth16Bn254),
        "SP1" => Ok(ProvingSystemId::SP1),
        _ => Err(anyhow!("Invalid proving system: {}, Available proving systems are: [GnarkPlonkBls12_381, GnarkPlonkBn254, Groth16Bn254, SP1]", proving_system))
    }
}
