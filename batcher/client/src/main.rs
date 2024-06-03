use std::path::PathBuf;

use alloy_primitives::Address;
use env_logger::Env;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use log::{info};
use tokio_tungstenite::connect_async;

use batcher::types::{parse_proving_system, VerificationData};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        name = "Batcher address",
        long = "conn",
        default_value = "ws://localhost:8080"
    )]
    connect_addr: String,

    #[arg(name = "Proving System", long = "proving_system")]
    proving_system_flag: String,

    #[arg(name = "Proof file name", long = "proof")]
    proof_file_name: PathBuf,

    #[arg(
        name = "Public input file name",
        long = "public_input",
        default_value = "."
    )]
    pub_input_file_name: PathBuf,

    #[arg(name = "Verification key file name", long = "vk", default_value = ".")]
    verification_key_file_name: PathBuf,

    #[arg(
        name = "VM prgram code file name",
        long = "vm_program",
        default_value = "."
    )]
    vm_program_code_file_name: PathBuf,

    #[arg(
        name = "Number of repetitions",
        long = "repetitions",
        default_value = "1"
    )]
    repetitions: u32,

    #[arg(
        name = "Proof generator address",
        long = "proof_generator_addr",
        default_value = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
    )] // defaults to anvil address 1
    proof_generator_addr: String,

}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let url = url::Url::parse(&args.connect_addr).unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    info!("WebSocket handshake has been successfully completed");

    let (mut ws_write, ws_read) = ws_stream.split();

    let proving_system = parse_proving_system(&args.proving_system_flag).unwrap();

    // Read proof file
    let proof = std::fs::read(&args.proof_file_name)
        .unwrap_or_else(|_| panic!("Failed to read .proof file: {:?}", args.proof_file_name));

    // Read public input file
    let mut pub_input: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.pub_input_file_name) {
        pub_input = Some(data);
    } else {
        info!("No public input file provided, continuing without public input...");
    }

    let mut verification_key: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.verification_key_file_name) {
        verification_key = Some(data);
    } else {
        info!("No verification key file provided, continuing without verification key...");
    }

    let mut vm_program_code: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.vm_program_code_file_name) {
        vm_program_code = Some(data);
    } else {
        info!("No VM program code file provided, continuing without VM program code...");
    }

    let proof_generator_addr: Address = Address::parse_checksummed(&args.proof_generator_addr, None).unwrap();

    let verification_data = VerificationData {
        proving_system,
        proof,
        pub_input,
        verification_key,
        vm_program_code,
        proof_generator_addr,
    };

    let json_data = serde_json::to_string(&verification_data).expect("Failed to serialize task");
    for _ in 0..args.repetitions {
        ws_write
            .send(tungstenite::Message::Text(json_data.to_string()))
            .await
            .unwrap();
    }

    ws_read
        .try_filter(|msg| future::ready(msg.is_text()))
        .for_each(|msg| async move {
            let data = msg.unwrap().into_text().unwrap();
            info!("Batch merkle root received: {}", data);
        })
        .await;

    info!("Closing connection...");
    ws_write
        .close()
        .await
        .expect("Failed to close WebSocket connection");
}
