use crate::connection::send_message;
use crate::risc_zero::verify_risc_zero_proof;
use crate::sp1::verify_sp1_proof;
use crate::types::batch_queue::BatchQueue;
use crate::{gnark::verify_gnark, types::batch_queue::BatchQueueEntry};
use aligned_sdk::core::types::{
    ProofInvalidReason, ProvingSystemId, ValidityResponseMessage, VerificationData,
};
use ethers::types::U256;
use log::{debug, info, warn};
use tokio::sync::MutexGuard;

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
                    "Trying to verify Risc0 proof but image ID was not provided. Returning false"
                );
                return false;
            };
            let Some(pub_input) = &verification_data.pub_input else {
                warn!("Trying to verify Risc0 proof but public input was not provided. Returning false");
                return false;
            };

            let mut image_id = [0u8; 32];
            image_id.copy_from_slice(image_id_slice.as_slice());
            verify_risc_zero_proof(verification_data.proof.as_slice(), &image_id, pub_input)
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
    verification_data: &VerificationData,
) -> bool {
    disabled_verifiers & (U256::one() << verification_data.proving_system as u64) != U256::zero()
}

pub(crate) async fn filter_disabled_verifiers(
    batch_queue: BatchQueue,
    disabled_verifiers: MutexGuard<'_, U256>,
) -> BatchQueue {
    let mut removed_entries = Vec::new();
    let filtered_batch_queue = batch_queue
        .iter()
        .filter_map(|(entry, entry_priority)| {
            info!(
                "Verifying proof for proving system {}",
                entry
                    .nonced_verification_data
                    .verification_data
                    .proving_system
            );
            let verification_data = &entry.nonced_verification_data.verification_data;
            if !is_verifier_disabled(*disabled_verifiers, verification_data)
                && !removed_entries
                    .iter()
                    .any(|e: &BatchQueueEntry| e.sender == entry.sender)
            {
                Some((entry.clone(), entry_priority.clone()))
            } else {
                warn!(
                    "Verifier for proving system {} is now disabled, removing proofs from batch",
                    verification_data.proving_system
                );
                removed_entries.push(entry.clone());

                None
            }
        })
        .collect();
    for entry in removed_entries {
        let ws_sink = entry.messaging_sink.as_ref();
        if let Some(ws_sink) = ws_sink {
            send_message(
                ws_sink.clone(),
                ValidityResponseMessage::InvalidProof(ProofInvalidReason::DisabledVerifier(
                    entry
                        .nonced_verification_data
                        .verification_data
                        .proving_system,
                )),
            )
            .await;
        }
    }
    filtered_batch_queue
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
                !is_verifier_disabled(disabled_verifiers, &verification_data),
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
                is_verifier_disabled(disabled_verifiers, &verification_data),
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
                    is_verifier_disabled(disabled_verifiers, &verification_data),
                    "Verifier {:?} should be disabled",
                    verifier
                );
            } else {
                assert!(
                    !is_verifier_disabled(disabled_verifiers, &verification_data),
                    "Verifier {:?} should not be disabled",
                    verifier
                );
            }
        }
    }
}
