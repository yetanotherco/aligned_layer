/// Consensus chain selection algorithms. The [`official specification`] was taken as a reference.
///
/// [`official specification`]: https://github.com/MinaProtocol/mina/blob/develop/docs/specs/consensus/README.md
mod consensus_state;
mod verifier_index;

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
        deserialize_blockchain_vk(MinaChain::Devnet).unwrap();
    static ref MAINNET_VERIFIER_INDEX: VerifierIndex<GroupAffine<PallasParameters>> =
        deserialize_blockchain_vk(MinaChain::Mainnet).unwrap();
    static ref MINA_SRS: SRS<Vesta> = SRS::<Vesta>::create(Fq::SRS_DEPTH);
}

// TODO(xqft): check proof size
const MAX_PROOF_SIZE: usize = 48 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;

#[no_mangle]
pub extern "C" fn verify_mina_state_ffi(
    proof_buffer: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    pub_input_buffer: &[u8; MAX_PUB_INPUT_SIZE],
    pub_input_len: usize,
) -> bool {
    let Some(proof_buffer_slice) = proof_buffer.get(..proof_len) else {
        eprintln!("Proof length argument is greater than max proof size");
        return false;
    };

    let Some(pub_input_buffer_slice) = pub_input_buffer.get(..pub_input_len) else {
        eprintln!("Public input length argument is greater than max public input size");
        return false;
    };

    let proof: MinaStateProof = match bincode::deserialize(proof_buffer_slice) {
        Ok(proof) => proof,
        Err(err) => {
            eprintln!("Failed to deserialize state proof: {}", err);
            return false;
        }
    };
    let pub_inputs: MinaStatePubInputs = match bincode::deserialize(pub_input_buffer_slice) {
        Ok(pub_inputs) => pub_inputs,
        Err(err) => {
            eprintln!("Failed to deserialize state pub inputs: {}", err);
            return false;
        }
    };

    // Checks the integrity of the public inputs, also checks if the states form a chain.
    let (candidate_tip_state, bridge_tip_state, candidate_tip_state_hash) =
        match check_pub_inputs(&proof, &pub_inputs) {
            Ok(validated_data) => validated_data,
            Err(err) => {
                eprintln!("Failed to check pub inputs: {err}");
                return false;
            }
        };

    // Consensus checks
    let secure_chain = match select_secure_chain(&candidate_tip_state, &bridge_tip_state) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Failed consensus checks for candidate tip: {err}");
            return false;
        }
    };
    if secure_chain == ChainResult::Bridge {
        eprintln!("Failed consensus checks for candidate tip: bridge's tip is more secure");
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
    use super::*;

    const PROOF_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/mina_state.proof");
    const PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/mina_state.pub");
    const BAD_HASH_PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/mina_state_bad_hash.pub");

    #[test]
    fn valid_mina_state_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result =
            verify_mina_state_ffi(&proof_buffer, proof_size, &pub_input_buffer, pub_input_size);
        assert!(result);
    }

    #[test]
    fn mina_state_proof_with_bad_bridge_tip_hash_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = BAD_HASH_PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(BAD_HASH_PUB_INPUT_BYTES);

        let result =
            verify_mina_state_ffi(&proof_buffer, proof_size, &pub_input_buffer, pub_input_size);
        assert!(!result);
    }

    #[test]
    fn empty_mina_state_proof_does_not_verify() {
        let proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result =
            verify_mina_state_ffi(&proof_buffer, proof_size, &pub_input_buffer, pub_input_size);
        assert!(!result);
    }

    #[test]
    fn valid_mina_state_proof_with_empty_pub_input_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();

        let result =
            verify_mina_state_ffi(&proof_buffer, proof_size, &pub_input_buffer, pub_input_size);
        assert!(!result);
    }

    #[test]
    fn valid_mina_state_proof_with_greater_proof_size_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let wrong_proof_size = super::MAX_PROOF_SIZE + 1;
        proof_buffer[..PROOF_BYTES.len()].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_mina_state_ffi(
            &proof_buffer,
            wrong_proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }

    #[test]
    fn valid_mina_state_proof_with_greater_pub_input_size_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let wrong_pub_input_size = MAX_PUB_INPUT_SIZE + 1;
        pub_input_buffer[..PUB_INPUT_BYTES.len()].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_mina_state_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            wrong_pub_input_size,
        );
        assert!(!result);
    }
}
