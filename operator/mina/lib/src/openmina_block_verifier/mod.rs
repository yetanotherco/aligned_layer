use mina_p2p_messages::{binprot::BinProtRead as _, v2::StateHash};
use protocol_state::ProtocolStateQuery;

pub mod protocol_state;

pub fn parse_query_to_mina_block_header(
    mina_protocol_state_query: &str,
    _mina_state_proof_vk_query: &str,
) {
    let protocol_state_query: ProtocolStateQuery =
        serde_json::from_str(mina_protocol_state_query).unwrap();
    let protocol_state = &protocol_state_query.data.best_chain[0].protocol_state;
    let previous_state_hash =
        StateHash::binprot_read(&mut protocol_state.previous_state_hash.as_bytes()).unwrap();
    assert_eq!(
        previous_state_hash.to_string(),
        protocol_state.previous_state_hash
    );
}

#[cfg(test)]
mod test {
    use super::parse_query_to_mina_block_header;

    const MINA_PROTOCOL_STATE_QUERY: &str = include_str!(
        "../../../../../batcher/aligned/test_files/mina/mina_protocol_state_query.json"
    );

    const MINA_STATE_PROOF_VK_QUERY: &str = include_str!(
        "../../../../../batcher/aligned/test_files/mina/mina_state_proof_vk_query.json"
    );

    #[test]
    fn test_parse_query_to_mina_block_header() {
        parse_query_to_mina_block_header(MINA_PROTOCOL_STATE_QUERY, MINA_STATE_PROOF_VK_QUERY);
    }
}
