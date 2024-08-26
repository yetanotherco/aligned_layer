use ethers::types::Signature;
use futures_util::stream::SplitSink;
use std::{collections::BinaryHeap, sync::Arc};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use aligned_sdk::core::types::{NoncedVerificationData, VerificationDataCommitment};

#[derive(Clone)]
pub(crate) struct BatchQueueEntry {
    pub(crate) nonced_verification_data: NoncedVerificationData,
    pub(crate) verification_data_commitment: VerificationDataCommitment,
    pub(crate) messaging_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    pub(crate) signature: Signature,
}

impl BatchQueueEntry {
    pub fn new(
        nonced_verification_data: NoncedVerificationData,
        verification_data_commitment: VerificationDataCommitment,
        messaging_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        signature: Signature,
    ) -> Self {
        BatchQueueEntry {
            nonced_verification_data,
            verification_data_commitment,
            messaging_sink,
            signature,
        }
    }
}

impl Eq for BatchQueueEntry {}

impl PartialEq for BatchQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.nonced_verification_data.max_fee == other.nonced_verification_data.max_fee
            && self.nonced_verification_data.nonce == other.nonced_verification_data.nonce
    }
}

impl PartialOrd for BatchQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BatchQueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self
            .nonced_verification_data
            .max_fee
            .cmp(&other.nonced_verification_data.max_fee);
        if ord == std::cmp::Ordering::Equal {
            self.nonced_verification_data
                .nonce
                .cmp(&other.nonced_verification_data.nonce)
                .reverse()
        } else {
            ord
        }
    }
}

pub(crate) type BatchQueue = BinaryHeap<BatchQueueEntry>;
