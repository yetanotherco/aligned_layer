use std::cmp::{max, min};

use blake2::{Blake2b512, Digest};
use serde::Deserialize;

const SLOTS_PER_EPOCH: u32 = 7140;
const GRACE_PERIOD_END: u32 = 1440;
const SLOTS_PER_SUB_WINDOW: u32 = 7;
const SUB_WINDOWS_PER_WINDOW: u32 = 11;
// FIXME: retrieve this through archive node
const SUB_WINDOW_DENSITIES: [u32; 11] = [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7];

#[derive(Clone)]
pub struct ConsensusState {
    pub slot_since_genesis: u32,
    pub epoch_count: u32,
    pub staking_epoch_data: EpochData,
    pub next_epoch_data: EpochData,
    pub min_window_density: u32,
    pub block_height: u32,
    pub last_vrf_output: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpochData {
    pub lock_checkpoint: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsensusStateQuery {
    pub slot_since_genesis: String,
    pub epoch_count: String,
    pub staking_epoch_data: EpochData,
    pub next_epoch_data: EpochData,
    pub min_window_density: String,
    pub block_height: String,
    pub last_vrf_output: String,
}

impl From<ConsensusStateQuery> for ConsensusState {
    fn from(value: ConsensusStateQuery) -> Self {
        Self {
            slot_since_genesis: u32::from_str_radix(&value.slot_since_genesis, 10).unwrap(),
            epoch_count: u32::from_str_radix(&value.epoch_count, 10).unwrap(),
            staking_epoch_data: value.staking_epoch_data,
            next_epoch_data: value.next_epoch_data,
            min_window_density: u32::from_str_radix(&value.min_window_density, 10).unwrap(),
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
            .and_then(|d| d.get("bestChain"))
            .and_then(|d| d.get(0))
            .and_then(|d| d.get("protocolState"))
            .and_then(|d| d.get("consensusState"))
            .ok_or("Could not parse consensus state: JSON structure is unexpected")?
            .to_owned();

        dbg!(consensus_state_query_value.to_owned());

        let consensus_state_query: ConsensusStateQuery =
            serde_json::from_value(consensus_state_query_value)
                .map_err(|err| format!("Could not parse mina consensus state: {err}"))?;

        Ok(consensus_state_query.into())
    }

    pub fn select_secure_chain(&self, other: &Self) -> Self {
        if self.is_short_range(other) {
            self.select_longer_chain(other)
        } else {
            let tip_density = self.relative_min_window_density(other);
            let candidate_density = other.relative_min_window_density(self);
            if candidate_density > tip_density {
                other.clone()
            } else if candidate_density == tip_density {
                self.select_longer_chain(other)
            } else {
                self.clone()
            }
        }
    }

    fn is_short_range(&self, other: &Self) -> bool {
        if self.epoch_count == other.epoch_count {
            // Simple case: blocks have same previous epoch, so compare previous epochs' lock_checkpoints
            self.staking_epoch_data.lock_checkpoint == other.staking_epoch_data.lock_checkpoint
        } else {
            // Check for previous epoch case using both orientations
            self.check(other) || other.check(self)
        }
    }

    fn check(&self, other: &Self) -> bool {
        if self.epoch_count == other.epoch_count + 1
            && other.epoch_slot() >= 2 / 3 * SLOTS_PER_EPOCH
        {
            // S1 is one epoch ahead of S2 and S2 is not in the seed update range
            self.staking_epoch_data.lock_checkpoint == other.next_epoch_data.lock_checkpoint
        } else {
            false
        }
    }

    fn epoch_slot(&self) -> u32 {
        self.slot_since_genesis % SLOTS_PER_EPOCH
    }

    fn relative_min_window_density(&self, other: &Self) -> u32 {
        let max_slot = max(self.slot_since_genesis, other.slot_since_genesis);

        // Grace-period rule
        if max_slot < GRACE_PERIOD_END {
            return self.min_window_density;
        }

        // Compute B1's window projected to max_slot
        let projected_window = {
            // Compute shift count
            let mut shift_count = min(
                max(max_slot - self.slot_since_genesis - 1, 0),
                SUB_WINDOWS_PER_WINDOW,
            );

            // Initialize projected window
            // FIXME: retrieve this through archive node
            let mut projected_window = SUB_WINDOW_DENSITIES;

            // Ring-shift
            let mut i = self.relative_sub_window() as usize;
            while shift_count > 0 {
                i = (i + 1) % SUB_WINDOWS_PER_WINDOW as usize;
                projected_window[i] = 0;
                shift_count -= 1;
            }

            projected_window
        };

        // Compute projected window density
        let projected_window_density = projected_window.iter().fold(0, |acc, w| acc + w);

        // Compute minimum window density
        return min(self.min_window_density, projected_window_density);
    }

    fn relative_sub_window(&self) -> u32 {
        (self.slot_since_genesis / SLOTS_PER_SUB_WINDOW) % SUB_WINDOWS_PER_WINDOW
    }

    fn select_longer_chain(&self, other: &Self) -> Self {
        if self.block_height < other.block_height {
            return other.clone();
        }
        // tiebreak logic
        else if self.block_height == self.block_height {
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
        "../../../../../batcher/aligned/test_files/mina/mina_devnet_protocol_query.json"
    );

    #[test]
    fn test_parse_consensus_state() {
        ConsensusState::from_json(MINA_CONSENSUS_STATE_QUERY).unwrap();
    }
}
