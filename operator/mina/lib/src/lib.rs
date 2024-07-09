use kimchi::mina_curves::pasta::Fp;
use mina_tree::proofs::verification::verify_block;
use mina_tree::proofs::verifier_index::{get_verifier_index, VerifierKind};
use mina_tree::verifier::get_srs;
use openmina_block_verifier::protocol_state;
use openmina_block_verifier::protocol_state_proof;

pub mod openmina_block_verifier;

// TODO: check these
const MAX_PROOF_SIZE: usize = 15 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 1024;

#[no_mangle]
pub extern "C" fn verify_protocol_state_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    public_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    public_input_len: usize,
) -> bool {
    let protocol_state_proof_base64 =
        if let Ok(protocol_state_proof_base64) = std::str::from_utf8(&proof_bytes[..proof_len]) {
            protocol_state_proof_base64
        } else {
            return false;
        };
    let protocol_state_hash_base58 = if let Ok(protocol_state_hash_base58) =
        std::str::from_utf8(&public_input_bytes[..public_input_len - 1])
    {
        protocol_state_hash_base58
    } else {
        return false;
    };

    let protocol_state_proof = if let Ok(protocol_state_proof) =
        protocol_state_proof::parse_base64(protocol_state_proof_base64)
    {
        protocol_state_proof
    } else {
        return false;
    };
    let protocol_state_hash =
        if let Ok(protocol_state_hash) = protocol_state::parse_base58(protocol_state_hash_base58) {
            protocol_state_hash
        } else {
            return false;
        };

    let verifier_index = get_verifier_index(VerifierKind::Blockchain);
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    verify_block(
        &protocol_state_proof,
        protocol_state_hash,
        &verifier_index,
        &srs,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const PROTOCOL_STATE_PROOF: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_proof.proof");
    const PROTOCOL_STATE_HASH: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_hash.pub");

    #[test]
    fn protocol_state_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROTOCOL_STATE_PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROTOCOL_STATE_PROOF);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PROTOCOL_STATE_HASH.len();
        pub_input_buffer[..pub_input_size].clone_from_slice(PROTOCOL_STATE_HASH);

        let result = verify_protocol_state_proof_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(result);
    }
}
