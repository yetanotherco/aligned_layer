use std::collections::{hash_map::Entry, HashMap};

use super::{
    batch_queue::{BatchQueue, BatchQueueEntry},
    user_state::UserState,
};
use ethers::types::{Address, U256};
use log::debug;

pub(crate) struct BatchState {
    pub(crate) batch_queue: BatchQueue,
    pub(crate) user_states: HashMap<Address, UserState>,
}

impl BatchState {
    pub(crate) fn new() -> Self {
        Self {
            batch_queue: BatchQueue::new(),
            user_states: HashMap::new(),
        }
    }

    pub(crate) fn new_with_user_states(user_states: HashMap<Address, UserState>) -> Self {
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

    pub(crate) fn get_user_state(&self, addr: &Address) -> Option<&UserState> {
        self.user_states.get(addr)
    }

    pub(crate) async fn get_user_nonce(&self, addr: &Address) -> Option<U256> {
        let user_state = self.get_user_state(addr)?;
        Some(user_state.nonce)
    }

    pub(crate) async fn get_user_min_fee(&self, addr: &Address) -> Option<U256> {
        let user_state = self.get_user_state(addr)?;
        Some(user_state.min_fee)
    }

    pub(crate) fn update_user_nonce(&mut self, addr: &Address, new_nonce: U256) -> Option<U256> {
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().nonce = new_nonce;
            return Some(new_nonce);
        }
        None
    }

    pub(crate) async fn get_user_proof_count(&self, addr: &Address) -> Option<usize> {
        let user_state = self.get_user_state(addr)?;
        Some(user_state.proofs_in_batch)
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

    pub(crate) fn update_user_min_fee(
        &mut self,
        addr: &Address,
        new_min_fee: U256,
    ) -> Option<U256> {
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().min_fee = new_min_fee;
            return Some(new_min_fee);
        }
        None
    }

    pub(crate) fn update_user_proof_count(
        &mut self,
        addr: &Address,
        new_proof_count: usize,
    ) -> Option<usize> {
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().proofs_in_batch = new_proof_count;
            return Some(new_proof_count);
        }
        None
    }

    /// Updates the user with address `addr` with the provided values of `new_nonce`, `new_min_fee` and
    /// `new_proof_count`.
    /// If state is updated successfully, returns the updated values inside a `Some()`
    /// If the address was not found in the user states, returns `None`
    pub(crate) fn update_user_state(
        &mut self,
        addr: &Address,
        new_nonce: U256,
        new_min_fee: U256,
        new_proof_count: usize,
    ) -> Option<(U256, U256, usize)> {
        let updated_nonce = self.update_user_nonce(addr, new_nonce);
        let updated_min_fee = self.update_user_min_fee(addr, new_min_fee);
        let updated_proof_count = self.update_user_proof_count(addr, new_proof_count);

        if updated_nonce.is_some() && updated_min_fee.is_some() && updated_proof_count.is_some() {
            return Some((new_nonce, new_min_fee, new_proof_count));
        }
        None
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
