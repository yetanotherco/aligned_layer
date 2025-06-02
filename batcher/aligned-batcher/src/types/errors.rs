use std::fmt;

use ethers::types::{Address, Bytes, SignatureError};
use tokio_tungstenite::tungstenite;

pub enum TransactionSendError {
    NoProofSubmitters,
    NoFeePerProof,
    InsufficientFeeForAggregator,
    SubmissionInsufficientBalance,
    BatchAlreadySubmitted,
    InsufficientFunds,
    OnlyBatcherAllowed,
    Generic(String),
}

impl From<Bytes> for TransactionSendError {
    fn from(e: Bytes) -> Self {
        let byte_string = e.to_string();
        let str_code = if byte_string.len() >= 10 {
            &byte_string[..10] // Extract the error code only
        } else {
            "" // Not gonna match
        };
        match str_code {
            "0xc43ac290" => TransactionSendError::NoProofSubmitters, // can't happen, don't flush
            "0xa3a8658a" => TransactionSendError::NoFeePerProof,     // can't happen, don't flush
            "0x7899ec71" => TransactionSendError::InsufficientFeeForAggregator, // shouldn't happen, don't flush
            // returning the proofs and retrying later may help
            "0x3102f10c" => TransactionSendError::BatchAlreadySubmitted, // can happen, don't flush
            "0x5c54305e" => TransactionSendError::InsufficientFunds, // shouldn't happen, don't flush
            "0x152bc288" => TransactionSendError::OnlyBatcherAllowed, // won't happen, don't flush
            "0x4f779ceb" => TransactionSendError::SubmissionInsufficientBalance, // shouldn't happen,
            // flush can help if something went wrong
            _ => {
                // flush because unkown error
                TransactionSendError::Generic(format!("Unknown bytestring error: {}", byte_string))
            }
        }
    }
}

pub enum BatcherError {
    TcpListenerError(String),
    ConnectionError(tungstenite::Error),
    BatchVerifiedEventStreamError(String),
    EthereumSubscriptionError(String),
    SignatureError(SignatureError),
    BatchUploadError(String),
    TaskCreationError(String),
    ReceiptNotFoundError,
    TransactionSendError(TransactionSendError),
    MaxRetriesReachedError,
    SerializationError(String),
    GasPriceError,
    DisabledVerifiersError(String),
    BatchCostTooHigh,
    WsSinkEmpty,
    AddressNotFoundInUserStates(Address),
    QueueRemoveError(String),
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
            BatcherError::TransactionSendError(e) => {
                write!(f, "Error sending tx: {}", e)
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
            BatcherError::QueueRemoveError(e) => {
                write!(f, "Error while removing entry from queue: {}", e)
            }
        }
    }
}

impl fmt::Display for TransactionSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionSendError::NoProofSubmitters => {
                write!(f, "No proof submitter error")
            }
            TransactionSendError::NoFeePerProof => {
                write!(f, "No fee per proof")
            }
            TransactionSendError::InsufficientFeeForAggregator => {
                write!(f, "Insufficient fee for aggregator")
            }
            TransactionSendError::SubmissionInsufficientBalance => {
                write!(f, "Submission insufficient balance")
            }
            TransactionSendError::BatchAlreadySubmitted => {
                write!(f, "Batch already submitted")
            }
            TransactionSendError::InsufficientFunds => {
                write!(f, "Insufficient funds")
            }
            TransactionSendError::OnlyBatcherAllowed => {
                write!(f, "Only batcher allowed")
            }
            TransactionSendError::Generic(e) => {
                write!(f, "Generic error: {}", e)
            }
        }
    }
}
