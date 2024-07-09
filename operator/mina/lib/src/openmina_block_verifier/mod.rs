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

pub fn verify_protocol_state_proof(mina_protocol_query: &str) -> Result<bool, String> {
    let protocol_state_proof = parse_base64(mina_protocol_query)?;
    let protocol_state_hash = parse_base58(mina_protocol_query)?;
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

    const MINA_PROTOCOL_STATE_PROOF_BASE64_QUERY: &str = include_str!(
        "../../../../../batcher/aligned/test_files/mina/mina_devnet_protocol_query.json"
    );

    #[test]
    fn test_verify_protocol_state_proof() {
        assert!(
            verify_protocol_state_proof(MINA_PROTOCOL_STATE_PROOF_BASE64_QUERY).unwrap(),
            "proof isn't valid!"
        );
    }
}
