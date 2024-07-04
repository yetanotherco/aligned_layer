use mina_p2p_messages::{
    binprot::BinProtRead,
    v2::{MinaStateProtocolStateValueStableV2, StateHash},
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolStateQuery {
    pub data: Data,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub best_chain: [BestChain; 1],
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BestChain {
    pub protocol_state: ProtocolState,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolState {
    pub previous_state_hash: String,
}

impl Into<MinaStateProtocolStateValueStableV2> for ProtocolState {
    fn into(self) -> MinaStateProtocolStateValueStableV2 {
        let previous_state_hash =
            StateHash::binprot_read(&mut self.previous_state_hash.as_bytes()).unwrap();

        MinaStateProtocolStateValueStableV2 {
            previous_state_hash,
            body: todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_previous_state_hash() {}
}
