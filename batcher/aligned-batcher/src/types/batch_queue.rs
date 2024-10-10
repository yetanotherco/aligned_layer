use aligned_sdk::{
    communication::serialization::cbor_serialize,
    core::types::{NoncedVerificationData, VerificationDataCommitment},
};
use ethers::types::{Address, Signature, U256};
use priority_queue::PriorityQueue;
use std::{
    hash::{Hash, Hasher},
    ops::ControlFlow,
};

use super::errors::BatcherError;
use crate::connection::WsMessageSink;

#[derive(Clone)]
pub(crate) struct BatchQueueEntry {
    pub(crate) nonced_verification_data: NoncedVerificationData,
    pub(crate) verification_data_commitment: VerificationDataCommitment,
    pub(crate) messaging_sink: Option<WsMessageSink>,
    pub(crate) signature: Signature,
    pub(crate) sender: Address,
}

#[derive(Clone)]
pub(crate) struct BatchQueueEntryPriority {
    max_fee: U256,
    nonce: U256,
}

impl BatchQueueEntry {
    pub fn new(
        nonced_verification_data: NoncedVerificationData,
        verification_data_commitment: VerificationDataCommitment,
        messaging_sink: WsMessageSink,
        signature: Signature,
        sender: Address,
    ) -> Self {
        BatchQueueEntry {
            nonced_verification_data,
            verification_data_commitment,
            messaging_sink: Some(messaging_sink),
            signature,
            sender,
        }
    }

    #[cfg(test)]
    pub fn new_for_testing(
        nonced_verification_data: NoncedVerificationData,
        verification_data_commitment: VerificationDataCommitment,
        signature: Signature,
        sender: Address,
    ) -> Self {
        BatchQueueEntry {
            nonced_verification_data,
            verification_data_commitment,
            messaging_sink: None,
            signature,
            sender,
        }
    }
}

impl BatchQueueEntryPriority {
    pub fn new(max_fee: U256, nonce: U256) -> Self {
        BatchQueueEntryPriority { max_fee, nonce }
    }
}

impl Eq for BatchQueueEntry {}

// We consider two entries to be equal if they have the same sender and nonce
impl PartialEq for BatchQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.sender == other.sender
            && self.nonced_verification_data.nonce == other.nonced_verification_data.nonce
    }
}

impl Hash for BatchQueueEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sender.hash(state);
        self.nonced_verification_data.nonce.hash(state);
    }
}

impl Eq for BatchQueueEntryPriority {}

impl PartialEq for BatchQueueEntryPriority {
    fn eq(&self, other: &Self) -> bool {
        self.max_fee == other.max_fee && self.nonce == other.nonce
    }
}

impl PartialOrd for BatchQueueEntryPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BatchQueueEntryPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = other.max_fee.cmp(&self.max_fee);
        if ord == std::cmp::Ordering::Equal {
            self.nonce.cmp(&other.nonce).reverse()
        } else {
            ord
        }
    }
}

pub(crate) type BatchQueue = PriorityQueue<BatchQueueEntry, BatchQueueEntryPriority>;

/// Calculates the size of the batch represented by the given batch queue.
pub(crate) fn calculate_batch_size(batch_queue: &BatchQueue) -> Result<usize, BatcherError> {
    let folded_result = batch_queue.iter().try_fold(0, |acc, (entry, _)| {
        if let Ok(verification_data_bytes) =
            cbor_serialize(&entry.nonced_verification_data.verification_data)
        {
            let current_batch_size = acc + verification_data_bytes.len();
            ControlFlow::Continue(current_batch_size)
        } else {
            ControlFlow::Break(())
        }
    });

    if let ControlFlow::Continue(batch_size) = folded_result {
        Ok(batch_size)
    } else {
        Err(BatcherError::SerializationError(String::from(
            "Could not calculate size of batch",
        )))
    }
}

