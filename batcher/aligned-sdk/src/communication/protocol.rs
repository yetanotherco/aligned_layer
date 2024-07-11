use futures_util::{stream::SplitStream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::core::{errors::SubmitError, types::ResponseMessage};

pub const EXPECTED_PROTOCOL_VERSION: u16 = 1;

pub async fn check_protocol_version(
    ws_read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Result<(), SubmitError> {
    if let Some(Ok(msg)) = ws_read.next().await {
        match serde_json::from_slice::<ResponseMessage>(&msg.into_data()) {
            Ok(ResponseMessage::ProtocolVersion(protocol_version)) => {
                if protocol_version > EXPECTED_PROTOCOL_VERSION {
                    return Err(SubmitError::ProtocolVersionMismatch {
                        current: protocol_version,
                        expected: EXPECTED_PROTOCOL_VERSION,
                    });
                }
                return Ok(());
            }
            Ok(_) => {
                return Err(SubmitError::UnexpectedBatcherResponse(
                    "Batcher did not respond with the protocol version".to_string(),
                ));
            }
            Err(e) => {
                return Err(SubmitError::SerializationError(e));
            }
        }
    }
    Err(SubmitError::UnexpectedBatcherResponse(
        "Batcher did not respond with the protocol version".to_string(),
    ))
}
