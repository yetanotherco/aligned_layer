use core::fmt;
use ethers::providers::ProviderError;
use ethers::signers::WalletError;
use ethers::types::transaction::eip712::Eip712Error;
use ethers::types::{SignatureError, H160};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;

use crate::communication::serialization::SerializationError;

use super::types::ProofInvalidReason;

#[derive(Debug)]
pub enum AlignedError {
    SubmitError(SubmitError),
    VerificationError(VerificationError),
    ChainIdError(ChainIdError),
    FeeEstimateError(FeeEstimateError),
    FileError(FileError),
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

impl From<ChainIdError> for AlignedError {
    fn from(e: ChainIdError) -> Self {
        AlignedError::ChainIdError(e)
    }
}

impl From<FeeEstimateError> for AlignedError {
    fn from(e: FeeEstimateError) -> Self {
        AlignedError::FeeEstimateError(e)
    }
}

impl From<FileError> for AlignedError {
    fn from(e: FileError) -> Self {
        AlignedError::FileError(e)
    }
}

impl fmt::Display for AlignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlignedError::SubmitError(e) => write!(f, "Submit error: {}", e),
            AlignedError::VerificationError(e) => write!(f, "Verification error: {}", e),
            AlignedError::ChainIdError(e) => write!(f, "Chain ID error: {}", e),
            AlignedError::FeeEstimateError(e) => write!(f, "Fee estimate error: {}", e),
            AlignedError::FileError(e) => write!(f, "File error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum SubmitError {
    WebSocketConnectionError(tokio_tungstenite::tungstenite::Error),
    WebSocketClosedUnexpectedlyError(CloseFrame<'static>),
    IoError(PathBuf, io::Error),
    SerializationError(SerializationError),
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
    InvalidNonce,
    InvalidMaxFee,
    ProofQueueFlushed,
    InvalidSignature,
    InvalidChainId,
    InvalidProof(ProofInvalidReason),
    ProofTooLarge,
    InvalidReplacementMessage,
    InsufficientBalance(H160),
    InvalidPaymentServiceAddress(H160, H160),
    BatchSubmissionFailed(String),
    AddToBatchError,
    InvalidProofInclusionData,
    GetNonceError(String),
    GenericError(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for SubmitError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        SubmitError::WebSocketConnectionError(e)
    }
}

impl From<SerializationError> for SubmitError {
    fn from(e: SerializationError) -> Self {
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
            VerificationError::EthereumNotAContract(address) => {
                SubmitError::InvalidEthereumAddress(address.to_string())
            }
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
            SubmitError::InvalidNonce => write!(f, "Invalid nonce"),
            SubmitError::InvalidMaxFee => write!(f, "Invalid max fee"),
            SubmitError::BatchSubmissionFailed(merkle_root) => write!(
                f,
                "Could not create task with batch merkle root {}",
                merkle_root
            ),
            SubmitError::GenericError(e) => write!(f, "Generic error: {}", e),
            SubmitError::InvalidSignature => write!(f, "Invalid Signature"),
            SubmitError::InvalidChainId => write!(f, "Invalid chain Id"),
            SubmitError::InvalidProof(reason) => write!(f, "Invalid proof {}", reason),
            SubmitError::ProofTooLarge => write!(f, "Proof too Large"),
            SubmitError::InvalidReplacementMessage => write!(f, "Invalid replacement message"),
            SubmitError::InsufficientBalance(addr) => {
                write!(f, "Insufficient balance, address: {}", addr)
            }
            SubmitError::InvalidPaymentServiceAddress(received_addr, expected_addr) => {
                write!(
                    f,
                    "Invalid payment service address, received: {}, expected: {}",
                    received_addr, expected_addr
                )
            }
            SubmitError::ProofQueueFlushed => write!(f, "Batch reset"),
            SubmitError::AddToBatchError => write!(f, "Error while adding entry to batch"),
            SubmitError::InvalidProofInclusionData => {
                write!(f, "Batcher responded with invalid batch inclusion data. Can't verify your proof was correctly included in the batch.")
            }
            SubmitError::GetNonceError(e) => write!(f, "Error while getting nonce {}", e),
        }
    }
}

#[derive(Debug)]
pub enum VerificationError {
    HexDecodingError(String),
    EthereumProviderError(String),
    EthereumCallError(String),
    EthereumNotAContract(H160),
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationError::HexDecodingError(e) => write!(f, "Hex decoding error: {}", e),
            VerificationError::EthereumProviderError(e) => {
                write!(f, "Ethereum provider error: {}", e)
            }
            VerificationError::EthereumCallError(e) => write!(f, "Ethereum call error: {}", e),
            VerificationError::EthereumNotAContract(address) => {
                write!(f, "Address {} does not contain a contract", address)
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GetNonceError {
    EthRpcError(String),
    ConnectionFailed(String),
    SerializationError(String),
    UnexpectedResponse(String),
    InvalidRequest(String),
    ProtocolMismatch { current: u16, expected: u16 },
}

#[derive(Debug)]
pub enum ChainIdError {
    EthereumProviderError(String),
    EthereumCallError(String),
}

impl fmt::Display for ChainIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChainIdError::EthereumProviderError(e) => {
                write!(f, "Ethereum provider error: {}", e)
            }
            ChainIdError::EthereumCallError(e) => write!(f, "Ethereum call error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum FeeEstimateError {
    EthereumProviderError(String),
    EthereumGasPriceError(String),
    FeeEstimateParseError(String),
}

impl fmt::Display for FeeEstimateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FeeEstimateError::EthereumProviderError(e) => {
                write!(f, "Ethereum provider error: {}", e)
            }
            FeeEstimateError::EthereumGasPriceError(e) => {
                write!(f, "Failed to retreive the current gas price: {}", e)
            }
            FeeEstimateError::FeeEstimateParseError(e) => {
                write!(f, "Error parsing PriceEstimate: {}", e)
            }
        }
    }
}

#[derive(Debug)]
pub enum VerifySignatureError {
    RecoverTypedDataError(SignatureError),
    EncodeError(Eip712Error),
}

impl From<SignatureError> for VerifySignatureError {
    fn from(e: SignatureError) -> Self {
        VerifySignatureError::RecoverTypedDataError(e)
    }
}

impl From<Eip712Error> for VerifySignatureError {
    fn from(e: Eip712Error) -> Self {
        VerifySignatureError::EncodeError(e)
    }
}

impl fmt::Display for VerifySignatureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerifySignatureError::RecoverTypedDataError(e) => {
                write!(f, "Recover typed data error: {}", e)
            }
            VerifySignatureError::EncodeError(e) => write!(f, "Encode error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum PaymentError {
    SendError(String),
    SubmitError(String),
    PaymentFailed,
}

#[derive(Debug)]
pub enum BalanceError {
    EthereumProviderError(String),
    EthereumCallError(String),
}

#[derive(Debug)]
pub enum FileError {
    IoError(PathBuf, io::Error),
    SerializationError(SerializationError),
}

impl From<SerializationError> for FileError {
    fn from(e: SerializationError) -> Self {
        FileError::SerializationError(e)
    }
}

impl From<io::Error> for FileError {
    fn from(e: io::Error) -> Self {
        FileError::IoError(PathBuf::new(), e)
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::IoError(path, e) => write!(f, "IO error: {}: {}", path.display(), e),
            FileError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}
