use aligned_sdk::core::types::VerificationData;

#[derive(Debug)]
pub enum GetBatchProofsError {
    FetchingS3Batch(String),
    Deserialization(String),
    EmptyBody(String),
    StatusFailed(String),
    ReqwestClientFailed((u16, String)),
}

// needed to make S3 bucket work
const DEFAULT_USER_AGENT: &str = "proof-aggregator/aligned-layer";

pub async fn get_aligned_batch_from_s3(
    url: String,
) -> Result<Vec<VerificationData>, GetBatchProofsError> {
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
            response.status().canonical_reason(),
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
