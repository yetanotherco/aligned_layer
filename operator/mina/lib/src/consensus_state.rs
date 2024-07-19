use blake2::{Blake2b512, Digest};
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq)]
pub struct ConsensusState {
    pub block_height: u32,
    pub last_vrf_output: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsensusStateQuery {
    pub block_height: String,
    pub last_vrf_output: String,
}

impl From<ConsensusStateQuery> for ConsensusState {
    fn from(value: ConsensusStateQuery) -> Self {
        Self {
            block_height: u32::from_str_radix(&value.block_height, 10).unwrap(),
            last_vrf_output: value.last_vrf_output,
        }
    }
}

impl ConsensusState {
    pub fn from_json(mina_consensus_state_query: &str) -> Result<Self, String> {
        let mina_consensus_state_query: serde_json::Map<String, serde_json::Value> =
            serde_json::from_str(mina_consensus_state_query)
                .map_err(|err| format!("Could not parse mina state consensus query: {err}"))?;

        let consensus_state_query_value = mina_consensus_state_query
            .get("data")
            .and_then(|d| d.get("block"))
            .and_then(|d| d.get("protocolState"))
            .and_then(|d| d.get("consensusState"))
            .ok_or("Could not parse consensus state: JSON structure is unexpected")?
            .to_owned();

        let consensus_state_query: ConsensusStateQuery =
            serde_json::from_value(consensus_state_query_value)
                .map_err(|err| format!("Could not parse mina consensus state: {err}"))?;

        Ok(consensus_state_query.into())
    }

    pub fn select_longer_chain(&self, other: &Self) -> Self {
        if self.block_height < other.block_height {
            return other.clone();
        }
        // tiebreak logic
        else if self.block_height == other.block_height {
            // compare last VRF digests lexicographically
            if other.hash_last_vrf() > self.hash_last_vrf() {
                return other.clone();
            } else if self.hash_last_vrf() == self.hash_last_vrf() {
                // compare consensus state hashes lexicographically
                // if other.hash_state() > self.hash_state() {
                //     return other.clone();
                // }
                // FIXME: replace with logic defined above
                return other.clone();
            }
        }

        self.clone()
    }

    fn hash_last_vrf(&self) -> String {
        let mut hasher = Blake2b512::new();
        hasher.update(self.last_vrf_output.clone());
        let digest = hasher.finalize().to_vec();

        String::from_utf8(digest).unwrap()
    }

    fn hash_state(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::ConsensusState;

    const MINA_CONSENSUS_STATE_QUERY: &str = include_str!(
        "../../../../batcher/aligned/test_files/mina/mina_mainnet_protocol_query.json"
    );

    #[test]
    fn check_consensus_rules() {
        let consensus_state_query = ConsensusState::from_json(MINA_CONSENSUS_STATE_QUERY).unwrap();
        let consensus_state: ConsensusState = consensus_state_query.into();
        dbg!(consensus_state.block_height);
        let fake_chain_state = ConsensusState {
            block_height: 1,
            last_vrf_output: String::new(),
        };

        let best_chain = consensus_state.select_longer_chain(&fake_chain_state);

        assert_eq!(best_chain, consensus_state);
    }
}
