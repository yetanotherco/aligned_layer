use alloy::network::EthereumWallet;
use reqwest::{multipart, Client};
use serde::de::DeserializeOwned;

use crate::{
    aggregation_layer::gateway::types::{
        GatewayResponse, NonceResponse, Receipt, ReceiptsQuery, ReceiptsResponse,
        SubmitProofResponse, SubmitSP1ProofMessage,
    },
    common::types::Network,
};

pub struct AggregationModeGatewayProvider {
    gateway_url: String,
    signer: Option<EthereumWallet>,
    http_client: Client,
}

#[derive(Debug)]
pub enum AggregationModeError {
    UnsupportedNetwork,
    Request(String),
    Api { status: u16, message: String },
    SignerNotConfigured,
}

impl AggregationModeGatewayProvider {
    pub fn new(network: Network) -> Result<Self, AggregationModeError> {
        let gateway_url = match network {
            Network::Devnet => "http://127.0.0.1:8089".into(),

            _ => return Err(AggregationModeError::UnsupportedNetwork),
        };

        Ok(Self {
            gateway_url,
            http_client: Client::new(),
            signer: None,
        })
    }

    pub fn new_with_signer(
        network: Network,
        signer: EthereumWallet,
    ) -> Result<Self, AggregationModeError> {
        let gateway_url = match network {
            Network::Devnet => "http://127.0.0.1:8089".into(),
            _ => return Err(AggregationModeError::UnsupportedNetwork),
        };

        Ok(Self {
            gateway_url,
            http_client: Client::new(),
            signer: Some(signer),
        })
    }

    pub fn signer(&self) -> Option<&EthereumWallet> {
        self.signer.as_ref()
    }
}

impl AggregationModeGatewayProvider {
    pub async fn gateway_url(&self) -> &str {
        &self.gateway_url
    }

    pub async fn get_nonce_for(&self, address: String) -> Result<u64, AggregationModeError> {
        let url = format!("{}/nonce/{}", self.gateway_url, address);
        let response: NonceResponse = self.send_request(self.http_client.get(url)).await?;

        Ok(response.nonce)
    }

    pub async fn get_receipts_for(
        &self,
        address: String,
        nonce: Option<u64>,
    ) -> Result<Vec<Receipt>, AggregationModeError> {
        let query = ReceiptsQuery {
            address: address,
            nonce,
        };

        let request = self
            .http_client
            .get(format!("{}/receipts", self.gateway_url))
            .query(&query);

        let response: ReceiptsResponse = self.send_request(request).await?;

        Ok(response.receipts)
    }

    pub async fn submit_sp1_proof(
        &self,
        serialized_proof: Vec<u8>,
        serialized_vk: Vec<u8>,
    ) -> Result<SubmitProofResponse, AggregationModeError> {
        let Some(signer) = &self.signer else {
            return Err(AggregationModeError::SignerNotConfigured);
        };
        let signer_address = signer.default_signer().address().to_string();

        let nonce = self.get_nonce_for(signer_address).await?;
        let message = SubmitSP1ProofMessage::new(nonce, serialized_proof, serialized_vk).sign();
        let form = multipart::Form::new()
            .text("nonce", message.nonce.to_string())
            .part(
                "proof",
                multipart::Part::bytes(message.proof).file_name("proof.bin"),
            )
            .part(
                "program_vk",
                multipart::Part::bytes(message.program_vk).file_name("program_vk.bin"),
            )
            .text("signature_hex", hex::encode(message.signature));

        let request = self
            .http_client
            .post(format!("{}/proof/sp1", self.gateway_url))
            .multipart(form);

        self.send_request(request).await
    }

    // TODO: verify proof from receipt merkle path

    async fn send_request<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, AggregationModeError> {
        let response = request
            .send()
            .await
            .map_err(|e| AggregationModeError::Request(e.to_string()))?;

        let payload: GatewayResponse<T> = response
            .json()
            .await
            .map_err(|e| AggregationModeError::Request(e.to_string()))?;

        if payload.status != 200 {
            return Err(AggregationModeError::Api {
                status: payload.status,
                message: payload.message,
            });
        }

        Ok(payload.data)
    }
}
