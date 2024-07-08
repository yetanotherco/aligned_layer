use std::sync::Arc;

use ethers::types::Address;
use futures_util::stream::SplitSink;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use aligned_sdk::core::types::{VerificationData, VerificationDataCommitment};

pub(crate) type BatchQueueEntry = (
    VerificationData,
    VerificationDataCommitment,
    Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    Address,
);

pub(crate) type BatchQueue = Vec<BatchQueueEntry>;
