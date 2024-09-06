use crate::halo2::ipa::verify_halo2_ipa;
use crate::halo2::kzg::verify_halo2_kzg;
use crate::risc_zero::verify_risc_zero_proof;
use crate::sp1::verify_sp1_proof;
use crate::{gnark::verify_gnark, mina::verify_proof_integrity};
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
            verify_proof_integrity(&verification_data.proof, pub_input)
            // TODO(xqft): add Pickles aggregator checks which are run alongside the Kimchi
            // verifier. These checks are fast and if they aren't successful then the Pickles proof
            // isn't valid.
        }
        ProvingSystemId::MinaAccount => {
            verification_data
                .pub_input
                .as_ref()
                .expect("Public input is required");
            true
            // TODO(xqft): add basic integrity checks (e.g. length of merkle proof being multiple of 32
            // bytes, etc)
        }
    }
}
