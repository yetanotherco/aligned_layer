use std::str::FromStr;

use kimchi::mina_curves::pasta::Fp;
use mina_p2p_messages::v2::StateHash;

pub fn parse_base58(mina_protocol_state_hash_query: &str) -> Result<Fp, String> {
    let mina_protocol_state_hash_query: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(mina_protocol_state_hash_query)
            .map_err(|err| format!("Could not parse mina state proof vk query: {err}"))?;

    let protocol_state_hash_query = mina_protocol_state_hash_query
        .get("data")
        .and_then(|d| d.get("bestChain"))
        .and_then(|d| d.get(0))
        .and_then(|d| d.get("protocolState"))
        .and_then(|d| d.get("previousStateHash"))
        .ok_or("Could not parse previous protocol state hash: JSON structure is unexpected")?
        .to_owned();

    let protocol_state_hash_base58: String = serde_json::from_value(protocol_state_hash_query)
        .map_err(|err| format!("Could not parse mina state proof: {err}"))?;

    StateHash::from_str(&protocol_state_hash_base58)
        .map_err(|err| err.to_string())?
        .to_fp()
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_base58;

    const MINA_PROTOCOL_STATE_PROOF_BASE64_QUERY: &str = include_str!(
        "../../../../../batcher/aligned/test_files/mina/mina_devnet_protocol_query.json"
    );

    #[test]
    fn parse_protocol_state() {
        parse_base58(MINA_PROTOCOL_STATE_PROOF_BASE64_QUERY).unwrap();
    }
}
