use aligned_sdk::common::{
    constants::CBOR_ARRAY_MAX_OVERHEAD,
    types::{NoncedVerificationData, VerificationDataCommitment},
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
        // Implementation of lowest-first:
        let ord: std::cmp::Ordering = other.max_fee.cmp(&self.max_fee);
        // This means, less max_fee will go first
        // We want this because we will .pop() to remove unwanted elements, low fee submitions.

        if ord == std::cmp::Ordering::Equal {
            // Case of same max_fee:
            // Implementation of biggest-first:
            // Since we want to .pop() entries with biggest nonce first, because we want to submit low nonce first
            self.nonce.cmp(&other.nonce)
        } else {
            ord
        }
    }
}

pub(crate) type BatchQueue = PriorityQueue<BatchQueueEntry, BatchQueueEntryPriority>;

/// Calculates the size of the batch represented by the given batch queue.
pub(crate) fn calculate_batch_size(batch_queue: &BatchQueue) -> Result<usize, BatcherError> {
    let folded_result = batch_queue.iter().try_fold(0, |acc, (entry, _)| {
        let verification_data_size = entry.nonced_verification_data.cbor_size_upper_bound();
        let current_batch_size = acc + verification_data_size;
        ControlFlow::<usize, usize>::Continue(current_batch_size)
    });

    if let ControlFlow::Continue(batch_size) = folded_result {
        // We over-estimate the size of the batch by at most 8 bytes.
        // This saves us from a scenario where we actually try to send more
        // than the maximum allowed bytes and get rejected by operators.
        Ok(CBOR_ARRAY_MAX_OVERHEAD + batch_size)
    } else {
        Err(BatcherError::SerializationError(String::from(
            "Could not calculate size of batch",
        )))
    }
}

/// This function tries to build a batch to be submitted to Aligned.
/// Given the current batch queue applies the following algorithm to find the biggest batch
/// of proofs from users that are willing to pay for it:
/// 1. Traverse each batch priority queue, starting from the one with minimum max fee.
/// 2. Calculate the `fee_per_proof` for the whole batch and compare with the `max_fee` of the entry.
/// 3. If `fee_per_proof` is less than the `max_fee` of the current entry, submit the batch. If not, pop this entry
///    from the queue. then repeat step 1.
///
/// Returns the finalized batch.
pub(crate) fn try_build_batch(
    batch_queue: BatchQueue,
    gas_price: U256,
    max_batch_byte_size: usize,
    max_batch_proof_qty: usize,
    constant_gas_cost: u128,
) -> Result<Vec<BatchQueueEntry>, BatcherError> {
    let mut finalized_batch = batch_queue;
    let mut batch_size = calculate_batch_size(&finalized_batch)?;

    while let Some((entry, _)) = finalized_batch.peek() {
        let batch_len = finalized_batch.len();
        let fee_per_proof = calculate_fee_per_proof(batch_len, gas_price, constant_gas_cost);

        // if batch is not acceptable:
        if batch_size > max_batch_byte_size
            || fee_per_proof > entry.nonced_verification_data.max_fee
            || batch_len > max_batch_proof_qty
        {
            // Update the state for the next iteration:
            // * Subtract this entry size to the size of the batch size.
            // * Push the current entry to the resulting batch queue.

            let verification_data_size = entry.nonced_verification_data.cbor_size_upper_bound();
            batch_size -= verification_data_size;

            finalized_batch.pop();

            continue;
        }

        // At this point, we break since we found a batch that can be submitted
        break;
    }

    // If `finalized_batch` is empty, this means that all the batch queue was traversed and we didn't find
    // any user willing to pay fot the fee per proof.
    if finalized_batch.is_empty() {
        return Err(BatcherError::BatchCostTooHigh);
    }

    Ok(finalized_batch.clone().into_sorted_vec())
}

fn calculate_fee_per_proof(batch_len: usize, gas_price: U256, constant_gas_cost: u128) -> U256 {
    let gas_per_proof = (constant_gas_cost
        + crate::ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * batch_len as u128)
        / batch_len as u128;

    U256::from(gas_per_proof) * gas_price
}

