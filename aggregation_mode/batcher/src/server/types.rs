use aligned_sdk::aggregation_layer::AggregationModeProvingSystem;
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

// TODO: move this to the sdk once ready

#[derive(Deserialize, Clone)]
pub(super) struct ProofMerkleQuery {
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SubmitProofRequest {
    pub message: SubmitProofRequestMessage,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct SubmitProofRequestMessage {
    pub nonce: u64,
    pub proving_system_id: AggregationModeProvingSystem,
    pub proof: Vec<u8>,
    pub public_inputs: Option<Vec<u8>>,
    pub program_id: Vec<u8>,
}
