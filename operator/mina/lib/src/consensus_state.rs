use blake2::{Blake2b512, Digest};
use kimchi::o1_utils::FieldHelpers;
use mina_p2p_messages::{
    hash::MinaHash,
    v2::{
        ConsensusProofOfStakeDataConsensusStateValueStableV2 as MinaConsensusState,
        MinaStateProtocolStateValueStableV2 as MinaProtocolState,
    },
};
use std::cmp::{max, min, Ordering};

const GRACE_PERIOD_END: u32 = 1440;
const SUB_WINDOWS_PER_WINDOW: u32 = 11;
const SLOTS_PER_SUB_WINDOW: u32 = 7;

#[derive(Debug, PartialEq)]
pub enum ChainResult {
    Bridge,
    Candidate,
}

pub fn select_secure_chain(
    candidate: &MinaProtocolState,
    tip: &MinaProtocolState,
) -> Result<ChainResult, String> {
    if is_short_range(candidate, tip)? {
        Ok(select_longer_chain(candidate, tip))
    } else {
        let tip_density = relative_min_window_density(candidate, tip);
        let candidate_density = relative_min_window_density(candidate, tip);
        Ok(match candidate_density.cmp(&tip_density) {
            Ordering::Less => ChainResult::Bridge,
            Ordering::Equal => select_longer_chain(candidate, tip),
            Ordering::Greater => ChainResult::Candidate,
        })
    }
}

fn select_longer_chain(candidate: &MinaProtocolState, tip: &MinaProtocolState) -> ChainResult {
    let candidate_block_height = &candidate.body.consensus_state.blockchain_length.as_u32();
    let tip_block_height = &tip.body.consensus_state.blockchain_length.as_u32();

    if candidate_block_height > tip_block_height {
        return ChainResult::Candidate;
    }
    // tiebreak logic
    else if candidate_block_height == tip_block_height {
        // compare last VRF digests lexicographically
        if hash_last_vrf(candidate) > hash_last_vrf(tip) {
            return ChainResult::Candidate;
        } else if hash_last_vrf(candidate) == hash_last_vrf(tip) {
            // compare consensus state hashes lexicographically
            if hash_state(candidate) > hash_state(tip) {
                return ChainResult::Candidate;
            }
        }
    }

    ChainResult::Bridge
}

/// Returns true if the fork is short-range, else the fork is long-range.
fn is_short_range(candidate: &MinaProtocolState, tip: &MinaProtocolState) -> Result<bool, String> {
    // TODO(xqft): verify constants are correct
    if tip.body.constants != candidate.body.constants {
        return Err("Protocol constants on candidate and tip state are not equal".to_string());
    }
    let slots_per_epoch = tip.body.constants.slots_per_epoch.as_u32();

    let candidate = &candidate.body.consensus_state;
    let tip = &tip.body.consensus_state;

    let check = |s1: &MinaConsensusState, s2: &MinaConsensusState| {
        let s2_epoch_slot = s2.global_slot() % slots_per_epoch;
        if s1.epoch_count.as_u32() == s2.epoch_count.as_u32() + 1
            && s2_epoch_slot >= slots_per_epoch * 2 / 3
        {
            s1.staking_epoch_data.lock_checkpoint == s2.next_epoch_data.lock_checkpoint
        } else {
            false
        }
    };

    Ok(if candidate.epoch_count == tip.epoch_count {
        candidate.staking_epoch_data.lock_checkpoint == tip.staking_epoch_data.lock_checkpoint
    } else {
        check(candidate, tip) || check(tip, candidate)
    })
}

fn relative_min_window_density(candidate: &MinaProtocolState, tip: &MinaProtocolState) -> u32 {
    let candidate = &candidate.body.consensus_state;
    let tip = &tip.body.consensus_state;

    let max_slot = max(candidate.global_slot(), tip.global_slot());

    if max_slot < GRACE_PERIOD_END {
        return candidate.min_window_density.as_u32();
    }

    // Added input checks to compute `shift_count` and `i` safely.
    // We don't take into account how to compute consensus checks in those border cases since
    // we assume those cases happen when candidate and tip states aren't taken from a Mina node
    // but built to perform a DoS attack.
    let projected_window = {
        let shift_count = max_slot
            .checked_sub(candidate.global_slot())
            .and_then(|result| result.checked_sub(1))
            .unwrap_or(0)
            .clamp(0, SUB_WINDOWS_PER_WINDOW);
        let mut projected_window: Vec<_> = candidate
            .sub_window_densities
            .iter()
            .map(|d| d.as_u32())
            .collect();

        let mut i = relative_sub_window(candidate);
        for _ in 0..shift_count {
            i = (i + 1) % SUB_WINDOWS_PER_WINDOW;
            if let Some(projected_window_i) = projected_window.get_mut(i as usize) {
                *projected_window_i = 0
            }
        }

        projected_window
    };

    let projected_window_density = projected_window.iter().sum();

    min(
        candidate.min_window_density.as_u32(),
        projected_window_density,
    )
}

fn relative_sub_window(state: &MinaConsensusState) -> u32 {
    (state.global_slot() / SLOTS_PER_SUB_WINDOW) % SUB_WINDOWS_PER_WINDOW
}

fn hash_last_vrf(chain: &MinaProtocolState) -> String {
    let mut hasher = Blake2b512::new();
    hasher.update(chain.body.consensus_state.last_vrf_output.as_slice());
    let digest = hasher.finalize().to_vec();

    hex::encode(digest)
}

