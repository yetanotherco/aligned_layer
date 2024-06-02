use std::{path::PathBuf, sync::Arc, time::Duration};

use alloy_primitives::Address;
use env_logger::Env;
use futures_util::{
    future,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use log::{info, warn};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use batcher::types::{parse_proving_system, BatchInclusionData, VerificationData};

use clap::Parser;
use tungstenite::Message;

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
    repetitions: usize,

    #[arg(
        name = "Proof generator address",
        long = "proof_generator_addr",
        default_value = "."
    )]
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
        warn!("No public input file provided, continuing without public input...");
    }

    let mut verification_key: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.verification_key_file_name) {
        verification_key = Some(data);
    } else {
        warn!("No verification key file provided, continuing without verification key...");
    }

    let mut vm_program_code: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.vm_program_code_file_name) {
        vm_program_code = Some(data);
    } else {
        warn!("No VM program code file provided, continuing without VM program code...");
    }

    let proof_generator_addr: Address =
        Address::parse_checksummed(&args.proof_generator_addr, None).unwrap();

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
        // NOTE(marian): This sleep is only for ease of testing interactions between client and batcher,
        // it can be removed.
        std::thread::sleep(Duration::from_millis(500));
        ws_write
            .send(tungstenite::Message::Text(json_data.to_string()))
            .await
            .unwrap();
        info!("Message sent...")
    }

    let num_responses = Arc::new(Mutex::new(0));
    let ws_write = Arc::new(Mutex::new(ws_write));

    receive(ws_read, ws_write, args.repetitions, num_responses).await;
}

async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    total_messages: usize,
    num_responses: Arc<Mutex<usize>>,
) {
    ws_read
        .try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .for_each(|msg| async {
            let mut num_responses_lock = num_responses.lock().await;
            *num_responses_lock += 1;
            let data = msg.unwrap().into_data();
            let deserialized_data: BatchInclusionData = serde_json::from_slice(&data).unwrap();
            info!("Batcher response received: {}", deserialized_data);

            if *num_responses_lock == total_messages {
                info!("All messages responded. Closing connection...");
                ws_write.lock().await.close().await.unwrap();
            }
        })
        .await;
}
