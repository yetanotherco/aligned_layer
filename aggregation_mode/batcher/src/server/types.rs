use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::db::TaskStatus;

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

/// Query parameters accepted by `GET /proof/merkle`, containing an optional proof id.
#[derive(Deserialize, Clone)]
pub(super) struct GetReceiptsQueryParams {
    pub address: String,
    pub nonce: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SubmitProofRequest<T> {
    pub nonce: u64,
    pub message: T,
    pub signature: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SubmitProofRequestMessageSP1 {
    pub proof: Vec<u8>,
    pub program_vk_commitment: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SubmitProofRequestMessageRisc0 {
    pub proof: Vec<u8>,
    pub program_image_id: Vec<u8>,
    pub public_inputs: Vec<u8>,
}

#[derive(Debug, Clone, sqlx::FromRow, sqlx::Type, serde::Serialize)]
pub struct GetReceiptsResponse {
    pub status: TaskStatus,
    pub merkle_path: Vec<String>,
    pub nonce: i64,
    pub address: String,
}
