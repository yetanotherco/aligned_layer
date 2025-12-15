use alloy::{hex, signers::Signer};
use reqwest::{multipart, Client};
use serde::de::DeserializeOwned;
use sp1_sdk::{SP1ProofWithPublicValues, SP1VerifyingKey};

use crate::{
    gateway::types::{
        EmptyDataResponse, GatewayResponse, NonceResponse, ReceiptsQueryParams, ReceiptsResponse,
        SubmitProofResponse, SubmitSP1ProofMessage,
    },
    types::Network,
};

pub struct AggregationModeGatewayProvider<S: Signer> {
    gateway_url: String,
    signer: Option<S>,
    http_client: Client,
    network: Network,
}

#[derive(Debug)]
pub enum GatewayError {
    Request(String),
    Api { status: u16, message: String },
    SignerNotConfigured,
    ProofSerialization(String),
    MessageSignature(String),
}

impl<S: Signer> AggregationModeGatewayProvider<S> {
    pub fn new(network: Network) -> Result<Self, GatewayError> {
        Ok(Self {
            gateway_url: network.gateway_url(),
            http_client: Client::new(),
            signer: None,
            network,
        })
    }

    pub fn new_with_signer(network: Network, signer: S) -> Result<Self, GatewayError> {
        Ok(Self {
            gateway_url: network.gateway_url(),
            http_client: Client::new(),
            signer: Some(signer),
            network,
        })
    }

    pub fn signer(&self) -> Option<&S> {
        self.signer.as_ref()
    }
}

impl<S: Signer> AggregationModeGatewayProvider<S> {
    pub async fn gateway_url(&self) -> &str {
        &self.gateway_url
    }

    pub async fn get_nonce_for(
        &self,
        address: String,
    ) -> Result<GatewayResponse<NonceResponse>, GatewayError> {
        let url = format!("{}/nonce/{}", self.gateway_url, address);
        self.send_request(self.http_client.get(url)).await
    }

    pub async fn get_receipts_for(
        &self,
        address: String,
        nonce: Option<u64>,
    ) -> Result<GatewayResponse<ReceiptsResponse>, GatewayError> {
        let query = ReceiptsQueryParams { address, nonce };

        let request = self
            .http_client
            .get(format!("{}/receipts", self.gateway_url))
            .query(&query);

        self.send_request(request).await
    }

    pub async fn submit_sp1_proof(
        &self,
        proof: &SP1ProofWithPublicValues,
        vk: &SP1VerifyingKey,
    ) -> Result<GatewayResponse<SubmitProofResponse>, GatewayError> {
        let serialized_proof = bincode::serialize(proof)
            .map_err(|e| GatewayError::ProofSerialization(e.to_string()))?;
        let serialized_vk =
            bincode::serialize(vk).map_err(|e| GatewayError::ProofSerialization(e.to_string()))?;

        let Some(signer) = &self.signer else {
            return Err(GatewayError::SignerNotConfigured);
        };
        let signer_address = signer.address().to_string();
        let nonce_response = self.get_nonce_for(signer_address).await?;
        let message =
            SubmitSP1ProofMessage::new(nonce_response.data.nonce, serialized_proof, serialized_vk)
                .sign(signer, &self.network)
                .await
                .map_err(GatewayError::MessageSignature)?;

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
    ) -> Result<GatewayResponse<T>, GatewayError> {
        let response = request
            .send()
            .await
            .map_err(|e| GatewayError::Request(e.to_string()))?;

        if !(200..300).contains(&response.status().as_u16()) {
            let payload: GatewayResponse<EmptyDataResponse> = response
                .json()
                .await
                .map_err(|e| GatewayError::Request(e.to_string()))?;

            return Err(GatewayError::Api {
                status: payload.status,
                message: payload.message,
            });
        }

        let payload: GatewayResponse<T> = response
            .json()
            .await
            .map_err(|e| GatewayError::Request(e.to_string()))?;

        Ok(payload)
    }
}
