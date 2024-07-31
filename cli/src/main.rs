use std::error::Error;
use dotenv::dotenv;
use env_logger::Env;
use log::{info};
use crate::types::VerificationData;
use crate::http::get_batch;
use crate::proving_systems::verify_sp1_proof;
use crate::utils::select_random_verification_data;

mod types;
mod http;
mod proving_systems;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let batch = get_batch("b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json").await?;
    let verification_data = select_random_verification_data(&batch);

    verify_proof(verification_data).await?;

    info!("Verified successfully");

    Ok(())
}

async fn verify_proof(verification_data: &VerificationData) -> Result<(), Box<dyn Error>> {
    let proving_system = &verification_data.proving_system;

    match proving_system {
        types::ProvingSystemId::SP1 => verify_sp1_proof(verification_data).await,
        _ => panic!("Proving system not supported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_verify_proof() {

        let batch = get_batch("b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json").await.unwrap();
        let verification_data = select_random_verification_data(&batch);

        let result = verify_proof(verification_data).await.is_ok();

        assert!(result);

    }
}
