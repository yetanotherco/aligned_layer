use ethers::types::{Address, Signature, U256};
use futures_util::stream::SplitSink;
use priority_queue::PriorityQueue;
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use aligned_sdk::core::types::{NoncedVerificationData, VerificationDataCommitment};

#[derive(Clone)]
pub(crate) struct BatchQueueEntry {
    pub(crate) nonced_verification_data: NoncedVerificationData,
    pub(crate) verification_data_commitment: VerificationDataCommitment,
    pub(crate) messaging_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
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
            messaging_sink,
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
            ord
        }
    }
}

pub(crate) type BatchQueue = PriorityQueue<BatchQueueEntry, BatchQueueEntryPriority>;
