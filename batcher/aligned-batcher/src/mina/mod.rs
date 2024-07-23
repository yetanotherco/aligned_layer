use base64::prelude::*;
use log::{debug, warn};

pub fn verify_protocol_state_proof_integrity(proof: &[u8], public_input: &[u8]) -> bool {
    debug!("Checking Mina protocol state proof");
    if let Err(err) = check_protocol_state_proof(proof) {
        warn!("Protocol state proof check failed: {}", err);
        return false;
    }

    debug!("Checking Mina protocol state public inputs");
    if let Err(err) = check_protocol_state_pub(public_input) {
        warn!("Protocol state public inputs check failed: {}", err);
        return false;
    }

    true
}

pub fn check_protocol_state_proof(protocol_state_proof_bytes: &[u8]) -> Result<(), String> {
    // TODO(xqft): check binprot deserialization
    let protocol_state_proof_base64 =
        std::str::from_utf8(protocol_state_proof_bytes).map_err(|err| err.to_string())?;
    BASE64_URL_SAFE
        .decode(protocol_state_proof_base64)
        .map_err(|err| err.to_string())?;

    Ok(())
}

pub fn check_protocol_state_pub(protocol_state_pub: &[u8]) -> Result<(), String> {
    // TODO(xqft): check hash and binprot deserialization
    let protocol_state_base64 =
        std::str::from_utf8(&protocol_state_pub[32..]).map_err(|err| err.to_string())?;
    BASE64_STANDARD
        .decode(protocol_state_base64)
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::verify_protocol_state_proof_integrity;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.proof");
    const PROTOCOL_STATE_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.pub");

    #[test]
    fn verify_protocol_state_proof_integrity_does_not_fail() {
        assert!(verify_protocol_state_proof_integrity(
            PROTOCOL_STATE_PROOF_BYTES,
            PROTOCOL_STATE_PUB_BYTES,
        ));
    }
}
