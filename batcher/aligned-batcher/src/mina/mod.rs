use base64::prelude::*;
use log::{debug, warn};

pub fn verify_protocol_state_proof_integrity(proof: &[u8], public_input: &[u8]) -> bool {
    debug!("Reading Mina protocol state proof base64");
    let protocol_state_proof_base64 =
        if let Ok(protocol_state_proof_base64) = std::str::from_utf8(&proof) {
            protocol_state_proof_base64
        } else {
            return false;
        };
    debug!("Reading Mina protocol state hash base58");
    let protocol_state_hash_base58 =
        if let Ok(protocol_state_hash_base58) = std::str::from_utf8(&public_input) {
            protocol_state_hash_base58
        } else {
            return false;
        };

    debug!("Decoding Mina protocol state proof base64");
    if BASE64_URL_SAFE
        .decode(protocol_state_proof_base64.trim_end())
        .is_err()
    {
        warn!("Failed to decode Mina protocol state proof base64");
        return false;
    }

    debug!("Decoding Mina protocol state hash base58");
    if bs58::decode(protocol_state_hash_base58.trim_end())
        .into_vec()
        .is_err()
    {
        warn!("Failed to decode Mina protocol state hash base58");
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use super::verify_protocol_state_proof_integrity;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_proof.proof");
    const PROTOCOL_STATE_HASH_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state_hash.pub");

    #[test]
    fn verify_protocol_state_proof_integrity_does_not_fail() {
        assert!(verify_protocol_state_proof_integrity(
            PROTOCOL_STATE_PROOF_BYTES,
            PROTOCOL_STATE_HASH_BYTES,
        ));
    }
}
