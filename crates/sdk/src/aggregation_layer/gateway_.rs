use crate::common::types::Network;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayReceipt {
    pub status: String,
    pub merkle_path: Vec<String>,
    pub nonce: i64,
    pub address: String,
}

pub struct SubmitSP1ProofMessage {
    nonce: u64,
    proof: Vec<u8>,
    program_vk: Vec<u8>,
    signature: Vec<u8>,
}

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

    pub fn sign(self) -> Self {
        self
    }
}
