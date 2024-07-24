use ethers::types::Signature;
use futures_util::stream::SplitSink;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use aligned_sdk::core::types::{NoncedVerificationData, VerificationDataCommitment};

pub(crate) type BatchQueueEntry = (
    NoncedVerificationData,
    VerificationDataCommitment,
    Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    Signature,
);

pub(crate) type BatchQueue = Vec<BatchQueueEntry>;
