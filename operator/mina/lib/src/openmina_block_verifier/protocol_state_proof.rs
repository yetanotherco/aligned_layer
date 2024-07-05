use base64::prelude::*;
use mina_p2p_messages::{binprot::BinProtRead, v2::MinaBaseProofStableV2};

pub fn parse_base64(
    mina_protocol_state_proof_base64_query: &str,
) -> Result<MinaBaseProofStableV2, String> {
    let mina_state_proof_vk_query: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(mina_protocol_state_proof_base64_query)
            .map_err(|err| format!("Could not parse mina state proof vk query: {err}"))?;

    let protocol_state_proof_base64 = mina_state_proof_vk_query
            .get("data")
            .and_then(|d| d.get("bestChain"))
            .and_then(|d| d.get(0))
            .and_then(|d| d.get("protocolStateProof"))
            .and_then(|d| d.get("base64"))
            .ok_or("Could not parse protocol state proof: JSON structure upto protocolStateProof is unexpected")?.to_owned();

    let protocol_state_proof_base64: String =
        serde_json::from_value(protocol_state_proof_base64)
            .map_err(|err| format!("Could not parse mina state proof: {err}"))?;

    let protocol_state_proof_binprot = BASE64_URL_SAFE
        .decode(protocol_state_proof_base64)
        .map_err(|err| err.to_string())?;

    MinaBaseProofStableV2::binprot_read(&mut protocol_state_proof_binprot.as_slice())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_base64;

    const MINA_PROTOCOL_STATE_PROOF_BASE64_QUERY: &str = include_str!(
        "../../../../../batcher/aligned/test_files/mina/mina_protocol_state_proof_base64.json"
    );

    #[test]
    fn parse_protocol_state_proof() {
        parse_base64(MINA_PROTOCOL_STATE_PROOF_BASE64_QUERY).unwrap();
    }
}
