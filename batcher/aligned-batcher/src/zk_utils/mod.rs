use crate::gnark::verify_gnark;
use crate::halo2::ipa::verify_halo2_ipa;
use crate::halo2::kzg::verify_halo2_kzg;
use crate::risc_zero::verify_risc_zero_proof;
use crate::sp1::verify_sp1_proof;
use aligned_sdk::core::types::{
    ProofInvalidReason, ProvingSystemId, ValidityResponseMessage, VerificationData,
};
use ethers::types::U256;
use log::{debug, warn};

pub(crate) async fn verify(
    verification_data: &VerificationData,
    blacklisted_verifiers: U256,
) -> ValidityResponseMessage {
    let verification_data = verification_data.clone();
    tokio::task::spawn_blocking(move || verify_internal(&verification_data, blacklisted_verifiers))
        .await
        .unwrap_or(ValidityResponseMessage::InvalidProof(
            ProofInvalidReason::Unknown,
        ))
}

fn verify_internal(
    verification_data: &VerificationData,
    blacklisted_verifiers: U256,
) -> ValidityResponseMessage {
    if blacklisted_verifiers & (U256::one() << verification_data.proving_system.clone() as u64)
        != U256::zero()
    {
        warn!(
            "Verifier {} is blacklisted, skipping verification",
            verification_data.proving_system
        );
        return ValidityResponseMessage::InvalidProof(ProofInvalidReason::BlacklistedVerifier);
    }
    match verification_data.proving_system {
        ProvingSystemId::SP1 => {
            if let Some(elf) = &verification_data.vm_program_code {
                let result = verify_sp1_proof(verification_data.proof.as_slice(), elf.as_slice());
                if result {
                    return ValidityResponseMessage::Valid;
                } else {
                    return ValidityResponseMessage::InvalidProof(ProofInvalidReason::Unknown);
                }
            }
            warn!("Trying to verify SP1 proof but ELF was not provided. Returning invalid");
            ValidityResponseMessage::InvalidProof(ProofInvalidReason::MissingVerificationData)
        }
        ProvingSystemId::Halo2KZG => {
            let vk = match verification_data.verification_key.as_ref() {
                Some(vk) => vk,
                None => {
                    warn!("Halo2-KZG verification key missing");
                    return ValidityResponseMessage::InvalidProof(
                        ProofInvalidReason::MissingVerificationData,
                    );
                }
            };

            let pub_input = match verification_data.pub_input.as_ref() {
                Some(pub_input) => pub_input,
                None => {
                    warn!("Halo2-KZG public input missing");
                    return ValidityResponseMessage::InvalidProof(
                        ProofInvalidReason::MissingVerificationData,
                    );
                }
            };

            let is_valid = verify_halo2_kzg(&verification_data.proof, pub_input, vk);
            debug!("Halo2-KZG proof is valid: {}", is_valid);
            if is_valid {
                ValidityResponseMessage::Valid
            } else {
                ValidityResponseMessage::InvalidProof(ProofInvalidReason::Unknown)
            }
        }
        ProvingSystemId::Halo2IPA => {
            let vk = match verification_data.verification_key.as_ref() {
                Some(vk) => vk,
                None => {
                    warn!("Halo2-IPA verification key missing");
                    return ValidityResponseMessage::InvalidProof(
                        ProofInvalidReason::MissingVerificationData,
                    );
                }
            };

            let pub_input = match verification_data.pub_input.as_ref() {
                Some(pub_input) => pub_input,
                None => {
                    warn!("Halo2-IPA public input missing");
                    return ValidityResponseMessage::InvalidProof(
                        ProofInvalidReason::MissingVerificationData,
                    );
                }
            };

            let is_valid = verify_halo2_ipa(&verification_data.proof, pub_input, vk);
            debug!("Halo2-IPA proof is valid: {}", is_valid);
            if is_valid {
                ValidityResponseMessage::Valid
            } else {
                ValidityResponseMessage::InvalidProof(ProofInvalidReason::Unknown)
            }
        }
        ProvingSystemId::Risc0 => {
            if let (Some(image_id_slice), Some(pub_input)) = (
                &verification_data.vm_program_code,
                &verification_data.pub_input,
            ) {
                let mut image_id = [0u8; 32];
                image_id.copy_from_slice(image_id_slice.as_slice());
                let result = verify_risc_zero_proof(
                    verification_data.proof.as_slice(),
                    &image_id,
                    pub_input,
                );
                if result {
                    return ValidityResponseMessage::Valid;
                } else {
                    return ValidityResponseMessage::InvalidProof(ProofInvalidReason::Unknown);
                }
            }

            warn!("Trying to verify Risc0 proof but image id or public input was not provided. Returning false");
            ValidityResponseMessage::InvalidProof(ProofInvalidReason::MissingVerificationData)
        }
        ProvingSystemId::GnarkPlonkBls12_381
        | ProvingSystemId::GnarkPlonkBn254
        | ProvingSystemId::Groth16Bn254 => {
            let vk = match verification_data.verification_key.as_ref() {
                Some(vk) => vk,
                None => {
                    warn!("Gnark verification key missing");
                    return ValidityResponseMessage::InvalidProof(
                        ProofInvalidReason::MissingVerificationData,
                    );
                }
            };

            let pub_input = match verification_data.pub_input.as_ref() {
                Some(pub_input) => pub_input,
                None => {
                    warn!("Gnark public input missing");
                    return ValidityResponseMessage::InvalidProof(
                        ProofInvalidReason::MissingVerificationData,
                    );
                }
            };

            let is_valid = verify_gnark(
                &verification_data.proving_system,
                &verification_data.proof,
                pub_input,
                vk,
            );
            debug!("Gnark proof is valid: {}", is_valid);
            if is_valid {
                ValidityResponseMessage::Valid
            } else {
                ValidityResponseMessage::InvalidProof(ProofInvalidReason::Unknown)
            }
        }
    }
}
