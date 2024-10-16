use std::sync::Arc;

use aligned_sdk::communication::serialization::cbor_serialize;
use futures_util::{stream::SplitSink, SinkExt};
use log::error;
use serde::Serialize;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    WebSocketStream,
};

pub(crate) type WsMessageSink = Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>;

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

pub(crate) async fn drop_connection(ws_conn_sink: WsMessageSink, reason: String) {
    let close_frame = CloseFrame {
        code: CloseCode::Normal,
        reason: reason.into(),
    };

    ws_conn_sink
        .write()
        .await
        .send(tokio_tungstenite::tungstenite::Message::Close(Some(
            close_frame,
        )))
        .await
        .expect("Failed to send close frame");
}
