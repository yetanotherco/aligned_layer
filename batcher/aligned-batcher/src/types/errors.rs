use std::fmt;

use ethers::types::{Address, SignatureError};
use tokio_tungstenite::tungstenite;

pub enum BatcherError {
    TcpListenerError(String),
    ConnectionError(tungstenite::Error),
    BatchVerifiedEventStreamError(String),
    EthereumSubscriptionError(String),
    SignatureError(SignatureError),
    BatchUploadError(String),
    TaskCreationError(String),
    ReceiptNotFoundError,
    TransactionSendError,
    MaxRetriesReachedError,
    SerializationError(String),
    GasPriceError,
    DisabledVerifiersError(String),
    BatchCostTooHigh,
    WsSinkEmpty,
    AddressNotFoundInUserStates(Address),
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
            BatcherError::TcpListenerError(e) => {
                write!(f, "TCP Listener error: {}", e)
            }
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
            BatcherError::BatchUploadError(e) => {
                write!(f, "Uploading Batch was not successful: {}", e)
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
            BatcherError::GasPriceError => {
                write!(f, "Gas price error")
            }
            BatcherError::BatchCostTooHigh => {
                write!(f, "No user in batch willing to pay the fee per proof. Checking again when another block arrives")
            }
            BatcherError::WsSinkEmpty => {
                write!(
                    f,
                    "Websocket sink was found empty. This should only happen in tests"
                )
            }
            BatcherError::AddressNotFoundInUserStates(addr) => {
                write!(
                    f,
                    "User with address {addr:?} was not found in Batcher user states cache"
                )
            }
            BatcherError::DisabledVerifiersError(reason) => {
                write!(
                    f,
                    "Error while trying to get disabled verifiers: {}",
                    reason
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum BatcherSendError {
    TransactionReverted(String),
    ReceiptNotFound,
    UnknownError(String),
}

impl From<BatcherSendError> for BatcherError {
    fn from(value: BatcherSendError) -> Self {
        match value {
            BatcherSendError::TransactionReverted(_) => BatcherError::TransactionSendError,
            BatcherSendError::ReceiptNotFound => BatcherError::ReceiptNotFoundError,
            BatcherSendError::UnknownError(err) => BatcherError::TaskCreationError(err),
        }
    }
}
