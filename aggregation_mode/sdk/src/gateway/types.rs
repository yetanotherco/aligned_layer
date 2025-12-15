use alloy::{
    dyn_abi::DynSolValue,
    primitives::{keccak256, Keccak256, U256},
    signers::Signer,
};
use serde::{Deserialize, Serialize};

use crate::types::Network;

#[derive(Debug, Deserialize)]
pub struct GatewayResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub(super) struct EmptyDataResponse {}

#[derive(Debug, Deserialize)]
pub struct NonceResponse {
    pub nonce: u64,
}

#[derive(Debug, Serialize)]
pub struct ReceiptsQueryParams {
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
pub struct ReceiptsResponse {
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

    fn hash_msg(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        let mut output = [0u8; 32];

        let nonce_bytes: [u8; 32] = U256::from_be_slice(&self.nonce.to_be_bytes()).to_be_bytes();

        hasher.update(nonce_bytes);
        hasher.update(&self.proof);
        hasher.update(&self.program_vk);
        hasher.finalize_into_array(&mut output);
        output
    }

    pub fn eip712_hash(&self, network: &Network) -> [u8; 32] {
        let domain_value = DynSolValue::Tuple(vec![
            DynSolValue::String("Aligned".to_string()),
            DynSolValue::String("1".to_string()),
            DynSolValue::Uint(U256::from(network.chain_id()), 256),
        ]);

        let message_value = DynSolValue::Tuple(vec![
            DynSolValue::FixedBytes(self.hash_msg().into(), 32),
            DynSolValue::Uint(U256::from(self.nonce), 256),
        ]);

        let encoded_domain = domain_value.abi_encode();
        let encoded_message = message_value.abi_encode();

        let domain_separator = keccak256(&encoded_domain);
        let message_hash = keccak256(&encoded_message);

        keccak256([&[0x19, 0x01], &domain_separator[..], &message_hash[..]].concat()).0
    }

    pub async fn sign<S: Signer>(mut self, signer: &S, network: &Network) -> Result<Self, String> {
        let signature = signer
            .sign_hash(&self.eip712_hash(network).into())
            .await
            .map_err(|e| e.to_string())?;

        self.signature = signature.as_bytes().to_vec();

        Ok(self)
    }
}
