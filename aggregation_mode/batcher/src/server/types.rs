use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
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

/// Query parameters accepted by `GET /proof/merkle`, containing an optional proof id.
#[derive(Deserialize, Clone)]
pub(super) struct GetProofMerklePathQueryParams {
    pub id: Option<String>,
}

#[derive(Debug, MultipartForm)]
pub(super) struct SubmitProofRequestSP1 {
    pub nonce: Text<u64>,
    pub proof: TempFile,
    pub program_vk: TempFile,
    pub _signature_hex: Text<String>,
}

#[derive(Debug, MultipartForm)]
pub(super) struct SubmitProofRequestRisc0 {
    pub _nonce: Text<u64>,
    pub _risc0_receipt: TempFile,
    pub _program_image_id_hex: Text<String>,
    pub _signature_hex: Text<String>,
}
