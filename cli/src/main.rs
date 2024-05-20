use aws_config::imds::client::error::IoError;
use dotenv::dotenv;
use env_logger::Env;
use log::{info};
use serde_json::{from_slice};
use crate::types::VerificationData;
use rand::prelude::*;
use sp1_sdk::ProverClient;

mod types;
#[tokio::main]
async fn main() -> Result<(), IoError> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let batch = get_batch("b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json").await.unwrap();
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..batch.len());
    let verification_data = &batch[random_index];
    let proof = &verification_data.proof;
    let vm_program_code = verification_data.vm_program_code.as_ref().expect("VM program code is missing");
    verify_sp1_proof(proof, vm_program_code).await.expect("Verification failed");
    info!("Verified successfully");
    Ok(())
}

async fn get_batch(key: &str) -> Result<Vec<VerificationData>, anyhow::Error> {
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

async fn verify_sp1_proof(proof: &[u8], elf: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sp1_prover_client: ProverClient = ProverClient::new();
    let (_pk, vk) = sp1_prover_client.setup(elf);
    let proof = bincode::deserialize(proof).map_err(|_| anyhow::anyhow!("Invalid proof"))?;

    sp1_prover_client
        .verify(&proof, &vk)
        .map_err(|_| anyhow::anyhow!("Failed to verify proof"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_verify_from_batch() {
        let batch = get_batch("b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json").await.unwrap();
        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..batch.len());
        let verification_data = &batch[random_index];
        let proof = &verification_data.proof;
        let vm_program_code = verification_data.vm_program_code.as_ref().expect("VM program code is missing");
        verify_sp1_proof(proof, vm_program_code).await.expect("Verification failed");
        // If the verification is successful, the test passes
        assert!(true);
    }
}
