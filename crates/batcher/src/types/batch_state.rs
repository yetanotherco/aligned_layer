use super::batch_queue::{BatchQueue, BatchQueueEntry};
use ethers::types::{Address, U256};
use log::debug;

pub(crate) struct BatchState {
    pub(crate) batch_queue: BatchQueue,
    pub(crate) max_size: usize,
}

impl BatchState {
    // CONSTRUCTORS:

    pub(crate) fn new(max_size: usize) -> Self {
        Self {
            batch_queue: BatchQueue::new(),
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

    pub(crate) fn get_user_min_fee_in_batch(&self, addr: &Address) -> U256 {
        self.batch_queue
            .iter()
            .filter(|(e, _)| &e.sender == addr)
            .map(|(e, _)| e.nonced_verification_data.max_fee)
            .min()
            .unwrap_or(U256::max_value())
    }

    // LOGIC:

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

    pub(crate) fn is_queue_full(&self) -> bool {
        self.batch_queue.len() >= self.max_size
    }
}
