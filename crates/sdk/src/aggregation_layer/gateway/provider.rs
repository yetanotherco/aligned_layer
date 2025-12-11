use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    aggregation_layer::gateway::types::{
        GatewayResponse, NonceResponse, Receipt, ReceiptsQuery, ReceiptsResponse,
    },
    common::types::Network,
};

pub struct AggregationModeGatewayProvider {
    gateway_url: String,
    http_client: Client,
}

#[derive(Debug)]
pub enum AggregationModeError {
    UnsupportedNetwork,
    Request(String),
    Api { status: u16, message: String },
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
        })
    }

    pub fn new_with_signer() {}

    pub fn signer() {}
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

    pub async fn submit_sp1_proof(&self, serialized_proof: Vec<u8>, serialized_vk: Vec<u8>) {}

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
