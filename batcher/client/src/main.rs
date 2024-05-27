use std::path::PathBuf;

use alloy_primitives::Address;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use log::info;
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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = url::Url::parse(&args.connect_addr).unwrap();
    println!("URL: {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let (mut ws_write, ws_read) = ws_stream.split();

    let proving_system = parse_proving_system(&args.proving_system_flag);

    // Read proof file
    let proof = std::fs::read(&args.proof_file_name)
        .unwrap_or_else(|_| panic!("Failed to read .proof file: {:?}", args.proof_file_name));

    // Read public input file
    let mut pub_input: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.pub_input_file_name) {
        pub_input = Some(data);
    } else {
        println!("Warning: No Public Input file, continuing with no public_input");
    }

    let mut verification_key: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.verification_key_file_name) {
        verification_key = Some(data);
    } else {
        println!("Warning: no Verification Key File, continuing with no VK File");
    }

    let mut vm_program_code: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.vm_program_code_file_name) {
        vm_program_code = Some(data);
    } else {
        println!("Warning: no VM Program Code File, continuing with no VM Program Code");
    }

    // Dummy address for testing.
    let addr_str = "0x66f9664f97F2b50F62D13eA064982f936dE76657";
    let proof_generator_addr: Address = Address::parse_checksummed(addr_str, None).unwrap();

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
            let data = msg.unwrap().into_text();
            info!("RESPONSE: {:?}", data);
        })
        .await;

    ws_write
        .close()
        .await
        .expect("Failed to close WebSocket connection");
}
