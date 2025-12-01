use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub(super) struct AppResponse {
    status: u16,
    message: String,
    data: Value,
}

impl AppResponse {
    pub(super) fn new_sucessfull(data: Value) -> Self {
        Self {
            status: 200,
            message: "Ok".to_string(),
            data,
        }
    }

    pub(super) fn new_unsucessfull(message: &str, status: u16) -> Self {
        Self {
            status,
            message: message.to_string(),
            data: serde_json::json!({}),
        }
    }
}

#[derive(Deserialize, Clone)]
pub(super) struct ProofMerkleQuery {
    pub id: Option<String>,
}
