use std::fmt;

use ethers::types::SignatureError;
use tokio_tungstenite::tungstenite;

pub enum BatcherError {
    ConnectionError(tungstenite::Error),
    BatchVerifiedEventStreamError(String),
    EthereumSubscriptionError(String),
    SignatureError(SignatureError),
    TaskCreationError(String),
    ReceiptNotFoundError,
    TransactionSendError,
    MaxRetriesReachedError,
    SerializationError(String),
}

impl From<tungstenite::Error> for BatcherError {
    fn from(e: tungstenite::Error) -> Self {
        BatcherError::ConnectionError(e)
    }
}

impl From<SignatureError> for BatcherError {
    fn from(e: SignatureError) -> Self {
        BatcherError::SignatureError(e)
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
            BatcherError::SignatureError(e) => {
                write!(f, "Message signature verification error: {}", e)
            }
            BatcherError::TaskCreationError(e) => {
                write!(f, "Task creation error: {}", e)
            }
            BatcherError::ReceiptNotFoundError => {
                write!(f, "Receipt not found")
            }
            BatcherError::TransactionSendError => {
                write!(f, "Error sending tx")
            }
            BatcherError::MaxRetriesReachedError => {
                write!(
                    f,
                    "Maximum tries reached. Could not send createNewTask call"
                )
            }
            BatcherError::SerializationError(e) => {
                write!(f, "Serialization error: {}", e)
            }
        }
    }
}
