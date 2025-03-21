/// Consensus chain selection algorithms. The [`official specification`] was taken as a reference.
///
/// [`official specification`]: https://github.com/MinaProtocol/mina/blob/develop/docs/specs/consensus/README.md
mod consensus_state;
mod verifier_index;

use log::error;
use mina_bridge_core::proof::state_proof::{MinaStateProof, MinaStatePubInputs};

use ark_ec::short_weierstrass_jacobian::GroupAffine;
use consensus_state::{select_secure_chain, ChainResult};
use kimchi::mina_curves::pasta::{Fp, PallasParameters};
use kimchi::verifier_index::VerifierIndex;
use lazy_static::lazy_static;
use mina_curves::pasta::{Fq, Vesta};
use mina_p2p_messages::hash::MinaHash;
use mina_p2p_messages::v2::{MinaStateProtocolStateValueStableV2, StateHash};
use mina_tree::proofs::field::FieldWitness as _;
use mina_tree::proofs::verification::verify_block;
use poly_commitment::srs::SRS;
use verifier_index::{deserialize_blockchain_vk, MinaChain};

lazy_static! {
    static ref DEVNET_VERIFIER_INDEX: VerifierIndex<GroupAffine<PallasParameters>> =
        deserialize_blockchain_vk(MinaChain::Devnet).unwrap_or_else(|err| {
            error!("Failed to load Devnet verification key: {}", err);
            std::process::exit(1);
        });
    static ref MAINNET_VERIFIER_INDEX: VerifierIndex<GroupAffine<PallasParameters>> =
        deserialize_blockchain_vk(MinaChain::Mainnet).unwrap_or_else(|err| {
            error!("Failed to load Mainnet verification key: {}", err);
            std::process::exit(1);
        });
    static ref MINA_SRS: SRS<Vesta> = SRS::<Vesta>::create(Fq::SRS_DEPTH);
}

#[no_mangle]
pub extern "C" fn verify_mina_state_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    pub_input_bytes: *const u8,
    pub_input_len: u32,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        inner_verify_mina_state_ffi(proof_bytes, proof_len, pub_input_bytes, pub_input_len)
    });

    match result {
        Ok(v) => v as i32,
        Err(_) => -1,
    }
}

fn inner_verify_mina_state_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    pub_input_bytes: *const u8,
    pub_input_len: u32,
) -> bool {
    if proof_bytes.is_null() || pub_input_bytes.is_null() {
        error!("Input buffer null");
        return false;
    }

    if proof_len == 0 || pub_input_len == 0 {
        error!("Input buffer length zero size");
        return false;
    }

    let proof_bytes = unsafe { std::slice::from_raw_parts(proof_bytes, proof_len as usize) };

    let pub_input_bytes =
        unsafe { std::slice::from_raw_parts(pub_input_bytes, pub_input_len as usize) };

    verify_mina_state(proof_bytes, pub_input_bytes)
}

/// Verifies that the Mina state included in `proof_bytes` is valid.
/// This includes checking that:
///
/// - The Mina state corresponds to the tip of the most secure chain
/// - The corresponding Pickles proof is valid
pub fn verify_mina_state(proof_bytes: &[u8], pub_input_bytes: &[u8]) -> bool {
    let proof: MinaStateProof = match bincode::deserialize(proof_bytes) {
        Ok(proof) => proof,
        Err(err) => {
            error!("Failed to deserialize state proof: {}", err);
            return false;
        }
    };
    let pub_inputs: MinaStatePubInputs = match bincode::deserialize(pub_input_bytes) {
        Ok(pub_inputs) => pub_inputs,
        Err(err) => {
            error!("Failed to deserialize state pub inputs: {}", err);
            return false;
        }
    };

    // Checks the integrity of the public inputs, also checks if the states form a chain.
    let (candidate_tip_state, bridge_tip_state, candidate_tip_state_hash) =
        match check_pub_inputs(&proof, &pub_inputs) {
            Ok(validated_data) => validated_data,
            Err(err) => {
                error!("Failed to check pub inputs: {err}");
                return false;
            }
        };

    // Consensus checks
    let secure_chain = match select_secure_chain(&candidate_tip_state, &bridge_tip_state) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed consensus checks for candidate tip: {err}");
            return false;
        }
    };
    if secure_chain == ChainResult::Bridge {
        error!("Failed consensus checks for candidate tip: bridge's tip is more secure");
        return false;
    }

    // Verify the tip block (and thanks to Pickles recursion all the previous states are verified
    // as well)
    if pub_inputs.is_state_proof_from_devnet {
        verify_block(
            &proof.candidate_tip_proof,
            candidate_tip_state_hash,
            &DEVNET_VERIFIER_INDEX,
            &MINA_SRS,
        )
    } else {
        verify_block(
            &proof.candidate_tip_proof,
            candidate_tip_state_hash,
            &MAINNET_VERIFIER_INDEX,
            &MINA_SRS,
        )
    }
}

