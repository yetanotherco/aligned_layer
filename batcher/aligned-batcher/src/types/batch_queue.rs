use std::sync::Arc;

use futures_util::stream::SplitSink;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use aligned_batcher_lib::types::{VerificationData, VerificationDataCommitment};

pub(crate) type BatchQueueEntry = (
    VerificationData,
    VerificationDataCommitment,
    Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
);

pub(crate) type BatchQueue = Vec<BatchQueueEntry>;