fn hash_state(chain: &MinaProtocolState) -> String {
    MinaHash::hash(chain).to_hex()
}

#[cfg(test)]
mod test {
    use ark_ff::Fp256;
    use mina_bridge_core::proof::state_proof::MinaStateProof;
    use mina_p2p_messages::{
        bigint::BigInt,
        number::Number,
        v2::{
            DataHashLibStateHashStableV1, MinaNumbersGlobalSlotSinceHardForkMStableV1,
            UnsignedExtendedUInt32StableV1,
        },
    };

    use super::*;

    const PROOF_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina/mina_state.proof");

    #[test]
    fn new_mina_state_passes_consensus_checks() {
        let valid_proof: MinaStateProof = bincode::deserialize(PROOF_BYTES).unwrap();
        let old_tip = valid_proof.bridge_tip_state;
        let new_tip = valid_proof.candidate_chain_states.last().unwrap();

        assert_eq!(
            select_secure_chain(new_tip, &old_tip).unwrap(),
            ChainResult::Candidate
        );
    }

    #[test]
    fn old_mina_state_fails_consensus_checks() {
        let valid_proof: MinaStateProof = bincode::deserialize(PROOF_BYTES).unwrap();
        let old_tip = valid_proof.bridge_tip_state;
        let new_tip = valid_proof.candidate_chain_states.last().unwrap();

        assert_eq!(
            select_secure_chain(&old_tip, new_tip).unwrap(),
            ChainResult::Bridge
        );
    }

    #[test]
    fn test_candidate_state_with_smaller_global_slot_than_tip_state() {
        let valid_proof: MinaStateProof = bincode::deserialize(PROOF_BYTES).unwrap();
        let new_tip = valid_proof.bridge_tip_state;
        let mut old_tip = valid_proof.candidate_chain_states.last().unwrap().clone();

        // Force checking long fork rule:
        // Set both `epoch_count` to be the same but set new `lock_checkpoint` of older one to be one less than newer one
        old_tip.body.consensus_state.epoch_count = new_tip.body.consensus_state.epoch_count;
        old_tip
            .body
            .consensus_state
            .staking_epoch_data
            .lock_checkpoint = DataHashLibStateHashStableV1(BigInt::from(
            new_tip
                .body
                .consensus_state
                .staking_epoch_data
                .lock_checkpoint
                .to_fp()
                .unwrap()
                - Fp256::from(1),
        ))
        .into();
        old_tip.body.consensus_state.sub_window_densities.pop_back();
        old_tip
            .body
            .consensus_state
            .sub_window_densities
            .push_back(UnsignedExtendedUInt32StableV1(Number(1)));

        // Set new `global_slot` to be `SUB_WINDOWS_PER_WINDOW + 2` less than older one
        old_tip
            .body
            .consensus_state
            .curr_global_slot_since_hard_fork
            .slot_number = MinaNumbersGlobalSlotSinceHardForkMStableV1::SinceHardFork(
            UnsignedExtendedUInt32StableV1(Number(
                new_tip
                    .body
                    .consensus_state
                    .curr_global_slot_since_hard_fork
                    .slot_number
                    .as_u32()
                    - (SUB_WINDOWS_PER_WINDOW + 2),
            )),
        );

        assert_eq!(
            select_secure_chain(&new_tip, &old_tip).unwrap(),
            ChainResult::Bridge
        );
    }

    #[test]
    fn test_candidate_state_with_less_sub_windows_densities_than_sub_windows_per_window() {
        let valid_proof: MinaStateProof = bincode::deserialize(PROOF_BYTES).unwrap();
        let mut new_tip = valid_proof.bridge_tip_state;
        let mut old_tip = valid_proof.candidate_chain_states.last().unwrap().clone();

        // Force checking long fork rule:
        // Set both `epoch_count` to be the same but set new `lock_checkpoint` of older one to be one less than newer one
        old_tip.body.consensus_state.epoch_count = new_tip.body.consensus_state.epoch_count;
        old_tip
            .body
            .consensus_state
            .staking_epoch_data
            .lock_checkpoint = DataHashLibStateHashStableV1(BigInt::from(
            new_tip
                .body
                .consensus_state
                .staking_epoch_data
                .lock_checkpoint
                .to_fp()
                .unwrap()
                - Fp256::from(1),
        ))
        .into();
        old_tip.body.consensus_state.sub_window_densities.pop_back();
        old_tip
            .body
            .consensus_state
            .sub_window_densities
            .push_back(UnsignedExtendedUInt32StableV1(Number(1)));

        // Set new `global_slot` to be `SUB_WINDOWS_PER_WINDOW` less than newer one
        old_tip
            .body
            .consensus_state
            .curr_global_slot_since_hard_fork
            .slot_number = MinaNumbersGlobalSlotSinceHardForkMStableV1::SinceHardFork(
            UnsignedExtendedUInt32StableV1(Number(
                new_tip
                    .body
                    .consensus_state
                    .curr_global_slot_since_hard_fork
                    .slot_number
                    .as_u32()
                    + SUB_WINDOWS_PER_WINDOW,
            )),
        );

        new_tip.body.consensus_state.sub_window_densities.pop_back();
        new_tip.body.consensus_state.sub_window_densities.pop_back();

        assert_eq!(
            select_secure_chain(&new_tip, &old_tip).unwrap(),
            ChainResult::Bridge
        );
    }
}
