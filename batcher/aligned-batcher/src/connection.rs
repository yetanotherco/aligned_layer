use std::sync::Arc;

use crate::types::{batch_queue::BatchQueueEntry, errors::BatcherError};
use aligned_sdk::{
    communication::serialization::cbor_serialize,
    core::types::{BatchInclusionData, ResponseMessage, VerificationCommitmentBatch},
};
use futures_util::{stream::SplitSink, SinkExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{error, info};
use serde::Serialize;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{
    tungstenite::{Error, Message},
    WebSocketStream,
};

pub(crate) type WsMessageSink = Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>;

pub(crate) async fn send_batch_inclusion_data_responses(
    finalized_batch: Vec<BatchQueueEntry>,
    batch_merkle_tree: &MerkleTree<VerificationCommitmentBatch>,
) -> Result<(), BatcherError> {
    for (vd_batch_idx, entry) in finalized_batch.iter().enumerate() {
        let batch_inclusion_data = BatchInclusionData::new(vd_batch_idx, batch_merkle_tree);
        let response = ResponseMessage::BatchInclusionData(batch_inclusion_data);

        let serialized_response = cbor_serialize(&response)
            .map_err(|e| BatcherError::SerializationError(e.to_string()))?;

        let Some(ws_sink) = entry.messaging_sink.as_ref() else {
            return Err(BatcherError::WsSinkEmpty);
        };

        let sending_result = ws_sink
            .write()
            .await
            .send(Message::binary(serialized_response))
            .await;

        match sending_result {
            Err(Error::AlreadyClosed) => (),
            Err(e) => error!("Error while sending batch inclusion data response: {}", e),
            Ok(_) => (),
        }

        info!("Response sent");
    }

    Ok(())
}

pub(crate) async fn send_message<T: Serialize>(ws_conn_sink: WsMessageSink, message: T) {
    match cbor_serialize(&message) {
        Ok(serialized_response) => {
            if let Err(err) = ws_conn_sink
                .write()
                .await
                .send(Message::binary(serialized_response))
                .await
            {
                error!("Error while sending message: {}", err)
            }
        }
        Err(e) => error!("Error while serializing message: {}", e),
    }
}
