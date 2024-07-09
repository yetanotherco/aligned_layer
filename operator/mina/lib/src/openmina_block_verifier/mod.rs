use kimchi::mina_curves::pasta::Fp;
use mina_tree::{
    proofs::{
        verification::verify_block,
        verifier_index::{get_verifier_index, VerifierKind},
    },
    verifier::get_srs,
};
use protocol_state::parse_base58;
use protocol_state_proof::parse_base64;

mod protocol_state;
mod protocol_state_proof;

pub fn verify_protocol_state_proof(
    protocol_state_proof_base64: &str,
    protocol_state_hash_base58: &str,
) -> Result<bool, String> {
    let protocol_state_proof = parse_base64(protocol_state_proof_base64)?;
    let protocol_state_hash = parse_base58(protocol_state_hash_base58)?;

    let verifier_index = get_verifier_index(VerifierKind::Blockchain);
    let srs = get_srs::<Fp>();
    let srs = srs.lock().unwrap();

    Ok(verify_block(
        &protocol_state_proof,
        protocol_state_hash,
        &verifier_index,
        &srs,
    ))
}

#[cfg(test)]
mod test {
    use super::verify_protocol_state_proof;

    const PROTOCOL_STATE_PROOF: &str =
        include_str!("../../../../../batcher/aligned/test_files/mina/protocol_state_proof.proof");
    const PROTOCOL_STATE_HASH: &str =
        include_str!("../../../../../batcher/aligned/test_files/mina/protocol_state_hash.pub");

    #[test]
    fn test_verify_protocol_state_proof() {
        assert!(
            verify_protocol_state_proof(PROTOCOL_STATE_PROOF, PROTOCOL_STATE_HASH).unwrap(),
            "proof isn't valid!"
        );
    }
}
