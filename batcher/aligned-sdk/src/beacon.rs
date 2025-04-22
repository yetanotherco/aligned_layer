use std::str::FromStr;

use ethers::core::k256::sha2::{Digest, Sha256};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// See https://eips.ethereum.org/EIPS/eip-4844#parameters
pub const KZG_VERSIONED_HASH: u8 = 0x1;

pub struct BeaconClient {
    beacon_client_url: String,
    api_client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum BeaconAPIResponse {
    Success { data: Value },
    Error { code: u64, message: String },
}

#[derive(Debug)]
pub enum BeaconClientError {
    Url(url::ParseError),
    ReqwestError(reqwest::Error),
    APIError { code: u64, message: String },
    Deserialization(serde_json::Error),
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
// https://ethereum.github.io/beacon-APIs/#/Beacon/getBlobSidecars
pub struct BlobData {
    pub index: String,
    pub blob: String,
    pub kzg_commitment: String,
    pub kzg_proof: String,
    pub kzg_commitment_inclusion_proof: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
// https://ethereum.github.io/beacon-APIs/#/Beacon/getBlockHeaders
pub struct BeaconBlock {
    pub root: String,
    pub canonical: bool,
    pub header: BeaconBlockHeader,
}

#[derive(Deserialize, Debug)]

pub struct BeaconBlockHeader {
    pub message: BeaconBlockMessage,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct BeaconBlockMessage {
    pub slot: String,
    pub proposer_index: String,
    pub parent_root: String,
    pub state_root: String,
    pub body_root: String,
}

impl BeaconClient {
    pub fn new(beacon_client_url: String) -> Self {
        Self {
            api_client: Client::new(),
            beacon_client_url,
        }
    }

    pub async fn get_block_header_from_parent_hash(
        &self,
        parent_block_hash: [u8; 32],
    ) -> Result<Option<BeaconBlock>, BeaconClientError> {
        let parent_block_hash_hex = format!("0x{}", hex::encode(parent_block_hash));
        let data = self
            .beacon_get(&format!(
                "/eth/v1/beacon/headers?parent_root={}",
                parent_block_hash_hex
            ))
            .await?;

        let res =
            Vec::<BeaconBlock>::deserialize(data).map_err(BeaconClientError::Deserialization)?;

        let block = res
            .into_iter()
            .find(|block| block.header.message.parent_root == parent_block_hash_hex);

        Ok(block)
    }

    pub async fn get_blobs_from_slot(&self, slot: u64) -> Result<Vec<BlobData>, BeaconClientError> {
        let data = self
            .beacon_get(&format!("/eth/v1/beacon/blob_sidecars/{}", slot))
            .await?;

        Vec::<BlobData>::deserialize(data).map_err(BeaconClientError::Deserialization)
    }

    pub async fn get_blob_by_versioned_hash(
        &self,
        slot: u64,
        blob_versioned_hash: [u8; 32],
    ) -> Result<Option<BlobData>, BeaconClientError> {
        let res = self.get_blobs_from_slot(slot).await?;

        let blob = res.into_iter().find(|blob| {
            let kzg_commitment_bytes =
                hex::decode(blob.kzg_commitment.replace("0x", "")).expect("A valid commitment");

            let mut hasher = Sha256::new();
            hasher.update(&kzg_commitment_bytes);
            let mut versioned_hash: [u8; 32] = hasher.finalize().into();
            versioned_hash[0] = KZG_VERSIONED_HASH;

            versioned_hash == blob_versioned_hash
        });

        Ok(blob)
    }

    async fn beacon_get(&self, path: &str) -> Result<Value, BeaconClientError> {
        let url = Url::from_str(&format!("{}{}", self.beacon_client_url, path))
            .map_err(BeaconClientError::Url)?;
        let req = self
            .api_client
            .get(url)
            .header("content-type", "application/json")
            .header("accept", "application/json");

        let res = req.send().await.map_err(BeaconClientError::ReqwestError)?;
        let beacon_response = res.json().await.map_err(BeaconClientError::ReqwestError)?;

        match beacon_response {
            BeaconAPIResponse::Success { data } => Ok(data),
            BeaconAPIResponse::Error { code, message } => {
                Err(BeaconClientError::APIError { code, message })
            }
        }
    }
}
