use aligned_sdk::core::types::VerificationData;

#[derive(Debug)]
pub enum GetBatchProofsError {
    Fetching,
    Deserialization,
    EmptyBody,
    StatusFailed,
}

pub async fn get_aligned_batch_from_s3(
    url: String,
) -> Result<Vec<VerificationData>, GetBatchProofsError> {
    let response = reqwest::get(url)
        .await
        .map_err(|_| GetBatchProofsError::Fetching)?;

    if !response.status().is_success() {
        return Err(GetBatchProofsError::StatusFailed);
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|_| GetBatchProofsError::EmptyBody)?;
    let bytes: &[u8] = bytes.iter().as_slice();

    let data: Vec<VerificationData> =
        ciborium::from_reader(bytes).map_err(|_| GetBatchProofsError::Deserialization)?;

    Ok(data)
}
