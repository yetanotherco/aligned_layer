use std::str::FromStr;

use kimchi::mina_curves::pasta::Fp;
use mina_p2p_messages::v2::StateHash;

pub fn parse_base58(protocol_state_hash_base58: &str) -> Result<Fp, String> {
    StateHash::from_str(&protocol_state_hash_base58)
        .map_err(|err| err.to_string())?
        .to_fp()
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_base58;

    const PROTOCOL_STATE_HASH: &str =
        include_str!("../../../../../batcher/aligned/test_files/mina/protocol_state_hash.pub");

    #[test]
    fn parse_base58_does_not_fail() {
        parse_base58(PROTOCOL_STATE_HASH).unwrap();
    }
}
