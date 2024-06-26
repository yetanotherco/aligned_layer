use log::info;
use serde_json::from_slice;
use crate::types::VerificationData;

pub async fn get_batch(key: &str) -> Result<Vec<VerificationData>, anyhow::Error> {
    info!("Retrieving batch from s3");
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    // This header is needed to avoid 403 Forbidden error
    headers.insert("user-agent","CUSTOM_NAME/1.0".parse().unwrap());

    let response = client.get(&format!("https://storage.alignedlayer.com/{}", key))
        .headers(headers)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to retrieve batch: {}", response.status()));
    }

    let body = response.bytes().await.unwrap();
    let batch: Vec<VerificationData> = from_slice(&body).unwrap();

    Ok(batch)
}
