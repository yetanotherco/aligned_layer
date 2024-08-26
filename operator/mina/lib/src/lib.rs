mod consensus_state;

use std::array::TryFromSliceError;

use ark_ec::short_weierstrass_jacobian::GroupAffine;
use base64::prelude::*;
use consensus_state::{select_longer_chain, LongerChainResult};
use kimchi::mina_curves::pasta::{Fp, PallasParameters};
use kimchi::o1_utils::FieldHelpers;
use kimchi::verifier_index::VerifierIndex;
use lazy_static::lazy_static;
use mina_p2p_messages::binprot::BinProtRead;
use mina_p2p_messages::hash::MinaHash;
use mina_p2p_messages::v2::{MinaBaseProofStableV2, MinaStateProtocolStateValueStableV2};
use mina_tree::proofs::verification::verify_block;
use mina_tree::verifier::get_srs;
use verifier_index::deserialize_blockchain_vk;

mod verifier_index;

lazy_static! {
    static ref VERIFIER_INDEX: VerifierIndex<GroupAffine<PallasParameters>> =
        deserialize_blockchain_vk().unwrap();
}

// TODO(xqft): check proof size
const MAX_PROOF_SIZE: usize = 16 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;
const STATE_HASH_SIZE: usize = 32;

#[no_mangle]
pub extern "C" fn verify_protocol_state_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    public_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    public_input_len: usize,
) -> bool {
    let protocol_state_proof = match parse_proof(&proof_bytes[..proof_len]) {
        Ok(protocol_state_proof) => protocol_state_proof,
        Err(err) => {
            eprintln!("Failed to parse protocol state proof: {}", err);
            return false;
        }
    };

    let (candidate_ledger_hash, candidate_hash, tip_hash, candidate_state, tip_state) =
        match parse_pub_inputs(&public_input_bytes[..public_input_len]) {
            Ok(protocol_state_pub) => protocol_state_pub,
            Err(err) => {
                eprintln!("Failed to parse protocol state public inputs: {}", err);
                return false;
            }
        };

    let expected_candidate_ledger_hash = match candidate_state
        .body
        .blockchain_state
        .staged_ledger_hash
        .non_snark
        .ledger_hash
        .to_fp()
    {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Failed to parse candidate ledger hash: {}", err);
            return false;
        }
    };

    // TODO(xqft): this can be a batcher's pre-verification check (but don't remove it from here)
    if candidate_ledger_hash != expected_candidate_ledger_hash {
        eprintln!("Candidate ledger hash on public inputs doesn't match the encoded state's one");
        return false;
    }

    // TODO(xqft): this can be a batcher's pre-verification check (but don't remove it from here)
    if MinaHash::hash(&tip_state) != tip_hash {
        eprintln!("The tip's protocol state doesn't match the hash provided as public input");
        return false;
    }
    // TODO(xqft): this can be a batcher's pre-verification check (but don't remove it from here)
    if MinaHash::hash(&candidate_state) != candidate_hash {
        eprintln!("The candidate's protocol state doesn't match the hash provided as public input");
        return false;
    }

    // TODO(xqft): srs should be a static, but can't make it so because it doesn't have all its
    // parameters initialized.
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    // Consensus check: Short fork rule
    let longer_chain = select_longer_chain(&candidate_state, &tip_state);
    if longer_chain == LongerChainResult::Tip {
        eprintln!("Consensus check failed");
        return false;
    }

    // Pickles verification
    verify_block(&protocol_state_proof, candidate_hash, &VERIFIER_INDEX, &srs)
}

pub fn parse_hash(pub_inputs: &[u8], offset: &mut usize) -> Result<Fp, String> {
    let hash = pub_inputs
        .get(*offset..*offset + STATE_HASH_SIZE)
        .ok_or("Failed to slice candidate hash".to_string())
        .and_then(|bytes| Fp::from_bytes(bytes).map_err(|err| err.to_string()))?;

    *offset += STATE_HASH_SIZE;

    Ok(hash)
}

pub fn parse_state(
    pub_inputs: &[u8],
    offset: &mut usize,
) -> Result<MinaStateProtocolStateValueStableV2, String> {
    let state_len: usize = pub_inputs
        .get(*offset..*offset + 4)
        .ok_or("Failed to slice state len".to_string())
        .and_then(|slice| {
            slice
                .try_into()
                .map_err(|err: TryFromSliceError| err.to_string())
        })
        .map(u32::from_be_bytes)
        .and_then(|len| usize::try_from(len).map_err(|err| err.to_string()))?;

    let state = pub_inputs
        .get(*offset + 4..*offset + 4 + state_len)
        .ok_or("Failed to slice state".to_string())
        .and_then(|bytes| std::str::from_utf8(bytes).map_err(|err| err.to_string()))
        .and_then(|base64| {
            BASE64_STANDARD
                .decode(base64)
                .map_err(|err| err.to_string())
        })
        .and_then(|binprot| {
            MinaStateProtocolStateValueStableV2::binprot_read(&mut binprot.as_slice())
                .map_err(|err| err.to_string())
        })?;

    *offset += 4 + state_len;

    Ok(state)
}

pub fn parse_pub_inputs(
    pub_inputs: &[u8],
) -> Result<
    (
        Fp,
        Fp,
        Fp,
        MinaStateProtocolStateValueStableV2,
        MinaStateProtocolStateValueStableV2,
    ),
    String,
> {
    let mut offset = 0;

    let candidate_ledger_hash = parse_hash(pub_inputs, &mut offset)?;

    let candidate_hash = parse_hash(pub_inputs, &mut offset)?;
    let tip_hash = parse_hash(pub_inputs, &mut offset)?;

    let candidate_state = parse_state(pub_inputs, &mut offset)?;
    let tip_state = parse_state(pub_inputs, &mut offset)?;

    Ok((
        candidate_ledger_hash,
        candidate_hash,
        tip_hash,
        candidate_state,
        tip_state,
    ))
}

pub fn parse_proof(proof_bytes: &[u8]) -> Result<MinaBaseProofStableV2, String> {
    std::str::from_utf8(proof_bytes)
        .map_err(|err| err.to_string())
        .and_then(|base64| {
            BASE64_URL_SAFE
                .decode(base64)
                .map_err(|err| err.to_string())
        })
        .and_then(|binprot| {
            MinaBaseProofStableV2::binprot_read(&mut binprot.as_slice())
                .map_err(|err| err.to_string())
        })
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
    fn parse_protocol_state_proof_does_not_fail() {
        parse_proof(PROOF_BYTES).unwrap();
    }

    #[test]
    fn parse_protocol_state_pub_does_not_fail() {
        parse_pub_inputs(PUB_INPUT_BYTES).unwrap();
    }

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