#[cfg(test)]
mod test {
    use aligned_sdk::common::constants::DEFAULT_CONSTANT_GAS_COST;
    use aligned_sdk::common::types::ProvingSystemId;
    use aligned_sdk::common::types::VerificationData;
    use ethers::types::Address;

    use super::*;
    use super::try_build_batch;

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
        let finalized_batch = try_build_batch(
            batch_queue.clone(),
            gas_price,
            5000000,
            50,
            DEFAULT_CONSTANT_GAS_COST,
        )
        .unwrap();

        assert_eq!(
            finalized_batch[0].nonced_verification_data.max_fee,
            max_fee_3
        );
        assert_eq!(
            finalized_batch[1].nonced_verification_data.max_fee,
            max_fee_2
        );
        assert_eq!(
            finalized_batch[2].nonced_verification_data.max_fee,
            max_fee_1
        );
    }

    #[test]
    fn batch_finalization_algorithm_works_from_same_sender_same_fee() {
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
        // All with the same fee

        let max_fee = U256::from(130000000000000u128);

        // Entry 1
        let nonce_1 = U256::from(1);
        let nonced_verification_data_1 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_1,
            max_fee,
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
        let batch_priority_1 = BatchQueueEntryPriority::new(max_fee, nonce_1);

        // Entry 2
        let nonce_2 = U256::from(2);
        let nonced_verification_data_2 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_2,
            max_fee,
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
        let batch_priority_2 = BatchQueueEntryPriority::new(max_fee, nonce_2);

        // Entry 3
        let nonce_3 = U256::from(3);
        let nonced_verification_data_3 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_3,
            max_fee,
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
        let batch_priority_3 = BatchQueueEntryPriority::new(max_fee, nonce_3);

        let mut batch_queue = BatchQueue::new();
        batch_queue.push(entry_1, batch_priority_1);
        batch_queue.push(entry_2, batch_priority_2);
        batch_queue.push(entry_3, batch_priority_3);

        let gas_price = U256::from(1);
        let finalized_batch = try_build_batch(
            batch_queue.clone(),
            gas_price,
            5000000,
            50,
            DEFAULT_CONSTANT_GAS_COST,
        )
        .unwrap();

        // All entries from the batch queue should be in
        // the finalized batch.
        assert!(batch_queue.len() == 3);

        assert_eq!(finalized_batch[0].nonced_verification_data.nonce, nonce_3);
        assert_eq!(finalized_batch[1].nonced_verification_data.nonce, nonce_2);
        assert_eq!(finalized_batch[2].nonced_verification_data.nonce, nonce_1);

        // sanity check
        assert_eq!(finalized_batch[2].nonced_verification_data.max_fee, max_fee);
    }

    #[test]
    fn batch_finalization_algorithm_works_from_same_sender_same_fee_nonempty_resulting_queue() {
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
        // All with the same fee

        let max_fee = U256::from(130000000000000u128);

        // Entry 1
        let nonce_1 = U256::from(1);
        let nonced_verification_data_1 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_1,
            max_fee,
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
        let batch_priority_1 = BatchQueueEntryPriority::new(max_fee, nonce_1);

        // Entry 2
        let nonce_2 = U256::from(2);
        let nonced_verification_data_2 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_2,
            max_fee,
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
        let batch_priority_2 = BatchQueueEntryPriority::new(max_fee, nonce_2);

        // Entry 3
        let nonce_3 = U256::from(3);
        let nonced_verification_data_3 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_3,
            max_fee,
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
        let batch_priority_3 = BatchQueueEntryPriority::new(max_fee, nonce_3);

        let mut batch_queue = BatchQueue::new();
        batch_queue.push(entry_1, batch_priority_1);
        batch_queue.push(entry_2, batch_priority_2);
        batch_queue.push(entry_3.clone(), batch_priority_3.clone());

        let gas_price = U256::from(1);
        let finalized_batch = try_build_batch(
            batch_queue.clone(),
            gas_price,
            5000000,
            2,
            DEFAULT_CONSTANT_GAS_COST,
        )
        .unwrap();

        // One Entry from the batch_queue should not be in the finalized batch
        // Particularly, nonce_3 is not in the finalized batch
        assert!(batch_queue.len() == 3);

        assert_eq!(finalized_batch[0].nonced_verification_data.nonce, nonce_2);
        assert_eq!(finalized_batch[1].nonced_verification_data.nonce, nonce_1);
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
        let finalized_batch = try_build_batch(
            batch_queue.clone(),
            gas_price,
            5000000,
            50,
            DEFAULT_CONSTANT_GAS_COST,
        )
        .unwrap();

        // All entries from the batch queue should be in
        // the finalized batch.
        assert_eq!(batch_queue.len(), 3);
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
        let finalized_batch = try_build_batch(
            batch_queue.clone(),
            gas_price,
            5000000,
            50,
            DEFAULT_CONSTANT_GAS_COST,
        )
        .unwrap();

        // All but one entries from the batch queue should be in the finalized batch.
        assert_eq!(batch_queue.len(), 3);
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

    #[test]
    fn batch_finalization_algorithm_works_not_bigger_than_max_batch_proof_qty() {
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
        let max_fee_1 = U256::from(1_300_000_000_000_002u128);
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

        // The max batch len is 2, so the algorithm should stop at the second entry.
        let max_batch_proof_qty = 2;

        let finalized_batch = try_build_batch(
            batch_queue.clone(),
            gas_price,
            5000000,
            max_batch_proof_qty,
            DEFAULT_CONSTANT_GAS_COST,
        )
        .unwrap();

        assert_eq!(batch_queue.len(), 3);
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

    #[test]
    fn test_batch_size_limit_enforcement_with_real_sp1_proofs() {
        use aligned_sdk::common::types::VerificationData;
        use aligned_sdk::communication::serialization::cbor_serialize;
        use std::fs;

        let proof_generator_addr = Address::random();
        let payment_service_addr = Address::random();
        let chain_id = U256::from(42);
        let dummy_signature = Signature {
            r: U256::from(1),
            s: U256::from(2),
            v: 3,
        };

        // Load actual SP1 proof files
        let proof_path = "../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof";
        let elf_path = "../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.elf";
        let pub_input_path = "../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.pub";

        let proof_data = match fs::read(proof_path) {
            Ok(data) => data,
            Err(_) => return, // Skip test if files not found
        };

        let elf_data = match fs::read(elf_path) {
            Ok(data) => data,
            Err(_) => return,
        };

        let pub_input_data = match fs::read(pub_input_path) {
            Ok(data) => data,
            Err(_) => return,
        };

        let verification_data = VerificationData {
            proving_system: ProvingSystemId::SP1,
            proof: proof_data,
            pub_input: Some(pub_input_data),
            verification_key: None,
            vm_program_code: Some(elf_data),
            proof_generator_addr,
        };

        // Create 10 entries using the same SP1 proof data
        let mut batch_queue = BatchQueue::new();
        let max_fee = U256::from(1_000_000_000_000_000u128);

        for i in 0..10 {
            let sender_addr = Address::random();
            let nonce = U256::from(i + 1);

            let nonced_verification_data = NoncedVerificationData::new(
                verification_data.clone(),
                nonce,
                max_fee,
                chain_id,
                payment_service_addr,
            );

            let vd_commitment: VerificationDataCommitment = nonced_verification_data.clone().into();
            let entry = BatchQueueEntry::new_for_testing(
                nonced_verification_data,
                vd_commitment,
                dummy_signature,
                sender_addr,
            );
            let batch_priority = BatchQueueEntryPriority::new(max_fee, nonce);
            batch_queue.push(entry, batch_priority);
        }

        // Test with a 5MB batch size limit
        let batch_size_limit = 5_000_000; // 5MB
        let gas_price = U256::from(1);

        let finalized_batch = try_build_batch(
            batch_queue.clone(),
            gas_price,
            batch_size_limit,
            50, // max proof qty
            DEFAULT_CONSTANT_GAS_COST,
        )
        .unwrap();

        // Verify the finalized batch respects the size limit
        let finalized_verification_data: Vec<VerificationData> = finalized_batch
            .iter()
            .map(|entry| entry.nonced_verification_data.verification_data.clone())
            .collect();

        let finalized_serialized = cbor_serialize(&finalized_verification_data).unwrap();
        let finalized_actual_size = finalized_serialized.len();

        // Assert the batch respects the limit
        assert!(
            finalized_actual_size <= batch_size_limit,
            "Finalized batch size {} exceeds limit {}",
            finalized_actual_size,
            batch_size_limit
        );

        // Verify some entries were included (not empty batch)
        assert!(!finalized_batch.is_empty(), "Batch should not be empty");

        // Verify not all entries were included (some should be rejected due to size limit)
        assert!(
            finalized_batch.len() < 10,
            "Batch should not include all entries due to size limit"
        );
    }

    #[test]
    fn test_cbor_size_upper_bound_accuracy() {
        use aligned_sdk::common::types::VerificationData;
        use aligned_sdk::communication::serialization::cbor_serialize;
        use std::fs;

        let proof_generator_addr = Address::random();
        let payment_service_addr = Address::random();
        let chain_id = U256::from(42);

        // Load actual SP1 proof files
        let proof_path = "../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof";
        let elf_path = "../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.elf";
        let pub_input_path = "../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.pub";

        let proof_data = match fs::read(proof_path) {
            Ok(data) => data,
            Err(_) => return, // Skip test if files not found
        };

        let elf_data = match fs::read(elf_path) {
            Ok(data) => data,
            Err(_) => return,
        };

        let pub_input_data = match fs::read(pub_input_path) {
            Ok(data) => data,
            Err(_) => return,
        };

        let verification_data = VerificationData {
            proving_system: ProvingSystemId::SP1,
            proof: proof_data,
            pub_input: Some(pub_input_data),
            verification_key: None,
            vm_program_code: Some(elf_data),
            proof_generator_addr,
        };

        let nonced_verification_data = NoncedVerificationData::new(
            verification_data.clone(),
            U256::from(1),
            U256::from(1_000_000_000_000_000u128),
            chain_id,
            payment_service_addr,
        );

        // Test cbor_size_upper_bound() accuracy
        let estimated_size = nonced_verification_data.cbor_size_upper_bound();

        // Compare with actual CBOR serialization of the full NoncedVerificationData
        let actual_nonced_serialized = cbor_serialize(&nonced_verification_data).unwrap();
        let actual_nonced_size = actual_nonced_serialized.len();

        // Also test the inner VerificationData for additional validation
        let actual_inner_serialized = cbor_serialize(&verification_data).unwrap();
        let actual_inner_size = actual_inner_serialized.len();

        // Verify CBOR encodes binary data efficiently (with serde_bytes fix), this misses some overhead but the proof is big enough as to not matter

        let raw_total = verification_data.proof.len()
            + verification_data.vm_program_code.as_ref().unwrap().len()
            + verification_data.pub_input.as_ref().unwrap().len();

        let cbor_efficiency_ratio = actual_inner_size as f64 / raw_total as f64;

        // With serde_bytes, CBOR should be very efficient (close to 1.0x)
        assert!(
            cbor_efficiency_ratio < 1.01,
            "CBOR serialization should be efficient with serde_bytes. Ratio: {:.3}x",
            cbor_efficiency_ratio
        );

        // Verify CBOR encodes binary data efficiently with serde_bytes
        // Should be close to 1.0x overhead (raw data size vs CBOR size)

        // The estimation should be an upper bound
        assert!(
            estimated_size >= actual_nonced_size,
            "cbor_size_upper_bound() should be an upper bound. Estimated: {}, Actual: {}",
            estimated_size,
            actual_nonced_size
        );

        // The estimation should also be reasonable (not wildly over-estimated)
        let estimation_overhead = estimated_size as f64 / actual_nonced_size as f64;
        assert!(
            estimation_overhead < 1.1,
            "Estimation should be reasonable, not wildly over-estimated. Overhead: {:.3}x",
            estimation_overhead
        );
    }
}
