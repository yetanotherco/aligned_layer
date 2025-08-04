use aligned_sdk::common::types::VerificationData;
use tracing::{info, warn};

#[derive(Debug)]
#[allow(dead_code)]
pub enum GetBatchProofsError {
    FetchingS3Batch(String),
    Deserialization(String),
    EmptyBody(String),
    StatusFailed((u16, String)),
    ReqwestClientFailed(String),
}

// needed to make S3 bucket work
const DEFAULT_USER_AGENT: &str = "proof-aggregator/aligned-layer";
const MAX_BATCH_URLS: usize = 5;

// get_aligned_batch_from_s3_with_multiple_urls tries multiple comma-separated URLs until first successful response
pub async fn get_aligned_batch_from_s3_with_multiple_urls(
    urls: String,
) -> Result<Vec<VerificationData>, GetBatchProofsError> {
    // Parse comma-separated URLs and limit to max 5
    let parsed_urls = parse_batch_urls(&urls);
    info!(
        "Getting batch from data service with {} URLs: {:?}",
        parsed_urls.len(),
        parsed_urls
    );

    let mut errors = Vec::new();

    // Try each URL until first successful response
    for url in parsed_urls.iter() {
        match get_aligned_batch_from_s3(url.clone()).await {
            Ok(data) => {
                return Ok(data);
            }
            Err(err) => {
                warn!("Failed to fetch batch from URL {}: {:?}", url, err);
                errors.push(format!("URL {}: {:?}", url, err));
            }
        }
    }

    // All URLs failed
    Err(GetBatchProofsError::FetchingS3Batch(format!(
        "Failed to get batch from all URLs, errors: {}",
        errors.join("; ")
    )))
}

// parse_batch_urls parses comma-separated URLs and limits to max 5
fn parse_batch_urls(batch_urls: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for url in batch_urls.split(',') {
        let trimmed_url = url.trim();
        if !trimmed_url.is_empty() {
            urls.push(trimmed_url.to_string());
            if urls.len() > MAX_BATCH_URLS {
                break;
            }
        }
    }
    urls
}

pub async fn get_aligned_batch_from_s3(
    url: String,
) -> Result<Vec<VerificationData>, GetBatchProofsError> {
    info!("Fetching batch from S3 URL: {}", url);
    let client = reqwest::Client::builder()
        .user_agent(DEFAULT_USER_AGENT)
        .build()
        .map_err(|e| GetBatchProofsError::ReqwestClientFailed(e.to_string()))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| GetBatchProofsError::FetchingS3Batch(e.to_string()))?;
    if !response.status().is_success() {
        return Err(GetBatchProofsError::StatusFailed((
            response.status().as_u16(),
            response
                .status()
                .canonical_reason()
                .unwrap_or("")
                .to_string(),
        )));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| GetBatchProofsError::EmptyBody(e.to_string()))?;
    let bytes: &[u8] = bytes.iter().as_slice();

    let data: Vec<VerificationData> = ciborium::from_reader(bytes)
        .map_err(|e| GetBatchProofsError::Deserialization(e.to_string()))?;

    Ok(data)
}
