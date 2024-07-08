use core::fmt;
use ethers::providers::ProviderError;
use ethers::signers::WalletError;
use ethers::utils::hex::FromHexError;
use std::io;
use std::path::PathBuf;

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

impl fmt::Debug for AlignedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlignedError::SubmitError(e) => write!(f, "Submit error: {:?}", e),
            AlignedError::VerificationError(e) => write!(f, "Verification error: {:?}", e),
        }
    }
}

pub enum SubmitError {
    ConnectionError(tokio_tungstenite::tungstenite::Error),
    IoError(PathBuf, io::Error),
    SerdeError(serde_json::Error),
    EthError(String),
    SignerError(String),
    MissingParameter(String),
    InvalidProvingSystem(String),
    InvalidAddress(String, String),
    ProtocolVersionMismatch(u16, u16),
    BatchVerifiedEventStreamError(String),
    GenericError(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for SubmitError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        SubmitError::ConnectionError(e)
    }
}

impl From<serde_json::Error> for SubmitError {
    fn from(e: serde_json::Error) -> Self {
        SubmitError::SerdeError(e)
    }
}

impl From<ProviderError> for SubmitError {
    fn from(e: ProviderError) -> Self {
        SubmitError::EthError(e.to_string())
    }
}

impl From<WalletError> for SubmitError {
    fn from(e: WalletError) -> Self {
        SubmitError::SignerError(e.to_string())
    }
}

impl From<FromHexError> for SubmitError {
    fn from(e: FromHexError) -> Self {
        SubmitError::EthError(e.to_string())
    }
}

impl From<VerificationError> for SubmitError {
    fn from(e: VerificationError) -> Self {
        match e {
            VerificationError::ParsingError(e) => SubmitError::GenericError(e),
            VerificationError::EthError(e) => SubmitError::EthError(e),
        }
    }
}

impl fmt::Debug for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubmitError::MissingParameter(param) => write!(
                f,
                "Missing parameter: {} required for this proving system",
                param
            ),
            SubmitError::ConnectionError(e) => {
                write!(f, "Web Socket Connection error: {}", e)
            }
            SubmitError::IoError(path, e) => {
                write!(f, "IO error for file: \"{}\", {}", path.display(), e)
            }
            SubmitError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            SubmitError::EthError(e) => write!(f, "Ethereum error: {}", e),
            SubmitError::SignerError(e) => write!(f, "Signer error: {}", e),
            SubmitError::InvalidProvingSystem(proving_system) => {
                write!(f, "Invalid proving system: {}", proving_system)
            }
            SubmitError::InvalidAddress(addr, msg) => {
                write!(f, "Invalid address: {}, {}", addr, msg)
            }
            SubmitError::ProtocolVersionMismatch(current, expected) => {
                write!(f, "Protocol version mismatch, SDK should be updated: current version: {} != expected version: {}", current, expected)
            }
            SubmitError::BatchVerifiedEventStreamError(e) => {
                write!(f, "`BatchVerified` event stream error: {}", e)
            }
            SubmitError::GenericError(e) => write!(f, "Generic error: {}", e),
        }
    }
}

impl fmt::Display for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubmitError::MissingParameter(param) => write!(
                f,
                "Missing parameter: {} required for this proving system",
                param
            ),
            SubmitError::ConnectionError(e) => {
                write!(f, "Web Socket Connection error: {}", e)
            }
            SubmitError::IoError(path, e) => {
                write!(f, "IO error for file: \"{}\", {}", path.display(), e)
            }
            SubmitError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            SubmitError::EthError(e) => write!(f, "Ethereum error: {}", e),
            SubmitError::SignerError(e) => write!(f, "Signer error: {}", e),
            SubmitError::InvalidProvingSystem(proving_system) => {
                write!(f, "Invalid proving system: {}", proving_system)
            }
            SubmitError::InvalidAddress(addr, msg) => {
                write!(f, "Invalid address: {}, {}", addr, msg)
            }
            SubmitError::ProtocolVersionMismatch(current, expected) => {
                write!(f, "Protocol version mismatch, SDK should be updated: current version: {} != expected version: {}", current, expected)
            }
            SubmitError::BatchVerifiedEventStreamError(e) => {
                write!(f, "`BatchVerified` event stream error: {}", e)
            }
            SubmitError::GenericError(e) => write!(f, "Generic error: {}", e),
        }
    }
}

pub enum VerificationError {
    ParsingError(String),
    EthError(String),
}

impl fmt::Debug for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationError::ParsingError(e) => write!(f, "Parsing error: {}", e),
            VerificationError::EthError(e) => write!(f, "Ethereum error: {}", e),
        }
    }
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationError::ParsingError(e) => write!(f, "Parsing error: {}", e),
            VerificationError::EthError(e) => write!(f, "Ethereum error: {}", e),
        }
    }
}
