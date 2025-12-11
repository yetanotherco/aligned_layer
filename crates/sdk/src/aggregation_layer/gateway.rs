use crate::common::types::Network;

pub struct AggregationModeGatewayProvider {
    gateway_url: String,
}

pub enum AggregationModeError {
    UnsupportedNetwork,
}

impl AggregationModeGatewayProvider {
    pub fn new(network: Network) -> Result<Self, AggregationModeError> {
        let provider = match network {
            Network::Devnet => Self {
                gateway_url: "http://127.0.0.1:8089".into(),
            },
            _ => return Err(AggregationModeError::UnsupportedNetwork),
        };

        Ok(provider)
    }

    pub fn new_with_signer() {}

    pub fn signer() {}
}

impl AggregationModeGatewayProvider {
    pub async fn get_connection_url(&self) {}

    pub async fn get_nonce_for(&self, address: String) {}

    pub async fn get_receipts_for(&self, address: String, nonce: Option<u64>) {}

    pub async fn submit_sp1_proof(&self, serialized_proof: Vec<u8>, serialized_vk: Vec<u8>) {}

    // TODO: verify proof from receipt merkle path
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
