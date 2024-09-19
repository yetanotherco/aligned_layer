use ethers::types::{Address, Signature, U256};
use futures_util::stream::SplitSink;
use priority_queue::PriorityQueue;
use std::{
    hash::{Hash, Hasher},
    ops::ControlFlow,
    sync::Arc,
};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use aligned_sdk::{
    communication::serialization::cbor_serialize,
    core::types::{NoncedVerificationData, VerificationDataCommitment},
};

use super::errors::BatcherError;

#[derive(Clone)]
pub(crate) struct BatchQueueEntry {
    pub(crate) nonced_verification_data: NoncedVerificationData,
    pub(crate) verification_data_commitment: VerificationDataCommitment,
    pub(crate) messaging_sink: Option<Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>>,
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
        messaging_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
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
        let ord = self.max_fee.cmp(&other.max_fee);
        if ord == std::cmp::Ordering::Equal {
            self.nonce.cmp(&other.nonce).reverse()
        } else {
            ord.reverse()
        }
    }
}

pub(crate) type BatchQueue = PriorityQueue<BatchQueueEntry, BatchQueueEntryPriority>;

pub(crate) fn calculate_batch_size(
    batch_queue: &PriorityQueue<BatchQueueEntry, BatchQueueEntryPriority>,
) -> Result<usize, BatcherError> {
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

pub(crate) fn try_build_batch(
    batch_queue_copy: &mut BatchQueue,
    gas_price: U256,
    max_batch_size: usize,
) -> Result<(BatchQueue, Vec<BatchQueueEntry>), BatcherError> {
    let mut batch_size = calculate_batch_size(batch_queue_copy)?;
    let mut resulting_priority_queue =
        PriorityQueue::<BatchQueueEntry, BatchQueueEntryPriority>::new();

    while let Some((entry, _)) = batch_queue_copy.peek() {
        let batch_len = batch_queue_copy.len();
        let fee_per_proof = calculate_fee_per_proof(batch_len, gas_price);

        if batch_size > max_batch_size || fee_per_proof > entry.nonced_verification_data.max_fee {
            // Update the state for the next iteration

            // It is safe to call `.unwrap()` here since any serialization error should have been caught
            // when calculating the total size of the batch with the `calculate_batch_size` function
            let verification_data_size =
                cbor_serialize(&entry.nonced_verification_data.verification_data)
                    .unwrap()
                    .len();

            batch_size -= verification_data_size;

            let (not_working_entry, not_woring_priority) = batch_queue_copy.pop().unwrap();
            resulting_priority_queue.push(not_working_entry, not_woring_priority);

            continue;
        }

        // At this point, we break since we found a batch that can be submitted
        break;
    }

    let batch = batch_queue_copy.clone().into_sorted_vec();

    // If `batch` is empty, this means that all the batch queue was traversed and we didn't find
    // any user willing to pay fot the fee per proof.
    if batch.is_empty() {
        return Err(BatcherError::BatchCostTooHigh);
    }

    Ok((
        resulting_priority_queue,
        batch_queue_copy.clone().into_sorted_vec(),
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
    use ethers::core::rand::thread_rng;
    use ethers::signers::LocalWallet;
    use ethers::signers::Signer;

    use super::*;

    #[tokio::test]
    fn batch_finalization_algorithm_works_from_same_sender() {
        // let stream = TcpStream::connect("test_stream").await.unwrap();

        let mut batch_queue = BatchQueue::new();
        // The following information will be the same for each entry, it is just some dummy data to see
        // algorithm working.
        let proof_generator_addr = LocalWallet::new(&mut thread_rng()).address();
        let payment_service_addr = LocalWallet::new(&mut thread_rng()).address();
        let sender_addr = LocalWallet::new(&mut thread_rng()).address();
        let some_bytes = vec![42_u8; 10];
        let verification_data = VerificationData {
            proving_system: ProvingSystemId::Risc0,
            proof: some_bytes.clone(),
            pub_input: Some(some_bytes.clone()),
            verification_key: Some(some_bytes.clone()),
            vm_program_code: Some(some_bytes),
            proof_generator_addr: proof_generator_addr,
        };
        let chain_id = U256::from(42);

        // Here we create different entries for the batch queue.

        // Entry 1
        let nonce_1 = U256::from(1);
        let max_fee_1 = U256::from(10);
        let nonced_verification_data_1 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_1,
            max_fee_1,
            chain_id,
            payment_service_addr,
        );

        // Entry 2
        let nonce_2 = U256::from(2);
        let max_fee_2 = U256::from(8);
        let nonced_verification_data_2 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_2,
            max_fee_2,
            chain_id,
            payment_service_addr,
        );

        // Entry 3
        let nonce_3 = U256::from(3);
        let max_fee_3 = U256::from(5);
        let nonced_verification_data_3 = NoncedVerificationData::new(
            verification_data.clone(),
            nonce_3,
            max_fee_3,
            chain_id,
            payment_service_addr,
        );
    }
}
