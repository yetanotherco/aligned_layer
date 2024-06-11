extern crate core;

mod errors;

use std::{path::PathBuf, sync::Arc};

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

use batcher::types::{parse_proving_system, BatchInclusionData, ProvingSystemId, VerificationData};

use crate::errors::BatcherClientError;
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

    #[arg(name = "Public input file name", long = "public_input")]
    pub_input_file_name: Option<PathBuf>,

    #[arg(name = "Verification key file name", long = "vk")]
    verification_key_file_name: Option<PathBuf>,

    #[arg(name = "VM prgram code file name", long = "vm_program")]
    vm_program_code_file_name: Option<PathBuf>,

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
async fn main() -> Result<(), errors::BatcherClientError> {
    let args = Args::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let url = url::Url::parse(&args.connect_addr)
        .map_err(|e| errors::BatcherClientError::InvalidUrl(e, args.connect_addr.clone()))?;

    let (ws_stream, _) = connect_async(url).await?;

    info!("WebSocket handshake has been successfully completed");

    let (mut ws_write, ws_read) = ws_stream.split();

    let repetitions = args.repetitions;
    let verification_data = verification_data_from_args(args)?;

    let json_data = serde_json::to_string(&verification_data)?;
    for _ in 0..repetitions {
        ws_write.send(Message::Text(json_data.to_string())).await?;
        info!("Message sent...")
    }

    let num_responses = Arc::new(Mutex::new(0));
    let ws_write = Arc::new(Mutex::new(ws_write));

    receive(ws_read, ws_write, repetitions, num_responses).await?;

    Ok(())
}

async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    total_messages: usize,
    num_responses: Arc<Mutex<usize>>,
) -> Result<(), BatcherClientError> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    while let Some(Ok(msg)) = response_stream.next().await {
        if let Message::Close(close_frame) = msg {
            if let Some(close_msg) = close_frame {
                error!("Connection was closed before receiving all messages. Reason: {}. Try submitting your proof again", close_msg.to_owned());
                ws_write.lock().await.close().await?;
                return Ok(());
            }
            error!("Connection was closed before receiving all messages. Try submitting your proof again");
            ws_write.lock().await.close().await?;
            return Ok(());
        } else {
            let mut num_responses_lock = num_responses.lock().await;
            *num_responses_lock += 1;

            let data = msg.into_data();
            match serde_json::from_slice::<BatchInclusionData>(&data) {
                Ok(batch_inclusion_data) => {
                    info!("Batcher response received: {}", batch_inclusion_data);
                    info!("Proof verified in aligned. See the batch in the explorer:\nhttps://explorer.alignedlayer.com/batches/0x{}", hex::encode(batch_inclusion_data.batch_merkle_root));
                }
                Err(e) => {
                    error!("Error while deserializing batcher response: {}", e);
                }
            }
            if *num_responses_lock == total_messages {
                info!("All messages responded. Closing connection...");
                ws_write.lock().await.close().await?;
                return Ok(());
            }
        }
    }

    Ok(())
}

fn verification_data_from_args(args: Args) -> Result<VerificationData, BatcherClientError> {
    let proving_system = parse_proving_system(&args.proving_system_flag)
        .map_err(|_| BatcherClientError::InvalidProvingSystem(args.proving_system_flag))?;

    // Read proof file
    let proof = read_file(args.proof_file_name)?;

    let mut pub_input: Option<Vec<u8>> = None;
    let mut verification_key: Option<Vec<u8>> = None;
    let mut vm_program_code: Option<Vec<u8>> = None;

    match proving_system {
        ProvingSystemId::SP1 => {
            vm_program_code = Some(read_file_option(
                "--vm_program",
                args.vm_program_code_file_name,
            )?);
        }
        ProvingSystemId::Halo2KZG
        | ProvingSystemId::Halo2IPA
        | ProvingSystemId::GnarkPlonkBls12_381
        | ProvingSystemId::GnarkPlonkBn254
        | ProvingSystemId::Groth16Bn254 => {
            verification_key = Some(read_file_option("--vk", args.verification_key_file_name)?);
            pub_input = Some(read_file_option(
                "--public_input",
                args.pub_input_file_name,
            )?);
        }
    }

    let proof_generator_addr: Address =
        Address::parse_checksummed(&args.proof_generator_addr, None).unwrap();

    Ok(VerificationData {
        proving_system,
        proof,
        pub_input,
        verification_key,
        vm_program_code,
        proof_generator_addr,
    })
}

fn read_file(file_name: PathBuf) -> Result<Vec<u8>, BatcherClientError> {
    std::fs::read(&file_name).map_err(|e| BatcherClientError::IoError(file_name, e))
}

fn read_file_option(
    param_name: &str,
    file_name: Option<PathBuf>,
) -> Result<Vec<u8>, BatcherClientError> {
    let file_name =
        file_name.ok_or(BatcherClientError::MissingParameter(param_name.to_string()))?;
    read_file(file_name)
}
