use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(super) struct GatewayResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub(super) struct NonceResponse {
    pub nonce: u64,
}

#[derive(Debug, Serialize)]
pub struct ReceiptsQuery {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Receipt {
    pub status: String,
    pub merkle_path: Vec<String>,
    pub nonce: i64,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ReceiptsResponse {
    pub receipts: Vec<Receipt>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitSP1ProofMessage {
    pub nonce: u64,
    pub proof: Vec<u8>,
    pub program_vk: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitProofResponse {
    pub task_id: String,
}

impl SubmitSP1ProofMessage {
    pub fn new(nonce: u64, serialized_proof: Vec<u8>, serialized_vk: Vec<u8>) -> Self {
        Self {
            nonce,
            proof: serialized_proof,
            program_vk: serialized_vk,
            signature: vec![],
        }
    }

    pub fn sign(mut self) -> Self {
        self
    }
}
