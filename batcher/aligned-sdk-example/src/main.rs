use aligned_sdk::errors::{AlignedError, SubmitError};
use aligned_sdk::models::{ProvingSystemId, VerificationData};
use aligned_sdk::{submit, verify_proof_onchain};
use env_logger::Env;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use futures_util::StreamExt;
use log::info;
use std::str::FromStr;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() -> Result<(), AlignedError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Connect to the WebSocket server where the batcher is running
    info!("Connecting to the batcher...");
    let (ws_stream, _) = connect_async("wss://batcher.alignedlayer.com")
        .await
        .map_err(SubmitError::ConnectionError)?;

    info!("WebSocket handshake has been successfully completed");

    let (ws_write, ws_read) = ws_stream.split();

    let ws_write_mutex = Arc::new(Mutex::new(ws_write));

    // Read the proof and the ELF file for the SP1 proving system into a PathBuf
    let proof_path = PathBuf::from("./test-files/sp1_fibonacci.proof");
    let elf_path = PathBuf::from("./test-files/sp1_fibonacci-elf");

    info!("Reading the proof and the ELF file...");
    // Read the proof and the ELF file for the SP1 proving system
    let proof =
        std::fs::read(proof_path.clone()).map_err(|e| SubmitError::IoError(proof_path, e))?;
    let elf = std::fs::read(elf_path.clone()).map_err(|e| SubmitError::IoError(elf_path, e))?;

    // Random address, in this case the first address of the anvil devnet
    let proof_generator_addr = Address::from_str("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")
        .map_err(|e| {
            SubmitError::InvalidAddress(
                "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
                e.to_string(),
            )
        })?;

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        pub_input: None,
        verification_key: None,
        vm_program_code: Some(elf),
        proof_generator_addr,
    };

    let verification_data_arr = vec![verification_data];

    info!("Submitting verification data...");

    let aligned_verification_data_vec =
        submit(ws_write_mutex, ws_read, verification_data_arr).await?;

    info!(
        "Aligned verification data: {:?}\n",
        aligned_verification_data_vec
    );

    // Wait a bit for the proof to be verified onchain
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let eth_rpc_provider =
        Provider::<Http>::try_from("https://ethereum-holesky-rpc.publicnode.com").unwrap();

    let aligned_verification_data = aligned_verification_data_vec.unwrap();

    let is_proof_verified_onchain = verify_proof_onchain(
        aligned_verification_data[0].clone(),
        aligned_sdk::models::Chain::Devnet,
        eth_rpc_provider,
    )
    .await?;

    info!("Is proof verified onchain: {}", is_proof_verified_onchain);

    Ok(())
}
