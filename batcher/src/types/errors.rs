use std::fmt;

use tokio_tungstenite::tungstenite;

pub enum BatcherError {
    ConnectionError(tungstenite::Error),
    BatchVerifiedEventStreamError(String),
    EthereumSubscriptionError(String),
    TransactionError(String),
}

impl From<tungstenite::Error> for BatcherError {
    fn from(e: tungstenite::Error) -> Self {
        BatcherError::ConnectionError(e)
    }
}

impl fmt::Debug for BatcherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BatcherError::ConnectionError(e) => {
                write!(f, "Web Socket Connection error: {}", e)
            }
            BatcherError::BatchVerifiedEventStreamError(e) => {
                write!(f, "`BatchVerified` event stream error: {}", e)
            }
            BatcherError::EthereumSubscriptionError(e) => {
                write!(f, "Ethereum subscription was not successful: {}", e)
            }
            BatcherError::TransactionError(e) => {
                write!(f, "Transaction error: {}", e)
            }
        }
    }
}
