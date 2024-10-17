use crate::gnark::verify_gnark;
use crate::risc_zero::verify_risc_zero_proof;
use crate::sp1::verify_sp1_proof;
use aligned_sdk::core::types::{ProvingSystemId, VerificationData};
use log::{debug, warn};

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
        ProvingSystemId::Risc0 => {
            let Some(image_id_slice) = &verification_data.vm_program_code else {
                warn!(
                    "Trying to verify Risc0 proof but image id was not provided. Returning false"
                );
                return false;
            };

            // Risc0 can have 0 public input. In which case we supply an empty Vec<u8>.
            let pub_input = verification_data.pub_input.clone().unwrap_or_default();

            let mut image_id = [0u8; 32];
            image_id.copy_from_slice(image_id_slice.as_slice());
            return verify_risc_zero_proof(
                verification_data.proof.as_slice(),
                &image_id,
                &pub_input,
            );
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
    }
}
