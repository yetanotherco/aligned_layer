use crate::gnark::verify_gnark;
use crate::risc_zero::verify_risc_zero_proof;
use crate::sp1::verify_sp1_proof;
use aligned_sdk::core::types::{ProvingSystemId, VerificationData};
use ethers::types::U256;
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
            let Some(elf) = &verification_data.vm_program_code else {
                warn!("Trying to verify SP1 proof but ELF was not provided. Returning invalid");
                return false;
            };
            verify_sp1_proof(verification_data.proof.as_slice(), elf.as_slice())
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
            let Some(vk) = verification_data.verification_key.as_ref() else {
                warn!("Gnark verification key missing");
                return false;
            };

            let Some(pub_input) = verification_data.pub_input.as_ref() else {
                warn!("Gnark public input missing");
                return false;
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

pub(crate) fn is_verifier_disabled(
    disabled_verifiers: U256,
    proving_system: ProvingSystemId,
) -> bool {
    disabled_verifiers & (U256::one() << proving_system as u64) != U256::zero()
}

#[cfg(test)]
mod test {
    use super::is_verifier_disabled;
    use aligned_sdk::core::types::{ProvingSystemId, VerificationData};
    use ethers::types::Address;

    fn get_all_verifiers() -> Vec<ProvingSystemId> {
        let verifiers = vec![
            ProvingSystemId::GnarkPlonkBls12_381,
            ProvingSystemId::GnarkPlonkBn254,
            ProvingSystemId::Groth16Bn254,
            ProvingSystemId::SP1,
            ProvingSystemId::Risc0,
        ];
        // Just to make sure we are not missing any verifier. The compilation will fail if we do and it forces us to add it to the vec above.
        for verifier in verifiers.iter() {
            match verifier {
                ProvingSystemId::SP1 => (),
                ProvingSystemId::Risc0 => (),
                ProvingSystemId::GnarkPlonkBls12_381 => (),
                ProvingSystemId::GnarkPlonkBn254 => (),
                ProvingSystemId::Groth16Bn254 => (),
            }
        }
        verifiers
    }

    #[test]
    fn test_all_verifiers_enabled() {
        let disabled_verifiers = ethers::types::U256::zero();
        for verifier in get_all_verifiers().iter() {
            let verification_data = VerificationData {
                proving_system: *verifier,
                vm_program_code: None,
                pub_input: None,
                proof: vec![],
                verification_key: None,
                proof_generator_addr: Address::zero(),
            };
            assert!(
                !is_verifier_disabled(disabled_verifiers, verification_data.proving_system),
                "Verifier {:?} should not be disabled",
                verifier
            );
        }
    }

    #[test]
    fn test_all_verifiers_disabled() {
        let verifiers = get_all_verifiers();
        // This creates a number with all bits set to 1 depending on the number of verifiers to disable all of them.
        let disabled_verifiers = ethers::types::U256::from(2u64.pow(verifiers.len() as u32) - 1);
        for verifier in get_all_verifiers().iter() {
            let verification_data = VerificationData {
                proving_system: *verifier,
                vm_program_code: None,
                pub_input: None,
                proof: vec![],
                verification_key: None,
                proof_generator_addr: Address::zero(),
            };
            assert!(
                is_verifier_disabled(disabled_verifiers, verification_data.proving_system),
                "Verifier {:?} should be disabled",
                verifier
            );
        }
    }

    #[test]
    fn test_some_verifiers_disabled() {
        let verifiers = get_all_verifiers();
        // Disabling only the first verifier
        let disabled_verifiers = ethers::types::U256::from(0b10001);
        for verifier in get_all_verifiers().iter() {
            let verification_data = VerificationData {
                proving_system: *verifier,
                vm_program_code: None,
                pub_input: None,
                proof: vec![],
                verification_key: None,
                proof_generator_addr: Address::zero(),
            };
            if verifier == &verifiers[0] || verifier == &verifiers[verifiers.len() - 1] {
                assert!(
                    is_verifier_disabled(disabled_verifiers, verification_data.proving_system),
                    "Verifier {:?} should be disabled",
                    verifier
                );
            } else {
                assert!(
                    !is_verifier_disabled(disabled_verifiers, verification_data.proving_system),
                    "Verifier {:?} should not be disabled",
                    verifier
                );
            }
        }
    }
}
