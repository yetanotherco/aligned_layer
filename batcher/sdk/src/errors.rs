
pub enum SubmitError {
    // Error variants here
    ConnectionError(tokio_tungstenite::tungstenite::Error),
    SerdeError(serde_json::Error),
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
