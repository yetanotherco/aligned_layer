use core::fmt;
use std::io;
use std::path::PathBuf;

use crate::hex::FromHexError;
use ethers::providers::ProviderError;
use ethers::signers::WalletError;

pub enum BatcherClientError {
    MissingParameter(String),
    InvalidUrl(url::ParseError, String),
    InvalidProvingSystem(String),
    ConnectionError(tokio_tungstenite::tungstenite::Error),
    IoError(PathBuf, io::Error),
    SerdeError(serde_json::Error),
    EthError(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for BatcherClientError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        BatcherClientError::ConnectionError(e)
    }
}

impl From<serde_json::Error> for BatcherClientError {
    fn from(e: serde_json::Error) -> Self {
        BatcherClientError::SerdeError(e)
    }
}

impl From<ProviderError> for BatcherClientError {
    fn from(e: ProviderError) -> Self {
        BatcherClientError::EthError(e.to_string())
    }
}

impl From<WalletError> for BatcherClientError {
    fn from(e: WalletError) -> Self {
        BatcherClientError::EthError(e.to_string())
    }
}

impl From<FromHexError> for BatcherClientError {
    fn from(e: FromHexError) -> Self {
        BatcherClientError::EthError(e.to_string())
    }
}

impl fmt::Debug for BatcherClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BatcherClientError::MissingParameter(param) => write!(
                f,
                "Missing parameter: {} required for this proving system",
                param
            ),
            BatcherClientError::InvalidUrl(err, url) => {
                write!(f, "Invalid URL \"{}\", {}", url, err)
            }
            BatcherClientError::InvalidProvingSystem(proving_system) => {
                write!(f, "Invalid proving system: {}", proving_system)
            }
            BatcherClientError::ConnectionError(e) => {
                write!(f, "Web Socket Connection error: {}", e)
            }
            BatcherClientError::IoError(path, e) => {
                write!(f, "IO error for file: \"{}\", {}", path.display(), e)
            }
            BatcherClientError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            BatcherClientError::EthError(e) => write!(f, "Ethereum error: {}", e),
        }
    }
}
