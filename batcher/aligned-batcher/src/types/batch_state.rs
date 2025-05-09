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
    pub(crate) max_size: usize,
}

impl BatchState {
    // CONSTRUCTORS:

    pub(crate) fn new(max_size: usize) -> Self {
        Self {
            batch_queue: BatchQueue::new(),
            user_states: HashMap::new(),
            max_size,
        }
    }

    pub(crate) fn new_with_user_states(
        user_states: HashMap<Address, UserState>,
        max_size: usize,
    ) -> Self {
        Self {
            batch_queue: BatchQueue::new(),
            user_states,
            max_size,
        }
    }

    // GETTERS:

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

    pub(crate) async fn get_user_last_max_fee_limit(&self, addr: &Address) -> Option<U256> {
        let user_state = self.get_user_state(addr)?;
        Some(user_state.last_max_fee_limit)
    }

    pub(crate) async fn get_user_total_fees_in_queue(&self, addr: &Address) -> Option<U256> {
        let user_state = self.get_user_state(addr)?;
        Some(user_state.total_fees_in_queue)
    }

    pub(crate) async fn get_user_proof_count(&self, addr: &Address) -> Option<usize> {
        let user_state = self.get_user_state(addr)?;
        Some(user_state.proofs_in_batch)
    }

    pub(crate) fn get_user_min_fee_in_batch(&self, addr: &Address) -> U256 {
        self.batch_queue
            .iter()
            .filter(|(e, _)| &e.sender == addr)
            .map(|(e, _)| e.nonced_verification_data.max_fee)
            .min()
            .unwrap_or(U256::max_value())
    }

    // SETTERS:

    pub(crate) fn update_user_max_fee_limit(
        &mut self,
        addr: &Address,
        new_max_fee_limit: U256,
    ) -> Option<U256> {
        // TODO refactor to return Result, or something less redundant
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().last_max_fee_limit = new_max_fee_limit;
            return Some(new_max_fee_limit);
        }
        None
    }

    pub(crate) fn update_user_proof_count(
        &mut self,
        addr: &Address,
        new_proof_count: usize,
    ) -> Option<usize> {
        // TODO refactor to return Result, or something less redundant
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().proofs_in_batch = new_proof_count;
            return Some(new_proof_count);
        }
        None
    }

    pub(crate) fn update_user_nonce(&mut self, addr: &Address, new_nonce: U256) -> Option<U256> {
        // TODO refactor to return Result, or something less redundant
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().nonce = new_nonce;
            return Some(new_nonce);
        }
        None
    }

    pub(crate) fn update_user_total_fees_in_queue(
        &mut self,
        addr: &Address,
        new_total_fees_in_queue: U256,
    ) -> Option<U256> {
        // TODO refactor to return Result, or something less redundant
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().total_fees_in_queue = new_total_fees_in_queue;
            return Some(new_total_fees_in_queue);
        }
        None
    }

    pub(crate) fn update_user_total_fees_in_queue_of_replacement_message(
        &mut self,
        addr: &Address,
        original_max_fee: U256,
        new_max_fee: U256,
    ) -> Option<U256> {
        // TODO refactor to return Result, or something less redundant
        let fee_difference = new_max_fee - original_max_fee; //here we already know new_max_fee > original_max_fee
        if let Entry::Occupied(mut user_state) = self.user_states.entry(*addr) {
            user_state.get_mut().total_fees_in_queue += fee_difference;
            return Some(user_state.get().total_fees_in_queue);
        }
        None
    }

    /// Updates the user with address `addr` with the provided values of
    /// `new_nonce`, `new_max_fee_limit`, `new_proof_count` and `new_total_fees_in_queue`
    /// If state is updated successfully, returns the updated values inside a `Some()`
    /// If the address was not found in the user states, returns `None`
    pub(crate) fn update_user_state(
        &mut self,
        addr: &Address,
        new_nonce: U256,
        new_max_fee_limit: U256,
        new_proof_count: usize,
        new_total_fees_in_queue: U256,
    ) -> Option<(U256, U256, usize, U256)> {
        // TODO refactor to return Result, or something less redundant
        let updated_nonce = self.update_user_nonce(addr, new_nonce);
        let updated_max_fee_limit = self.update_user_max_fee_limit(addr, new_max_fee_limit);
        let updated_proof_count = self.update_user_proof_count(addr, new_proof_count);
        let updated_total_fees_in_queue =
            self.update_user_total_fees_in_queue(addr, new_total_fees_in_queue);

        if updated_nonce.is_some()
            && updated_max_fee_limit.is_some()
            && updated_proof_count.is_some()
            && updated_total_fees_in_queue.is_some()
        {
            return Some((
                new_nonce,
                new_max_fee_limit,
                new_proof_count,
                new_total_fees_in_queue,
            ));
        }
        None
    }

    // LOGIC:

    pub(crate) fn calculate_new_user_states_data(&self) -> HashMap<Address, (usize, U256, U256)> {
        let mut updated_user_states = HashMap::new(); // address -> (proof_count, max_fee_limit, total_fees_in_queue)
        for (entry, _) in self.batch_queue.iter() {
            let addr = entry.sender;
            let max_fee = entry.nonced_verification_data.max_fee;

            let (proof_count, max_fee_limit, total_fees_in_queue) = updated_user_states
                .entry(addr)
                .or_insert((0, max_fee, U256::zero()));

            *proof_count += 1;
            *total_fees_in_queue += max_fee;
            if max_fee < *max_fee_limit {
                *max_fee_limit = max_fee;
            }
        }

        updated_user_states
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

    /// Updates or removes a user's state when their latest proof entry is removed from the batch queue.
    ///
    /// If the user has no other proofs remaining in the queue, their state is removed entirely.
    /// Otherwise, the user's state is updated to reflect the next most recent entry in the queue.
    ///
    /// Note: The given `removed_entry` must be the most recent (latest or highest nonce) entry for the user in the queue.
    pub(crate) fn update_user_state_on_entry_removal(&mut self, removed_entry: &BatchQueueEntry) {
        let addr = removed_entry.sender;

        let new_last_max_fee_limit = match self
            .batch_queue
            .iter()
            .filter(|(e, _)| e.sender == addr)
            .next_back()
        {
            Some((last_entry, _)) => last_entry.nonced_verification_data.max_fee,
            None => {
                self.user_states.remove(&addr);
                return;
            }
        };

        if let Entry::Occupied(mut user_state) = self.user_states.entry(addr) {
            user_state.get_mut().proofs_in_batch -= 1;
            user_state.get_mut().nonce -= U256::one();
            user_state.get_mut().total_fees_in_queue -=
                removed_entry.nonced_verification_data.max_fee;
            user_state.get_mut().last_max_fee_limit = new_last_max_fee_limit;
        }
    }

    pub(crate) fn is_queue_full(&self) -> bool {
        self.batch_queue.len() >= self.max_size
    }
}
