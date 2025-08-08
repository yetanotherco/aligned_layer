use crate::backend::retry::{retry_function, RetryError};
use aligned_sdk::common::types::VerificationData;
use std::time::Duration;
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

// Retry parameters for S3 requests
/// Initial delay before first retry attempt (in milliseconds)
const RETRY_MIN_DELAY_MILLIS: u64 = 500;
/// Exponential backoff multiplier for retry delays
const RETRY_FACTOR: f32 = 2.0;
/// Maximum number of retry attempts
const RETRY_MAX_TIMES: usize = 5;
/// Maximum delay between retry attempts (in seconds)
const RETRY_MAX_DELAY_SECONDS: u64 = 10;

/// Timeout for establishing a connection to S3
const CONNECT_TIMEOUT_SECONDS: Duration = Duration::from_secs(10);
/// Timeout for Batch Download Requests
const BATCH_DOWNLOAD_TIMEOUT_SECONDS: Duration = Duration::from_secs(5 * 60);

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

async fn get_aligned_batch_from_s3_retryable(
    url: String,
) -> Result<Vec<VerificationData>, RetryError<GetBatchProofsError>> {
    info!("Fetching batch from S3 URL: {}", url);
    let client = reqwest::Client::builder()
        .user_agent(DEFAULT_USER_AGENT)
        .connect_timeout(CONNECT_TIMEOUT_SECONDS)
        .timeout(BATCH_DOWNLOAD_TIMEOUT_SECONDS)
        .build()
        .map_err(|e| {
            RetryError::Permanent(GetBatchProofsError::ReqwestClientFailed(e.to_string()))
        })?;

    let response = client.get(&url).send().await.map_err(|e| {
        warn!("Failed to send request to {}: {}", url, e);
        RetryError::Transient(GetBatchProofsError::FetchingS3Batch(e.to_string()))
    })?;

    if !response.status().is_success() {
        let status_code = response.status().as_u16();
        let reason = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();

        // Determine if the error is retryable based on status code
        let error = GetBatchProofsError::StatusFailed((status_code, reason));
        return match status_code {
            // Client errors (4xx) are generally permanent, except for specific cases
            400..=499 => match status_code {
                408 | 429 => Err(RetryError::Transient(error)), // Request Timeout, Too Many Requests
                _ => Err(RetryError::Permanent(error)),
            },
            // Server errors (5xx) are generally transient
            500..=599 => Err(RetryError::Transient(error)),
            _ => Err(RetryError::Permanent(error)),
        };
    }

    let bytes = response.bytes().await.map_err(|e| {
        warn!("Failed to read response body from {}: {}", url, e);
        RetryError::Transient(GetBatchProofsError::EmptyBody(e.to_string()))
    })?;
    let bytes: &[u8] = bytes.iter().as_slice();

    let data: Vec<VerificationData> = ciborium::from_reader(bytes).map_err(|e| {
        warn!("Failed to deserialize batch data from {}: {}", url, e);
        RetryError::Permanent(GetBatchProofsError::Deserialization(e.to_string()))
    })?;

    Ok(data)
}

/// Download batch from Storage Service using the provided URL.
///
/// Retries on recoverable errors using exponential backoff up to `RETRY_MAX_TIMES` times:
/// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs).
pub async fn get_aligned_batch_from_s3(
    url: String,
) -> Result<Vec<VerificationData>, GetBatchProofsError> {
    let url_clone = url.clone();
    retry_function(
        move || {
            let url = url_clone.clone();
            get_aligned_batch_from_s3_retryable(url)
        },
        RETRY_MIN_DELAY_MILLIS,
        RETRY_FACTOR,
        RETRY_MAX_TIMES,
        RETRY_MAX_DELAY_SECONDS,
    )
    .await
    .map_err(|retry_err| retry_err.inner())
}