/// Checks public inputs against the proof data, making sure the inputs correspond to the proofs
/// we're verifying. Returns validated data for executing the rest of the verification steps.
fn check_pub_inputs(
    proof: &MinaStateProof,
    pub_inputs: &MinaStatePubInputs,
) -> Result<
    (
        MinaStateProtocolStateValueStableV2,
        MinaStateProtocolStateValueStableV2,
        Fp,
    ),
    String,
> {
    let candidate_root_state_hash = proof
        .candidate_chain_states
        .first()
        .map(|state| state.hash())
        .ok_or("failed to retrieve root state hash".to_string())?;
    // Reconstructs the state hashes if the states form a chain, and compares them to the public
    // input state hashes. Does not compare the tip state hash.
    let mut state_hash = candidate_root_state_hash;
    for (body_hash, expected_prev_state_hash) in proof
        .candidate_chain_states
        .iter()
        .skip(1)
        .map(|state| state.body.hash())
        .zip(pub_inputs.candidate_chain_state_hashes.iter())
    {
        let curr_state_hash = StateHash::from_hashes(&state_hash, &body_hash);
        let prev_state_hash = std::mem::replace(&mut state_hash, curr_state_hash);

        // Check if all hashes (but the last one) in the public input are correct
        if &prev_state_hash != expected_prev_state_hash {
            return Err("public input state hashes do not match the states to verify, or states don't form a chain".to_string());
        }
    }

    // Check if the tip hash (the last one) is correct, so we also verify the Merkle list
    if &state_hash
        != pub_inputs
            .candidate_chain_state_hashes
            .last()
            .ok_or("failed to retrieve tip state hash".to_string())?
    {
        return Err("public input tip state hash is not correct".to_string());
    }

    // Validate the public input ledger hashes
    let expected_candidate_chain_ledger_hashes = proof.candidate_chain_states.iter().map(|state| {
        &state
            .body
            .blockchain_state
            .ledger_proof_statement
            .target
            .first_pass_ledger
    });
    if pub_inputs
        .candidate_chain_ledger_hashes
        .iter()
        .ne(expected_candidate_chain_ledger_hashes)
    {
        return Err(
            "candidate chain ledger hashes on public inputs don't match the ones on the states to verify"
                .to_string(),
        );
    }

    // Validate the public input bridge's tip state hash
    let bridge_tip_state_hash = pub_inputs
        .bridge_tip_state_hash
        .to_fp()
        .map_err(|err| format!("Can't parse bridge tip state hash to fp: {err}"))?;

    if MinaHash::hash(&proof.bridge_tip_state) != bridge_tip_state_hash {
        return Err(
            "the candidate's chain tip state doesn't match the hash provided as public input"
                .to_string(),
        );
    }

    let candidate_tip_state = proof
        .candidate_chain_states
        .last()
        .ok_or("failed to get candidate tip state from proof".to_string())?
        .clone();
    let bridge_tip_state = proof.bridge_tip_state.clone();

    let candidate_tip_state_hash = pub_inputs
        .candidate_chain_state_hashes
        .last()
        .ok_or("failed to get candidate tip hash from public inputs".to_string())
        .and_then(|hash| {
            hash.to_fp()
                .map_err(|err| format!("failed to convert tip state hash to field element: {err}"))
        })?;

    Ok((
        candidate_tip_state,
        bridge_tip_state,
        candidate_tip_state_hash,
    ))
}

