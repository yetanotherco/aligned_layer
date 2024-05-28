use std::path::PathBuf;

use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

use batcher::types::{VerificationData, get_proving_system_from_str};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(name = "Batcher address", long = "conn", default_value = "ws://localhost:8080")]
    connect_addr: String,

    #[arg(name = "Proving System", long = "proving_system")]
    proving_system_flag: String,

    #[arg(name = "Proof file name", long = "proof")]
    proof_file_name: PathBuf,

    #[arg(name = "Public input file name", long = "public_input", default_value = ".")]
    public_input_file_name: PathBuf,

    #[arg(name = "Verification key file name", long = "vk", default_value = ".")]
    verification_key_file_name: PathBuf,

    #[arg(name = "VM prgram code file name", long = "vm_program", default_value = ".")]
    vm_program_code_file_name: PathBuf,

    #[arg(name = "Number of repetitions", long = "repetitions", default_value = "1")]
    repetitions: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = url::Url::parse(&args.connect_addr).unwrap();
    // panic!("Usage: {} <ws://addr> <sp1|plonk_bls12_381|plonk_bn254|groth16_bn254>", args[0]);


    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let (mut ws_write, ws_read) = ws_stream.split();

    let proving_system = get_proving_system_from_str(&args.proving_system_flag)
        .expect("Invalid proving system");

    // Read proof file
    let proof =
        std::fs::read(&args.proof_file_name)
        .unwrap_or_else(|_| panic!("Failed to read .proof file: {:?}", args.proof_file_name));
        

    // Read public input file
    let mut public_input: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.public_input_file_name) {
        public_input = Some(data);
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

    let task = VerificationData {
        proving_system,
        proof,
        public_input,
        verification_key,
        vm_program_code,
    };

    let json_data = serde_json::to_string(&task).expect("Failed to serialize task");
    for _ in 0..args.repetitions {
        ws_write
            .send(tungstenite::Message::Text(json_data.to_string()))
            .await
            .unwrap();
    }

    ws_read.take(args.repetitions as usize).for_each(|message| async move {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    })
    .await;

    ws_write.close().await.expect("Failed to close WebSocket connection");
}