/// This function tries to build a batch to be submitted to Aligned.
/// Given a copy of the current batch queue, , and applyies an algorithm to find the biggest batch
/// of proofs from users that are willing to pay for it:
/// 1. Traverse each batch priority queue, starting from the one with minimum max fee.
/// 2. Calculate the `fee_per_proof` for the whole batch and compare with the `max_fee` of the entry.
/// 3. If `fee_per_proof` is less than the `max_fee` of the current entry, submit the batch. If not, pop this entry
///     from the queue and push it to `resulting_priority_queue`, then repeat step 1.
///
/// `resulting_priority_queue` will be the batch queue composed of all entries that were not willing to pay for the batch.
/// This is outputted in along with the finalized batch.
pub(crate) fn try_build_batch(
    batch_queue: BatchQueue,
    gas_price: U256,
    max_batch_size: usize,
) -> Result<(BatchQueue, Vec<BatchQueueEntry>), BatcherError> {
    let mut batch_queue = batch_queue;
    let mut batch_size = calculate_batch_size(&batch_queue)?;
    let mut resulting_priority_queue = BatchQueue::new();

    while let Some((entry, _)) = batch_queue.peek() {
        let batch_len = batch_queue.len();
        let fee_per_proof = calculate_fee_per_proof(batch_len, gas_price);

        if batch_size > max_batch_size || fee_per_proof > entry.nonced_verification_data.max_fee {
            // Update the state for the next iteration:
            // * Subtract this entry size to the size of the batch size.
            // * Push the current entry to the resulting batch queue.

            // It is safe to call `.unwrap()` here since any serialization error should have been caught
            // when calculating the total size of the batch with the `calculate_batch_size` function
            let verification_data_size =
                cbor_serialize(&entry.nonced_verification_data.verification_data)
                    .unwrap()
                    .len();
            batch_size -= verification_data_size;

            let (not_working_entry, not_working_priority) = batch_queue.pop().unwrap();
            resulting_priority_queue.push(not_working_entry, not_working_priority);

            continue;
        }

        // At this point, we break since we found a batch that can be submitted
        break;
    }

    // If `batch_queue_copy` is empty, this means that all the batch queue was traversed and we didn't find
    // any user willing to pay fot the fee per proof.
    if batch_queue.is_empty() {
        return Err(BatcherError::BatchCostTooHigh);
    }

    Ok((
        resulting_priority_queue,
        batch_queue.clone().into_sorted_vec(),
    ))
}

fn calculate_fee_per_proof(batch_len: usize, gas_price: U256) -> U256 {
    let gas_per_proof = (crate::CONSTANT_GAS_COST
        + crate::ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * batch_len as u128)
        / batch_len as u128;

    U256::from(gas_per_proof) * gas_price
}

#[cfg(test)]
mod test {
    use aligned_sdk::core::types::ProvingSystemId;
    use aligned_sdk::core::types::VerificationData;
    use ethers::types::Address;

    use super::*;

    #[test]
    fn batch_finalization_algorithm_works_from_same_sender() {
        // The following information will be the same for each entry, it is just some dummy data to see
        // algorithm working.

        let proof_generator_addr = Address::random();
        let payment_service_addr = Address::random();
        let sender_addr = Address::random();
        let bytes_for_verification_data = vec![42_u8; 10];
        let dummy_signature = Signature {
            r: U256::from(1),
            s: U256::from(2),
            v: 3,
        };
        let verification_data = VerificationData {
            proving_system: ProvingSystemId::Risc0,
            proof: bytes_for_verification_data.clone(),
            pub_input: Some(bytes_for_verification_data.clone()),
            verification_key: Some(bytes_for_verification_data.clone()),
            vm_program_code: Some(bytes_for_verification_data),
            proof_generator_addr,
        };
        let chain_id = U256::from(42);

        // Here we create different entries for the batch queue.
        // Since we are sending with the same address, the low nonces should have higher max fees.

        // Entry 1
        let nonce_1 = U256::from(1);
        let max_fee_1 = U256::from(1300000000000002u128);
        let nonced_verification_data_1 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_1,
            max_fee_1,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_1: VerificationDataCommitment = nonced_verification_data_1.clone().into();
        let entry_1 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_1,
            vd_commitment_1,
            dummy_signature,
            sender_addr,
        );
        let batch_priority_1 = BatchQueueEntryPriority::new(max_fee_1, nonce_1);

