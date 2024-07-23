use std::str::FromStr;

use ark_ec::short_weierstrass_jacobian::GroupAffine;
use base64::prelude::*;
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
const MAX_PUB_INPUT_SIZE: usize = 3 * 1024;

#[no_mangle]
pub extern "C" fn verify_protocol_state_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    public_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    public_input_len: usize,
) -> bool {
    // TODO(xqft): add message errors

    let protocol_state_proof_base64 =
        if let Ok(protocol_state_proof_base64) = std::str::from_utf8(&proof_bytes[..proof_len]) {
            protocol_state_proof_base64
        } else {
            return false;
        };

    let protocol_state_hash =
        if let Ok(protocol_state_hash) = Fp::from_bytes(&public_input_bytes[..32]) {
            protocol_state_hash
        } else {
            return false;
        };

    let protocol_state_base64 = if let Ok(protocol_state_base64) =
        std::str::from_utf8(&public_input_bytes[32..public_input_len])
    {
        protocol_state_base64
    } else {
        return false;
    };

    let protocol_state_proof =
        if let Ok(protocol_state_proof) = parse_protocol_state_proof(protocol_state_proof_base64) {
            protocol_state_proof
        } else {
            return false;
        };

    let protocol_state = if let Ok(protocol_state) = parse_protocol_state(protocol_state_base64) {
        protocol_state
    } else {
        return false;
    };

    println!("checking hash");
    // check that protocol state hash is correct
    // TODO(xqft): this can be a batcher's pre-verification check (but don't remove it from here)
    if MinaHash::hash(&protocol_state) != protocol_state_hash {
        return false;
    }
    println!("hash checked");

    // TODO(xqft): srs should be a static, but can't make it so because it doesn't have all its
    // parameters initialized.
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    verify_block(
        &protocol_state_proof,
        protocol_state_hash,
        &VERIFIER_INDEX,
        &srs,
    )
}

pub fn parse_protocol_state_proof(
    protocol_state_proof_base64: &str,
) -> Result<MinaBaseProofStableV2, String> {
    let protocol_state_proof_binprot = BASE64_URL_SAFE
        .decode(protocol_state_proof_base64.trim_matches(char::from(0)))
        .map_err(|err| err.to_string())?;

    MinaBaseProofStableV2::binprot_read(&mut protocol_state_proof_binprot.as_slice())
        .map_err(|err| err.to_string())
}

pub fn parse_protocol_state(
    protocol_state_base64: &str,
) -> Result<MinaStateProtocolStateValueStableV2, String> {
    let protocol_state_binprot = BASE64_STANDARD
        .decode(protocol_state_base64)
        .map_err(|err| err.to_string())?;

    MinaStateProtocolStateValueStableV2::binprot_read(&mut protocol_state_binprot.as_slice())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.proof");
    const PROTOCOL_STATE_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.pub");
    const BAD_PROTOCOL_STATE_HASH_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/bad_protocol_state_hash.pub");

    const PROTOCOL_STATE_PROOF_STR: &str =
        include_str!("../../../../batcher/aligned/test_files/mina/protocol_state_proof.proof");
    const PROTOCOL_STATE_HASH_STR: &str =
        include_str!("../../../../batcher/aligned/test_files/mina/protocol_state_hash.pub");

    #[test]
    fn parse_protocol_state_proof_does_not_fail() {
        parse_protocol_state_proof(PROTOCOL_STATE_PROOF_STR).unwrap();
    }

    // TODO(xqft): parse_protocol_state_does_not_fail()

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
    fn bad_protocol_state_proof_fails() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROTOCOL_STATE_PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = BAD_PROTOCOL_STATE_HASH_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(BAD_PROTOCOL_STATE_HASH_BYTES);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }
}
