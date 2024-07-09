use base64::prelude::*;
use mina_p2p_messages::{binprot::BinProtRead, v2::MinaBaseProofStableV2};

pub fn parse_base64(protocol_state_proof_base64: &str) -> Result<MinaBaseProofStableV2, String> {
    let protocol_state_proof_binprot = BASE64_URL_SAFE
        .decode(protocol_state_proof_base64)
        .map_err(|err| err.to_string())?;

    MinaBaseProofStableV2::binprot_read(&mut protocol_state_proof_binprot.as_slice())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_base64;

    const PROTOCOL_STATE_PROOF: &str =
        include_str!("../../../../../batcher/aligned/test_files/mina/protocol_state_proof.proof");

    #[test]
    fn parse_base64_does_not_fail() {
        parse_base64(PROTOCOL_STATE_PROOF).unwrap();
    }
}
