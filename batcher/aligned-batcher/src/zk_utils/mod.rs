use crate::gnark::verify_gnark;
use crate::halo2::ipa::verify_halo2_ipa;
use crate::halo2::kzg::verify_halo2_kzg;
use crate::sp1::verify_sp1_proof;
use crate::jolt::verify_jolt_proof;
use aligned_batcher_lib::types::{ProvingSystemId, VerificationData};
use log::{debug, warn};

pub(crate) fn verify(verification_data: &VerificationData) -> bool {
    match verification_data.proving_system {
        ProvingSystemId::SP1 => {
            if let Some(elf) = &verification_data.vm_program_code {
                return verify_sp1_proof(verification_data.proof.as_slice(), elf.as_slice());
            }
            warn!("Trying to verify SP1 proof but ELF was not provided. Returning false");
            false
        }
        ProvingSystemId::Jolt => {
            if let Some(elf) = &verification_data.vm_program_code {
                return verify_jolt_proof(verification_data.proof.as_slice(), elf.as_slice());
            }
            warn!("Trying to verify Jolt proof but ELF was not provided. Returning false");
            false
        }
        ProvingSystemId::Halo2KZG => {
            let vk = verification_data
                .verification_key
                .as_ref()
                .expect("Verification key is required");

            let pub_input = verification_data
                .pub_input
                .as_ref()
                .expect("Public input is required");
            let is_valid = verify_halo2_kzg(&verification_data.proof, pub_input, vk);
            debug!("Halo2-KZG proof is valid: {}", is_valid);
            is_valid
        }
        ProvingSystemId::Halo2IPA => {
            let vk = verification_data
                .verification_key
                .as_ref()
                .expect("Verification key is required");

            let pub_input = verification_data
                .pub_input
                .as_ref()
                .expect("Public input is required");
            let is_valid = verify_halo2_ipa(&verification_data.proof, pub_input, vk);
            debug!("Halo2-IPA proof is valid: {}", is_valid);
            is_valid
        }
        ProvingSystemId::GnarkPlonkBls12_381
        | ProvingSystemId::GnarkPlonkBn254
        | ProvingSystemId::Groth16Bn254 => {
            let vk = verification_data
                .verification_key
                .as_ref()
                .expect("Verification key is required");

            let pub_input = verification_data
                .pub_input
                .as_ref()
                .expect("Public input is required");
            let is_valid = verify_gnark(
                &verification_data.proving_system,
                &verification_data.proof,
                pub_input,
                vk,
            );
            debug!("Gnark proof is valid: {}", is_valid);
            is_valid
        }
    }
}
