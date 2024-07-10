use core::fmt;
use ethers::providers::ProviderError;
use ethers::signers::WalletError;
use std::io;
use std::path::PathBuf;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;

#[derive(Debug)]
pub enum AlignedError {
    SubmitError(SubmitError),
    VerificationError(VerificationError),
}

impl From<SubmitError> for AlignedError {
    fn from(e: SubmitError) -> Self {
        AlignedError::SubmitError(e)
    }
}

impl From<VerificationError> for AlignedError {
    fn from(e: VerificationError) -> Self {
        AlignedError::VerificationError(e)
    }
}

impl fmt::Display for AlignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlignedError::SubmitError(e) => write!(f, "Submit error: {}", e),
            AlignedError::VerificationError(e) => write!(f, "Verification error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum SubmitError {
    WebSocketConnectionError(tokio_tungstenite::tungstenite::Error),
    WebSocketClosedUnexpectedlyError(CloseFrame<'static>),
    IoError(PathBuf, io::Error),
    SerializationError(serde_json::Error),
    EthereumProviderError(String),
    HexDecodingError(String),
    WalletSignerError(String),
    MissingRequiredParameter(String),
    UnsupportedProvingSystem(String),
    InvalidEthereumAddress(String),
    ProtocolVersionMismatch { current: u16, expected: u16 },
    BatchVerifiedEventStreamError(String),
    BatchVerificationTimeout { timeout_seconds: u64 },
    NoResponseFromBatcher,
    UnexpectedBatcherResponse(String),
    EmptyVerificationDataCommitments,
    EmptyVerificationDataList,
    GenericError(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for SubmitError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        SubmitError::WebSocketConnectionError(e)
    }
}

impl From<serde_json::Error> for SubmitError {
    fn from(e: serde_json::Error) -> Self {
        SubmitError::SerializationError(e)
    }
}

impl From<ProviderError> for SubmitError {
    fn from(e: ProviderError) -> Self {
        SubmitError::EthereumProviderError(e.to_string())
    }
}

impl From<WalletError> for SubmitError {
    fn from(e: WalletError) -> Self {
        SubmitError::WalletSignerError(e.to_string())
    }
}

impl From<VerificationError> for SubmitError {
    fn from(e: VerificationError) -> Self {
        match e {
            VerificationError::HexDecodingError(e) => SubmitError::HexDecodingError(e.to_string()),
            VerificationError::EthereumProviderError(e) => SubmitError::EthereumProviderError(e),
            VerificationError::EthereumCallError(e) => SubmitError::EthereumProviderError(e),
        }
    }
}

impl fmt::Display for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubmitError::WebSocketConnectionError(e) => {
                write!(f, "WebSocket connection error: {}", e)
            }
            SubmitError::WebSocketClosedUnexpectedlyError(close_frame) => {
                write!(f, "WebSocket closed unexpectedly: {}", close_frame)
            }
            SubmitError::IoError(path, e) => write!(f, "IO error: {}: {}", path.display(), e),
            SubmitError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            SubmitError::EthereumProviderError(e) => write!(f, "Ethereum provider error: {}", e),
            SubmitError::HexDecodingError(e) => write!(f, "Hex decoding error: {}", e),
            SubmitError::WalletSignerError(e) => write!(f, "Wallet signer error: {}", e),
            SubmitError::MissingRequiredParameter(param) => {
                write!(f, "Missing required parameter: {}", param)
            }
            SubmitError::UnsupportedProvingSystem(proving_system) => {
                write!(f, "Unsupported proving system: {}", proving_system)
            }
            SubmitError::InvalidEthereumAddress(address) => {
                write!(f, "Invalid Ethereum address: {}", address)
            }
            SubmitError::ProtocolVersionMismatch { current, expected } => write!(
                f,
                "Protocol version mismatch: current={}, expected={}",
                current, expected
            ),
            SubmitError::BatchVerifiedEventStreamError(e) => {
                write!(f, "Batch verified event stream error: {}", e)
            }
            SubmitError::BatchVerificationTimeout { timeout_seconds } => {
                write!(
                    f,
                    "Batch verification timed out after {} seconds",
                    timeout_seconds
                )
            }
            SubmitError::NoResponseFromBatcher => write!(f, "No response received from batcher"),
            SubmitError::UnexpectedBatcherResponse(response) => {
                write!(f, "Unexpected batcher response: {}", response)
            }
            SubmitError::EmptyVerificationDataCommitments => {
                write!(f, "Verification data commitments are empty")
            }
            SubmitError::EmptyVerificationDataList => write!(f, "Verification data list is empty"),
            SubmitError::GenericError(e) => write!(f, "Generic error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum VerificationError {
    HexDecodingError(String),
    EthereumProviderError(String),
    EthereumCallError(String),
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationError::HexDecodingError(e) => write!(f, "Hex decoding error: {}", e),
            VerificationError::EthereumProviderError(e) => {
                write!(f, "Ethereum provider error: {}", e)
            }
            VerificationError::EthereumCallError(e) => write!(f, "Ethereum call error: {}", e),
        }
    }
}
