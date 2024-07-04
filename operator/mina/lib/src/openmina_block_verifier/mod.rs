mod protocol_state_proof;

pub fn parse_query_to_mina_block_header(mina_state_proof_vk_query: &str) {
    todo!()
}

#[cfg(test)]
mod test {
    use super::parse_query_to_mina_block_header;

    const MINA_STATE_PROOF_VK_QUERY: &str = include_str!(
        "../../../../../batcher/aligned/test_files/mina/mina_state_proof_vk_query.json"
    );

    #[test]
    fn test_parse_query_to_mina_block_header() {
        parse_query_to_mina_block_header(MINA_STATE_PROOF_VK_QUERY);
    }
}
