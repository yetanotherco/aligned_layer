use tokio::sync::{broadcast::Receiver, Mutex};
use tokio_tungstenite::tungstenite::Message;

/// A struct representing a websocket connection state.
///
/// `rx` is the receiver part of a broadcast channel. It listens for messages to be
/// responded to the connected clients. It is set to None until some data was included to the
/// batch. It is reseted to None once the batch merkle root information is sent to the
/// connected clients.
/// `received_msgs` manages the count of received messages from this connection.
/// `responded_msgs` manages the count of data added to a batch, what will mean that
/// the message was responded.
pub(crate) struct Connection {
    pub(crate) rx: Mutex<Option<Receiver<Message>>>,
    pub(crate) received_msgs: Mutex<usize>,
    pub(crate) responded_msgs: Mutex<usize>,
}

impl Connection {
    pub(crate) fn new() -> Self {
        Connection {
            rx: Mutex::new(None),
            received_msgs: Mutex::new(0),
            responded_msgs: Mutex::new(0),
        }
    }
}