#[cfg(test)]
mod test {
    use std::ptr;

    use super::*;

    const DEVNET_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/devnet_mina_state.proof");
    const DEVNET_PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/devnet_mina_state.pub");
    const MAINNET_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/mainnet_mina_state.proof");
    const MAINNET_PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/mainnet_mina_state.pub");
    const BAD_HASH_PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/mina_state_bad_hash.pub");

    #[test]
    fn valid_devnet_mina_state_proof_verifies() {
        let result = verify_mina_state_ffi(
            DEVNET_PROOF_BYTES.as_ptr(),
            DEVNET_PROOF_BYTES.len() as u32,
            DEVNET_PUB_INPUT_BYTES.as_ptr(),
            DEVNET_PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn valid_mainnet_mina_state_proof_verifies() {
        let result = verify_mina_state_ffi(
            MAINNET_PROOF_BYTES.as_ptr(),
            MAINNET_PROOF_BYTES.len() as u32,
            MAINNET_PUB_INPUT_BYTES.as_ptr(),
            MAINNET_PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn mina_state_proof_with_bad_bridge_tip_hash_does_not_verify() {
        let result = verify_mina_state_ffi(
            DEVNET_PROOF_BYTES.as_ptr(),
            DEVNET_PROOF_BYTES.len() as u32,
            BAD_HASH_PUB_INPUT_BYTES.as_ptr(),
            BAD_HASH_PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn zeroized_mina_state_proof_does_not_verify() {
        const PROOF_SIZE: usize = DEVNET_PROOF_BYTES.len();
        let empty_proof_buffer = [0u8; PROOF_SIZE];

        let result = verify_mina_state_ffi(
            empty_proof_buffer.as_ptr(),
            PROOF_SIZE as u32,
            DEVNET_PUB_INPUT_BYTES.as_ptr(),
            DEVNET_PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_mina_state_proof_with_zeroized_pub_input_does_not_verify() {
        const PUB_INPUT_SIZE: usize = DEVNET_PUB_INPUT_BYTES.len();
        let empty_pub_input_buffer = [0u8; PUB_INPUT_SIZE];

        let result = verify_mina_state_ffi(
            DEVNET_PROOF_BYTES.as_ptr(),
            DEVNET_PROOF_BYTES.len() as u32,
            empty_pub_input_buffer.as_ptr(),
            PUB_INPUT_SIZE as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn null_mina_state_proof_does_not_verify() {
        let result = verify_mina_state_ffi(
            ptr::null(),
            DEVNET_PROOF_BYTES.len() as u32,
            DEVNET_PUB_INPUT_BYTES.as_ptr(),
            DEVNET_PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_mina_state_proof_with_null_pub_input_does_not_verify() {
        let result = verify_mina_state_ffi(
            DEVNET_PROOF_BYTES.as_ptr(),
            DEVNET_PROOF_BYTES.len() as u32,
            ptr::null(),
            DEVNET_PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn empty_mina_state_proof_does_not_verify() {
        let result = verify_mina_state_ffi(
            DEVNET_PROOF_BYTES.as_ptr(),
            0,
            DEVNET_PUB_INPUT_BYTES.as_ptr(),
            DEVNET_PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_mina_state_proof_with_empty_pub_input_does_not_verify() {
        let result = verify_mina_state_ffi(
            DEVNET_PROOF_BYTES.as_ptr(),
            DEVNET_PROOF_BYTES.len() as u32,
            DEVNET_PUB_INPUT_BYTES.as_ptr(),
            0,
        );
        assert_eq!(result, 0);
    }
}
