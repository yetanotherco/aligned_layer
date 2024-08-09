use std::array::TryFromSliceError;

use base64::prelude::*;
use log::{debug, warn};

const STATE_HASH_SIZE: usize = 32;

pub fn verify_proof_integrity(proof: &[u8], public_input: &[u8]) -> bool {
    debug!("Checking Mina protocol state proof");
    if let Err(err) = check_proof(proof) {
        warn!("Protocol state proof check failed: {}", err);
        return false;
    }

    debug!("Checking Mina protocol state public inputs");
    if let Err(err) = check_pub_inputs(public_input) {
        warn!("Protocol state public inputs check failed: {}", err);
        return false;
    }

    true
}

pub fn check_hash(pub_inputs: &[u8], offset: &mut usize) -> Result<(), String> {
    pub_inputs
        .get(*offset..*offset + STATE_HASH_SIZE)
        .ok_or("Failed to slice candidate hash".to_string())?;

    *offset += STATE_HASH_SIZE;

    Ok(())
}

pub fn check_state(pub_inputs: &[u8], offset: &mut usize) -> Result<(), String> {
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

    pub_inputs
        .get(*offset + 4..*offset + 4 + state_len)
        .ok_or("Failed to slice state".to_string())
        .and_then(|bytes| std::str::from_utf8(bytes).map_err(|err| err.to_string()))
        .and_then(|base64| {
            BASE64_STANDARD
                .decode(base64)
                .map_err(|err| err.to_string())
        })?;
    *offset += 4 + state_len;

    Ok(())
}

pub fn check_pub_inputs(pub_inputs: &[u8]) -> Result<(), String> {
    let mut offset = 0;

    check_hash(pub_inputs, &mut offset)?; // candidate hash
    check_hash(pub_inputs, &mut offset)?; // tip hash

    check_state(pub_inputs, &mut offset)?; // candidate state
    check_state(pub_inputs, &mut offset)?; // tip state

    Ok(())
}

pub fn check_proof(proof_bytes: &[u8]) -> Result<(), String> {
    std::str::from_utf8(proof_bytes)
        .map_err(|err| err.to_string())
        .and_then(|base64| {
            BASE64_URL_SAFE
                .decode(base64)
                .map_err(|err| err.to_string())
        })?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::verify_proof_integrity;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.proof");
    const PROTOCOL_STATE_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.pub");

    #[test]
    fn verify_proof_integrity_does_not_fail() {
        assert!(verify_proof_integrity(
            PROTOCOL_STATE_PROOF_BYTES,
            PROTOCOL_STATE_PUB_BYTES,
        ));
    }
}
