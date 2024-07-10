use std::cmp::{max, min};

const SLOTS_PER_EPOCH: u32 = 7140;
const GRACE_PERIOD_END: u32 = 1440;
const SLOTS_PER_SUB_WINDOW: u32 = 7;
const SUB_WINDOWS_PER_WINDOW: u32 = 11;
// FIXME: retrieve this through archive node
const SUB_WINDOW_DENSITIES: [u32; 11] = [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7];

pub struct ConsensusState {
    pub slot_since_genesis: u32,
    pub epoch_count: u32,
    pub staking_epoch_data: EpochData,
    pub next_epoch_data: EpochData,
    pub min_window_density: u32,
}

pub struct EpochData {
    pub lock_checkpoint: String,
}

impl ConsensusState {
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
            let shift_count = min(
                max(max_slot - self.slot_since_genesis - 1, 0),
                SUB_WINDOWS_PER_WINDOW,
            );

            // Initialize projected window
            // FIXME: retrieve this through archive node
            let mut projected_window = SUB_WINDOW_DENSITIES;

            // Ring-shift
            let i = self.relative_sub_window() as usize;
            while shift_count > 0 {
                i = (i + 1) % SUB_WINDOWS_PER_WINDOW as usize;
                projected_window[i] = 0;
                shift_count -= 1;
            }

            projected_window
        };

        // Compute projected window density
        let projected_window_density = projected_window.iter().reduce(|acc, w| acc + w).unwrap();

        // Compute minimum window density
        return min(self.min_window_density, *projected_window_density);
    }

    fn relative_sub_window(&self) -> u32 {
        (self.slot_since_genesis / SLOTS_PER_SUB_WINDOW) % SUB_WINDOWS_PER_WINDOW
    }
}
