use crate::gnark::verify_gnark;
use crate::halo2::ipa::verify_halo2_ipa;
use crate::halo2::kzg::verify_halo2_kzg;
use crate::risc_zero::verify_risc_zero_proof;
use crate::sp1::verify_sp1_proof;
use aligned_sdk::core::types::{ProvingSystemId, VerificationData};
use log::{debug, warn};
use mina_account_verifier_ffi::verify_account_inclusion_ffi;
use mina_state_verifier_ffi::verify_mina_state_ffi;

pub(crate) async fn verify(verification_data: &VerificationData) -> bool {
    let verification_data = verification_data.clone();
    tokio::task::spawn_blocking(move || verify_internal(&verification_data))
        .await
        .unwrap_or(false)
}

fn verify_internal(verification_data: &VerificationData) -> bool {
    match verification_data.proving_system {
        ProvingSystemId::SP1 => {
            if let Some(elf) = &verification_data.vm_program_code {
                return verify_sp1_proof(verification_data.proof.as_slice(), elf.as_slice());
            }
            warn!("Trying to verify SP1 proof but ELF was not provided. Returning false");
            false
        }
        ProvingSystemId::Halo2KZG => {
            let vk = match verification_data.verification_key.as_ref() {
                Some(vk) => vk,
                None => {
                    warn!("Halo2-KZG verification key missing");
                    return false;
                }
            };

            let pub_input = match verification_data.pub_input.as_ref() {
                Some(pub_input) => pub_input,
                None => {
                    warn!("Halo2-KZG public input missing");
                    return false;
                }
            };

            let is_valid = verify_halo2_kzg(&verification_data.proof, pub_input, vk);
            debug!("Halo2-KZG proof is valid: {}", is_valid);
            is_valid
        }
        ProvingSystemId::Halo2IPA => {
            let vk = match verification_data.verification_key.as_ref() {
                Some(vk) => vk,
                None => {
                    warn!("Halo2-IPA verification key missing");
                    return false;
                }
            };

            let pub_input = match verification_data.pub_input.as_ref() {
                Some(pub_input) => pub_input,
                None => {
                    warn!("Halo2-IPA public input missing");
                    return false;
                }
            };

            let is_valid = verify_halo2_ipa(&verification_data.proof, pub_input, vk);
            debug!("Halo2-IPA proof is valid: {}", is_valid);
            is_valid
        }
        ProvingSystemId::Risc0 => {
            if let (Some(image_id_slice), Some(pub_input)) = (
                &verification_data.vm_program_code,
                &verification_data.pub_input,
            ) {
                let mut image_id = [0u8; 32];
                image_id.copy_from_slice(image_id_slice.as_slice());
                return verify_risc_zero_proof(
                    verification_data.proof.as_slice(),
                    &image_id,
                    pub_input,
                );
            }

            warn!("Trying to verify Risc0 proof but image id or public input was not provided. Returning false");
            false
        }
        ProvingSystemId::GnarkPlonkBls12_381
        | ProvingSystemId::GnarkPlonkBn254
        | ProvingSystemId::Groth16Bn254 => {
            let vk = match verification_data.verification_key.as_ref() {
                Some(vk) => vk,
                None => {
                    warn!("Gnark verification key missing");
                    return false;
                }
            };

            let pub_input = match verification_data.pub_input.as_ref() {
                Some(pub_input) => pub_input,
                None => {
                    warn!("Gnark public input missing");
                    return false;
                }
            };

            let is_valid = verify_gnark(
                &verification_data.proving_system,
                &verification_data.proof,
                pub_input,
                vk,
            );
            debug!("Gnark proof is valid: {}", is_valid);
            is_valid
        }
        ProvingSystemId::Mina => {
            let pub_input = verification_data
                .pub_input
                .as_ref()
                .expect("Public input is required");

            const MAX_PROOF_SIZE: usize = 48 * 1024;
            const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;

            let mut proof_buffer = [0; MAX_PROOF_SIZE];
            for (buffer_item, proof_item) in proof_buffer.iter_mut().zip(&verification_data.proof) {
                *buffer_item = *proof_item;
            }
            let proof_len = verification_data.proof.len();

            let mut pub_input_buffer = [0; MAX_PUB_INPUT_SIZE];
            for (buffer_item, pub_input_item) in pub_input_buffer.iter_mut().zip(pub_input) {
                *buffer_item = *pub_input_item;
            }
            let pub_input_len = pub_input.len();

            verify_mina_state_ffi(&proof_buffer, proof_len, &pub_input_buffer, pub_input_len)
        }
        ProvingSystemId::MinaAccount => {
            let pub_input = verification_data
                .pub_input
                .as_ref()
                .expect("Public input is required");

            const MAX_PROOF_SIZE: usize = 16 * 1024;
            const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;

            let mut proof_buffer = [0; MAX_PROOF_SIZE];
            for (buffer_item, proof_item) in proof_buffer.iter_mut().zip(&verification_data.proof) {
                *buffer_item = *proof_item;
            }
            let proof_len = verification_data.proof.len();

            let mut pub_input_buffer = [0; MAX_PUB_INPUT_SIZE];
            for (buffer_item, pub_input_item) in pub_input_buffer.iter_mut().zip(pub_input) {
                *buffer_item = *pub_input_item;
            }
            let pub_input_len = pub_input.len();

            verify_account_inclusion_ffi(&proof_buffer, proof_len, &pub_input_buffer, pub_input_len)
        }
    }
}
