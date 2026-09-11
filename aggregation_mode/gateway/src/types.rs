use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use db::types::TaskStatus;
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

/// Query parameters accepted by `GET /receipts`. Requires an address, and accepts a nonce
/// and a limit for the amount of tasks included in the query (the maximum value is 100).
/// Note: The limit value will only be taken into account if nonce is None.
#[derive(Deserialize, Clone)]
pub(super) struct GetReceiptsQueryParams {
    pub address: String,
    pub nonce: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, MultipartForm)]
pub(super) struct SubmitProofRequestSP1 {
    pub nonce: Text<u64>,
    pub proof: TempFile,
    pub program_vk: TempFile,
    pub signature_hex: Text<String>,
}

#[derive(Debug, MultipartForm)]
pub(super) struct SubmitProofRequestRisc0 {
    pub _nonce: Text<u64>,
    pub _risc0_receipt: TempFile,
    pub _program_image_id_hex: Text<String>,
    pub _signature_hex: Text<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, sqlx::Type, serde::Serialize)]
pub struct GetReceiptsResponse {
    pub status: TaskStatus,
    pub merkle_path: Vec<String>,
    pub nonce: i64,
    pub address: String,
}

#[derive(Debug, Clone, sqlx::FromRow, sqlx::Type, serde::Serialize)]
pub struct Receipt {
    pub status: TaskStatus,
    pub merkle_path: Option<Vec<u8>>,
    pub nonce: i64,
    pub address: String,
}
