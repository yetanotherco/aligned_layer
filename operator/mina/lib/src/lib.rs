mod consensus_state;

use core::proof::state_proof::{MinaStateProof, MinaStatePubInputs};

use ark_ec::short_weierstrass_jacobian::GroupAffine;
use consensus_state::{select_longer_chain, LongerChainResult};
use kimchi::mina_curves::pasta::{Fp, PallasParameters};
use kimchi::verifier_index::VerifierIndex;
use lazy_static::lazy_static;
use mina_p2p_messages::hash::MinaHash;
use mina_p2p_messages::v2::{MinaStateProtocolStateValueStableV2, StateHash};
use mina_tree::proofs::verification::verify_block;
use mina_tree::verifier::get_srs;
use verifier_index::deserialize_blockchain_vk;

mod verifier_index;

lazy_static! {
    static ref VERIFIER_INDEX: VerifierIndex<GroupAffine<PallasParameters>> =
        deserialize_blockchain_vk().unwrap();
}

// TODO(xqft): check proof size
const MAX_PROOF_SIZE: usize = 48 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;

#[no_mangle]
pub extern "C" fn verify_protocol_state_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    pub_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    pub_input_len: usize,
) -> bool {
    let proof: MinaStateProof = match bincode::deserialize(&proof_bytes[..proof_len]) {
        Ok(proof) => proof,
        Err(err) => {
            eprintln!("Failed to deserialize state proof: {}", err);
            return false;
        }
    };
    let pub_inputs: MinaStatePubInputs =
        match bincode::deserialize(&pub_input_bytes[..pub_input_len]) {
            Ok(pub_inputs) => pub_inputs,
            Err(err) => {
                eprintln!("Failed to deserialize state pub inputs: {}", err);
                return false;
            }
        };

    // TODO(xqft): this can also be a batcher's pre-verification check
    let (candidate_tip_state, bridge_tip_state, candidate_tip_state_hash) =
        match check_pub_inputs(&proof, &pub_inputs) {
            Ok(validated_data) => validated_data,
            Err(err) => {
                eprintln!("Failed to check pub inputs: {err}");
                return false;
            }
        };

    // TODO(xqft): srs should be a static, but can't make it so because it doesn't have all its
    // parameters initialized.
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    // Consensus check: Short fork rule
    let longer_chain = select_longer_chain(&candidate_tip_state, &bridge_tip_state);
    if longer_chain == LongerChainResult::Bridge {
        eprintln!("Failed consensus checks for candidate tip state against bridge's tip");
        return false;
    }

    // Verify the tip block (and thanks to Pickles recursion all the previous states are verified
    // as well)
    verify_block(
        &proof.candidate_tip_proof,
        candidate_tip_state_hash,
        &VERIFIER_INDEX,
        &srs,
    )
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
    // Reconstructs the state hashes if the states form a chain. The iterator will be in the
    // reversed order of the public inputs one.
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
            .staged_ledger_hash
            .non_snark
            .ledger_hash
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
    use super::*;

    const PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.proof");
    const PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.pub");
    const PROTOCOL_STATE_BAD_HASH_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_bad_hash.pub");
    const PROTOCOL_STATE_BAD_CONSENSUS_PUB_BYTES: &[u8] = include_bytes!(
        "../../../../batcher/aligned/test_files/mina/protocol_state_bad_consensus.pub"
    );

    #[test]
    fn protocol_state_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(result);
    }

    #[test]
    fn proof_of_protocol_state_with_bad_hash_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PROTOCOL_STATE_BAD_HASH_PUB_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PROTOCOL_STATE_BAD_HASH_PUB_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }

    #[test]
    fn proof_of_protocol_state_with_bad_consensus_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PROTOCOL_STATE_BAD_CONSENSUS_PUB_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PROTOCOL_STATE_BAD_CONSENSUS_PUB_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }
}