        // Entry 2
        let nonce_2 = U256::from(2);
        let max_fee_2 = U256::from(1_300_000_000_000_001u128);
        let nonced_verification_data_2 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_2,
            max_fee_2,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_2: VerificationDataCommitment = nonced_verification_data_2.clone().into();
        let entry_2 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_2,
            vd_commitment_2,
            dummy_signature,
            sender_addr,
        );
        let batch_priority_2 = BatchQueueEntryPriority::new(max_fee_2, nonce_2);

        // Entry 3
        let nonce_3 = U256::from(3);
        let max_fee_3 = U256::from(1_300_000_000_000_000u128);
        let nonced_verification_data_3 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_3,
            max_fee_3,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_3: VerificationDataCommitment = nonced_verification_data_3.clone().into();
        let entry_3 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_3,
            vd_commitment_3,
            dummy_signature,
            sender_addr,
        );
        let batch_priority_3 = BatchQueueEntryPriority::new(max_fee_3, nonce_3);

        let mut batch_queue = BatchQueue::new();
        batch_queue.push(entry_1, batch_priority_1);
        batch_queue.push(entry_2, batch_priority_2);
        batch_queue.push(entry_3, batch_priority_3);

        let gas_price = U256::from(1);
        let (resulting_batch_queue, batch) =
            try_build_batch(batch_queue, gas_price, 5000000).unwrap();

        assert!(resulting_batch_queue.is_empty());

        assert_eq!(batch[0].nonced_verification_data.max_fee, max_fee_3);
        assert_eq!(batch[1].nonced_verification_data.max_fee, max_fee_2);
        assert_eq!(batch[2].nonced_verification_data.max_fee, max_fee_1);
    }

    #[test]
    fn batch_finalization_algorithm_works_from_different_senders() {
        // The following information will be the same for each entry, it is just some dummy data to see
        // algorithm working.

        let proof_generator_addr = Address::random();
        let payment_service_addr = Address::random();
        let sender_addr_1 = Address::random();
        let sender_addr_2 = Address::random();
        let sender_addr_3 = Address::random();
        let bytes_for_verification_data = vec![42_u8; 10];
        let dummy_signature = Signature {
            r: U256::from(1),
            s: U256::from(2),
            v: 3,
        };
        let verification_data = VerificationData {
            proving_system: ProvingSystemId::Risc0,
            proof: bytes_for_verification_data.clone(),
            pub_input: Some(bytes_for_verification_data.clone()),
            verification_key: Some(bytes_for_verification_data.clone()),
            vm_program_code: Some(bytes_for_verification_data),
            proof_generator_addr,
        };
        let chain_id = U256::from(42);

        // Here we create different entries for the batch queue.
        // Since we are sending from different addresses, there is no restriction on the max fee and
        // nonces of the batch queue entries.

        // Entry 1
        let nonce_1 = U256::from(10);
        let max_fee_1 = U256::from(1300000000000001u128);
        let nonced_verification_data_1 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_1,
            max_fee_1,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_1: VerificationDataCommitment = nonced_verification_data_1.clone().into();
        let entry_1 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_1,
            vd_commitment_1,
            dummy_signature,
            sender_addr_1,
        );
        let batch_priority_1 = BatchQueueEntryPriority::new(max_fee_1, nonce_1);

        // Entry 2
        let nonce_2 = U256::from(20);
        let max_fee_2 = U256::from(1_300_000_000_000_002u128);
        let nonced_verification_data_2 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_2,
            max_fee_2,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_2: VerificationDataCommitment = nonced_verification_data_2.clone().into();
        let entry_2 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_2,
            vd_commitment_2,
            dummy_signature,
            sender_addr_2,
        );
        let batch_priority_2 = BatchQueueEntryPriority::new(max_fee_2, nonce_2);

        // Entry 3
        let nonce_3 = U256::from(14);
        let max_fee_3 = U256::from(1_300_000_000_000_000u128);
        let nonced_verification_data_3 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_3,
            max_fee_3,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_3: VerificationDataCommitment = nonced_verification_data_3.clone().into();
        let entry_3 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_3,
            vd_commitment_3,
            dummy_signature,
            sender_addr_3,
        );
        let batch_priority_3 = BatchQueueEntryPriority::new(max_fee_3, nonce_3);

        let mut batch_queue = BatchQueue::new();
        batch_queue.push(entry_1, batch_priority_1);
        batch_queue.push(entry_2, batch_priority_2);
        batch_queue.push(entry_3, batch_priority_3);

        let gas_price = U256::from(1);
        let (resulting_batch_queue, finalized_batch) =
            try_build_batch(batch_queue, gas_price, 5000000).unwrap();

        // The resulting batch queue (entries from the old batch queue that were not willing to pay
        // in this batch), should be empty and hence, all entries from the batch queue should be in
        // the finalized batch.
        assert!(resulting_batch_queue.is_empty());
        assert_eq!(finalized_batch.len(), 3);
        assert_eq!(
            finalized_batch[0].nonced_verification_data.max_fee,
            max_fee_3
        );
        assert_eq!(
            finalized_batch[1].nonced_verification_data.max_fee,
            max_fee_1
        );
        assert_eq!(
            finalized_batch[2].nonced_verification_data.max_fee,
            max_fee_2
        );
    }

    #[test]
    fn batch_finalization_algorithm_works_one_not_willing_to_pay() {
        // The following information will be the same for each entry, it is just some dummy data to see
        // algorithm working.

        let proof_generator_addr = Address::random();
        let payment_service_addr = Address::random();
        let sender_addr_1 = Address::random();
        let sender_addr_2 = Address::random();
        let sender_addr_3 = Address::random();
        let bytes_for_verification_data = vec![42_u8; 10];
        let dummy_signature = Signature {
            r: U256::from(1),
            s: U256::from(2),
            v: 3,
        };
        let verification_data = VerificationData {
            proving_system: ProvingSystemId::Risc0,
            proof: bytes_for_verification_data.clone(),
            pub_input: Some(bytes_for_verification_data.clone()),
            verification_key: Some(bytes_for_verification_data.clone()),
            vm_program_code: Some(bytes_for_verification_data),
            proof_generator_addr,
        };
        let chain_id = U256::from(42);

        // Entry 1
        let nonce_1 = U256::from(10);
        let max_fee_1 = U256::from(1300000000000002u128);
        let nonced_verification_data_1 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_1,
            max_fee_1,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_1: VerificationDataCommitment = nonced_verification_data_1.clone().into();
        let entry_1 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_1,
            vd_commitment_1,
            dummy_signature,
            sender_addr_1,
        );
        let batch_priority_1 = BatchQueueEntryPriority::new(max_fee_1, nonce_1);

        // Entry 2
        let nonce_2 = U256::from(20);
        let max_fee_2 = U256::from(1_300_000_000_000_001u128);
        let nonced_verification_data_2 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_2,
            max_fee_2,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_2: VerificationDataCommitment = nonced_verification_data_2.clone().into();
        let entry_2 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_2,
            vd_commitment_2,
            dummy_signature,
            sender_addr_2,
        );
        let batch_priority_2 = BatchQueueEntryPriority::new(max_fee_2, nonce_2);

        // Entry 3
        let nonce_3 = U256::from(14);
        let max_fee_3 = U256::from(10);
        let nonced_verification_data_3 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_3,
            max_fee_3,
            chain_id,
            payment_service_addr,
        );
        let vd_commitment_3: VerificationDataCommitment = nonced_verification_data_3.clone().into();
        let entry_3 = BatchQueueEntry::new_for_testing(
            nonced_verification_data_3,
            vd_commitment_3,
            dummy_signature,
            sender_addr_3,
        );
        let batch_priority_3 = BatchQueueEntryPriority::new(max_fee_3, nonce_3);

        let mut batch_queue = BatchQueue::new();
        batch_queue.push(entry_1, batch_priority_1);
        batch_queue.push(entry_2, batch_priority_2);
        batch_queue.push(entry_3, batch_priority_3);

        let gas_price = U256::from(1);
        let (resulting_batch_queue, finalized_batch) =
            try_build_batch(batch_queue, gas_price, 5000000).unwrap();

        // The resulting batch queue (entries from the old batch queue that were not willing to pay
        // in this batch), should be empty and hence, all entries from the batch queue should be in
        // the finalized batch.

        assert_eq!(resulting_batch_queue.len(), 1);
        assert_eq!(finalized_batch.len(), 2);
        assert_eq!(
            finalized_batch[0].nonced_verification_data.max_fee,
            max_fee_2
        );
        assert_eq!(
            finalized_batch[1].nonced_verification_data.max_fee,
            max_fee_1
        );
    }
}
