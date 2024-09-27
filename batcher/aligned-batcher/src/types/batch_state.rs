use std::collections::HashMap;

use super::batch_queue::{BatchQueue, BatchQueueEntry};
use ethers::types::{Address, U256};
use log::debug;

pub(crate) struct BatchState {
    pub(crate) batch_queue: BatchQueue,
}

impl BatchState {
    pub(crate) fn new() -> Self {
        Self {
            batch_queue: BatchQueue::new(),
        }
    }

    // fn get_user_proof_count(&self, addr: &Address) -> u64 {
    //     *self.user_proof_count_in_batch.get(addr).unwrap_or(&0)
    // }

    // /*
    //    Increments the user proof count in the batch, if the user is already in the hashmap.
    //    If the user is not in the hashmap, it adds the user to the hashmap with a count of 1 to represent the first proof.
    // */
    // fn increment_user_proof_count(&mut self, addr: &Address) {
    //     self.user_proof_count_in_batch
    //         .entry(*addr)
    //         .and_modify(|count| *count += 1)
    //         .or_insert(1);
    // }

    pub(crate) fn get_entry(&self, sender: Address, nonce: U256) -> Option<&BatchQueueEntry> {
        self.batch_queue
            .iter()
            .map(|(entry, _)| entry)
            .find(|entry| entry.sender == sender && entry.nonced_verification_data.nonce == nonce)
    }

    /// Checks if the entry is valid
    /// An entry is valid if there is no entry with the same sender,
    /// lower nonce and a lower fee
    /// If the entry is valid, it replaces the entry in the queue
    /// to increment the max fee, then it updates the user min fee if necessary
    /// If the entry is invalid, it returns a validity response message.
    /// If the entry is valid, it returns None.
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

    //     if !is_valid {
    //         return Some(ValidityResponseMessage::InvalidReplacementMessage);
    //     }

    //     // remove the old entry and insert the new one
    //     // note that the entries are considered equal for the priority queue
    //     // if they have the same nonce and sender, so we can remove the old entry
    //     // by calling remove with the new entry
    //     self.batch_queue.remove(&replacement_entry);
    //     self.batch_queue.push(
    //         replacement_entry.clone(),
    //         BatchQueueEntryPriority::new(replacement_max_fee, nonce),
    //     );

    //     let user_min_fee = self
    //         .batch_queue
    //         .iter()
    //         .filter(|(e, _)| e.sender == sender)
    //         .map(|(e, _)| e.nonced_verification_data.max_fee)
    //         .min()
    //         .unwrap_or(U256::max_value());

    //     self.user_min_fee.insert(sender, user_min_fee);

    //     None
    // }

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
