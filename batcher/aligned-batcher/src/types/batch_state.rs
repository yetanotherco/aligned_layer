use std::collections::HashMap;

use super::{
    batch_queue::{BatchQueue, BatchQueueEntry},
    user_state::UserState,
};
use ethers::types::{Address, U256};
use log::debug;
use tokio::sync::Mutex;

pub(crate) struct BatchState {
    pub(crate) batch_queue: BatchQueue,
    pub(crate) user_states: HashMap<Address, Mutex<UserState>>,
}

impl BatchState {
    pub(crate) fn new() -> Self {
        Self {
            batch_queue: BatchQueue::new(),
            user_states: HashMap::new(),
        }
    }

    pub(crate) fn new_with_user_states(user_states: HashMap<Address, Mutex<UserState>>) -> Self {
        Self {
            batch_queue: BatchQueue::new(),
            user_states,
        }
    }

    pub(crate) fn get_entry(&self, sender: Address, nonce: U256) -> Option<&BatchQueueEntry> {
        self.batch_queue
            .iter()
            .map(|(entry, _)| entry)
            .find(|entry| entry.sender == sender && entry.nonced_verification_data.nonce == nonce)
    }

    pub(crate) fn get_user_state(&self, addr: &Address) -> Option<&Mutex<UserState>> {
        self.user_states.get(addr)
    }

    pub(crate) async fn get_user_nonce(&self, addr: &Address) -> Option<U256> {
        let Some(user_state) = self.get_user_state(addr) else {
            return None;
        };
        user_state.lock().await.nonce
    }

    pub(crate) async fn get_user_min_fee(&self, addr: &Address) -> Option<U256> {
        let Some(user_state) = self.get_user_state(addr) else {
            return None;
        };
        Some(user_state.lock().await.min_fee)
    }

    pub(crate) async fn get_user_proof_count(&self, addr: &Address) -> Option<usize> {
        let Some(user_state) = self.get_user_state(addr) else {
            return None;
        };
        Some(user_state.lock().await.proofs_in_batch)
    }

    /// Checks if the entry is valid
    /// An entry is valid if there is no entry with the same sender, lower nonce and a lower fee
    pub(crate) fn replacement_entry_is_valid(
        &mut self,
        replacement_entry: &BatchQueueEntry,
    ) -> bool {
        let replacement_max_fee = replacement_entry.nonced_verification_data.max_fee;
        let nonce = replacement_entry.nonced_verification_data.nonce;
        let sender = replacement_entry.sender;

        debug!(
            "Checking validity of entry with sender: {:?}, nonce: {:?}, max_fee: {:?}",
            sender, nonce, replacement_max_fee
        );

        // it is a valid entry only if there is no entry with the same sender, lower nonce and a lower fee
        !self.batch_queue.iter().any(|(entry, _)| {
            entry.sender == sender
                && entry.nonced_verification_data.nonce < nonce
                && entry.nonced_verification_data.max_fee < replacement_max_fee
        })
    }

    pub(crate) fn get_user_min_fee_in_batch(&self, addr: &Address) -> U256 {
        self.batch_queue
            .iter()
            .filter(|(e, _)| &e.sender == addr)
            .map(|(e, _)| e.nonced_verification_data.max_fee)
            .min()
            .unwrap_or(U256::max_value())
    }

    pub(crate) fn get_user_proofs_in_batch_and_min_fee(&self) -> HashMap<Address, (usize, U256)> {
        let mut updated_user_states = HashMap::new();
        for (entry, _) in self.batch_queue.iter() {
            let addr = entry.sender;
            let user_min_fee = entry.nonced_verification_data.max_fee;

            let (proof_count, min_fee) =
                updated_user_states.entry(addr).or_insert((0, user_min_fee));

            *proof_count += 1;
            if entry.nonced_verification_data.max_fee < *min_fee {
                *min_fee = entry.nonced_verification_data.max_fee;
            }
        }

        updated_user_states
    }
}
