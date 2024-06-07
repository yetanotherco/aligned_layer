use std::{path::PathBuf, sync::Arc, time::Duration};

use alloy_primitives::{hex, Address};
use env_logger::Env;
use futures_util::{
    future,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use log::{error, info};
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
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    while let Some(Ok(msg)) = response_stream.next().await {
        if let Message::Close(close_frame) = msg {
            if let Some(close_msg) = close_frame {
                error!("Connection was closed before receiving all messages. Reason: {}. Try submitting again", close_msg.to_owned());
                ws_write.lock().await.close().await.unwrap();
                return;
            }
            error!("Connection was closed before receiving all messages. Try submitting again");
            ws_write.lock().await.close().await.unwrap();
            return;
        } else {
            let mut num_responses_lock = num_responses.lock().await;
            *num_responses_lock += 1;

            let data = msg.into_data();
            match serde_json::from_slice::<BatchInclusionData>(&data) {
                Ok(batch_inclusion_data) => {
                    info!("Batcher response received: {}", batch_inclusion_data);
                    info!("See the batch in the explorer:\nhttps://explorer.alignedlayer.com/batches/0x{}", hex::encode(batch_inclusion_data.batch_merkle_root));
                }
                Err(e) => {
                    error!("Error while deserializing batcher response: {}", e);
                }
            }
            if *num_responses_lock == total_messages {
                info!("All messages responded. Closing connection...");
                ws_write.lock().await.close().await.unwrap();
                return;
            }
        }
    }
}
