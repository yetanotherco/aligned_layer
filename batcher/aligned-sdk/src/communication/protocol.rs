use futures_util::{stream::SplitStream, StreamExt};
use log::error;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::core::{errors::SubmitError, types::ResponseMessage};

pub const CURRENT_PROTOCOL_VERSION: u16 = 0;

pub async fn check_protocol_version(
    ws_read: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Result<(), SubmitError> {
    if let Some(Ok(msg)) = ws_read.next().await {
        match serde_json::from_slice::<ResponseMessage>(&msg.into_data()) {
            Ok(ResponseMessage::ProtocolVersion(protocol_version)) => {
                if protocol_version > CURRENT_PROTOCOL_VERSION {
                    return Err(SubmitError::ProtocolVersionMismatch(
                        CURRENT_PROTOCOL_VERSION,
                        protocol_version,
                    ));
                }
            }
            Ok(_) => {
                error!("Batcher did not respond with the protocol version");
                return Err(SubmitError::GenericError(
                    "No protocol version received".to_string(),
                ));
            }
            Err(e) => {
                error!("Error while deserializing batcher response: {}", e);
                return Err(SubmitError::SerdeError(e));
            }
        }
    } else {
        error!("Batcher did not respond with the protocol version");
        return Err(SubmitError::GenericError(
            "No protocol version received".to_string(),
        ));
    }

    Ok(())
}
