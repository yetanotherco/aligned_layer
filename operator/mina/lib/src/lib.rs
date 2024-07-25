mod consensus_state;

use std::array;

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
    let protocol_state_proof = match parse_protocol_state_proof(&proof_bytes[..proof_len]) {
        Ok(protocol_state_proof) => protocol_state_proof,
        Err(err) => {
            eprintln!("Failed to parse protocol state proof: {}", err);
            return false;
        }
    };

    let (
        candidate_protocol_state_hash,
        candidate_protocol_state,
        tip_protocol_state_hash,
        tip_protocol_state,
    ) = match parse_protocol_state_pub(&public_input_bytes[..public_input_len]) {
        Ok(protocol_state_pub) => protocol_state_pub,
        Err(err) => {
            eprintln!("Failed to parse protocol state public inputs: {}", err);
            return false;
        }
    };

    // TODO(xqft): this can be a batcher's pre-verification check (but don't remove it from here)
    if MinaHash::hash(&tip_protocol_state) != tip_protocol_state_hash {
        eprintln!("The tip's protocol state doesn't match the hash provided as public input");
        return false;
    }
    if MinaHash::hash(&candidate_protocol_state) != candidate_protocol_state_hash {
        eprintln!("The candidate's protocol state doesn't match the hash provided as public input");
        return false;
    }

    // TODO(xqft): srs should be a static, but can't make it so because it doesn't have all its
    // parameters initialized.
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    // Consensus check: Short fork rule
    let longer_chain = select_longer_chain(&candidate_protocol_state, &tip_protocol_state);
    if longer_chain == LongerChainResult::Tip {
        eprintln!("Consensus check failed");
        return false;
    }

    // Pickles verification
    verify_block(
        &protocol_state_proof,
        candidate_protocol_state_hash,
        &VERIFIER_INDEX,
        &srs,
    )
}

pub fn parse_protocol_state_proof(
    protocol_state_proof_bytes: &[u8],
) -> Result<MinaBaseProofStableV2, String> {
    let protocol_state_proof_base64 =
        std::str::from_utf8(protocol_state_proof_bytes).map_err(|err| err.to_string())?;
    let protocol_state_proof_binprot = BASE64_URL_SAFE
        .decode(protocol_state_proof_base64)
        .map_err(|err| err.to_string())?;
    MinaBaseProofStableV2::binprot_read(&mut protocol_state_proof_binprot.as_slice())
        .map_err(|err| err.to_string())
}

pub fn parse_protocol_state_pub(
    protocol_state_pub: &[u8],
) -> Result<
    (
        Fp,
        MinaStateProtocolStateValueStableV2,
        Fp,
        MinaStateProtocolStateValueStableV2,
    ),
    String,
> {
    let (tip_protocol_state_hash, tip_protocol_state, candidate_start) =
        parse_protocol_state_with_hash(&protocol_state_pub, 0)?;

    let (candidate_protocol_state_hash, candidate_protocol_state, _) =
        parse_protocol_state_with_hash(&protocol_state_pub, candidate_start)?;

    Ok((
        tip_protocol_state_hash,
        tip_protocol_state,
        candidate_protocol_state_hash,
        candidate_protocol_state,
    ))
}

fn parse_protocol_state_with_hash(
    protocol_state_pub: &[u8],
    start: usize,
) -> Result<
    (
        ark_ff::Fp256<mina_curves::pasta::fields::FpParameters>,
        MinaStateProtocolStateValueStableV2,
        usize,
    ),
    String,
> {
    let protocol_state_hash_bytes: Vec<_> = protocol_state_pub
        .iter()
        .skip(start)
        .take(STATE_HASH_SIZE)
        .map(|byte| byte.clone())
        .collect();
    let protocol_state_hash =
        Fp::from_bytes(&protocol_state_hash_bytes).map_err(|err| err.to_string())?;

    let protocol_state_len_vec: Vec<_> = protocol_state_pub
        .iter()
        .skip(start + STATE_HASH_SIZE)
        .take(8)
        .collect();
    let protocol_state_len_bytes: [u8; 4] = array::from_fn(|i| protocol_state_len_vec[i].clone());
    let protocol_state_len = u32::from_be_bytes(protocol_state_len_bytes) as usize;

    let protocol_state_bytes: Vec<_> = protocol_state_pub
        .iter()
        .skip(start + STATE_HASH_SIZE + 4)
        .take(protocol_state_len)
        .map(|byte| byte.clone())
        .collect();
    let protocol_state_base64 =
        std::str::from_utf8(&protocol_state_bytes).map_err(|err| err.to_string())?;
    let protocol_state_binprot = BASE64_STANDARD
        .decode(protocol_state_base64)
        .map_err(|err| err.to_string())?;
    let protocol_state =
        MinaStateProtocolStateValueStableV2::binprot_read(&mut protocol_state_binprot.as_slice())
            .map_err(|err| err.to_string())?;

    Ok((
        protocol_state_hash,
        protocol_state,
        start + STATE_HASH_SIZE + 4 + protocol_state_len,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.proof");
    const PROTOCOL_STATE_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.pub");
    const PROTOCOL_STATE_BAD_HASH_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_bad_hash.pub");
    const PROTOCOL_STATE_BAD_CONSENSUS_PUB_BYTES: &[u8] = include_bytes!(
        "../../../../batcher/aligned/test_files/mina/protocol_state_bad_consensus.pub"
    );

    #[test]
    fn parse_protocol_state_proof_does_not_fail() {
        parse_protocol_state_proof(PROTOCOL_STATE_PROOF_BYTES).unwrap();
    }

    #[test]
    fn parse_protocol_state_pub_does_not_fail() {
        parse_protocol_state_pub(PROTOCOL_STATE_PUB_BYTES).unwrap();
    }

    #[test]
    fn protocol_state_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PROTOCOL_STATE_PUB_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PROTOCOL_STATE_PUB_BYTES);

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
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

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
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

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
