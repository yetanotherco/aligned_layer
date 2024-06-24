use core::fmt;
use std::io;
use std::path::PathBuf;

pub enum SubmitError {
    ConnectionError(tokio_tungstenite::tungstenite::Error),
    SerdeError(serde_json::Error),
    MissingParameter(String),
    InvalidProvingSystem(String),
    EthError(String),
    IoError(PathBuf, io::Error),
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

impl fmt::Debug for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubmitError::MissingParameter(param) => write!(
                f,
                "Missing parameter: {} required for this proving system",
                param
            ),
            SubmitError::InvalidProvingSystem(proving_system) => {
                write!(f, "Invalid proving system: {}", proving_system)
            }
            SubmitError::ConnectionError(e) => {
                write!(f, "Web Socket Connection error: {}", e)
            }
            SubmitError::IoError(path, e) => {
                write!(f, "IO error for file: \"{}\", {}", path.display(), e)
            }
            SubmitError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            SubmitError::EthError(e) => write!(f, "Ethereum error: {}", e),
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
