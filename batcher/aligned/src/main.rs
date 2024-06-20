mod errors;
mod eth;

use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::str::FromStr;
use std::{path::PathBuf, sync::Arc};

use env_logger::Env;
use ethers::prelude::*;
use futures_util::{
    future,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};
use log::{error, info};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use aligned_batcher_lib::types::{
    parse_proving_system, BatchInclusionData, ProvingSystemId, VerificationData,
};
use clap::Subcommand;
use ethers::utils::hex;

use crate::errors::BatcherClientError;
use crate::AlignedCommands::Submit;
use crate::AlignedCommands::VerifyProofOnchain;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AlignedArgs {
    #[clap(subcommand)]
    pub command: AlignedCommands,
}

#[derive(Subcommand, Debug)]
pub enum AlignedCommands {
    #[clap(about = "Submit proof to the batcher")]
    Submit(SubmitArgs),
    #[clap(about = "Verify the proof was included in a verified batch on Ethereum")]
    VerifyProofOnchain(VerifyProofOnchainArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SubmitArgs {
    #[arg(
        name = "Batcher address",
        long = "conn",
        default_value = "ws://localhost:8080"
    )]
    connect_addr: String,
    #[arg(name = "Proving system", long = "proving_system")]
    proving_system_flag: String,
    #[arg(name = "Proof file path", long = "proof")]
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
    #[arg(
        name = "Aligned verification data directory Path",
        long = "aligned_verification_data_path",
        default_value = "./aligned_verification_data/"
    )]
    batch_inclusion_data_directory_path: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct VerifyProofOnchainArgs {
    #[arg(name = "Aligned verification data", long = "aligned-verification-data")]
    batch_inclusion_data: PathBuf,
    #[arg(
        name = "Ethereum RPC provider address",
        long = "rpc",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "The Ethereum network's name",
        long = "chain",
        default_value = "devnet"
    )]
    chain: Chain,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Chain {
    Devnet,
    Holesky,
}

#[tokio::main]
async fn main() -> Result<(), errors::BatcherClientError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: AlignedArgs = AlignedArgs::parse();

    match args.command {
        Submit(submit_args) => {
            let url = url::Url::parse(&submit_args.connect_addr).map_err(|e| {
                errors::BatcherClientError::InvalidUrl(e, submit_args.connect_addr.clone())
            })?;

            let (ws_stream, _) = connect_async(url).await?;

            info!("WebSocket handshake has been successfully completed");
            let (mut ws_write, ws_read) = ws_stream.split();

            let batch_inclusion_data_directory_path =
                PathBuf::from(&submit_args.batch_inclusion_data_directory_path);

            let repetitions = submit_args.repetitions;
            let verification_data = verification_data_from_args(submit_args)?;

            let json_data = serde_json::to_string(&verification_data)?;
            for _ in 0..repetitions {
                ws_write.send(Message::Text(json_data.to_string())).await?;
                info!("Message sent...")
            }

            let num_responses = Arc::new(Mutex::new(0));
            let ws_write = Arc::new(Mutex::new(ws_write));

            receive(
                ws_read,
                ws_write,
                repetitions,
                num_responses,
                batch_inclusion_data_directory_path,
            )
            .await?;
        }

        VerifyProofOnchain(verify_inclusion_args) => {
            let contract_address = match verify_inclusion_args.chain {
                Chain::Devnet => "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
                Chain::Holesky => "0x58F280BeBE9B34c9939C3C39e0890C81f163B623",
            };

            let batch_inclusion_file =
                File::open(verify_inclusion_args.batch_inclusion_data).unwrap();
            let reader = BufReader::new(batch_inclusion_file);
            let batch_inclusion_data: BatchInclusionData = serde_json::from_reader(reader)?;

            let verification_data_comm = batch_inclusion_data.verification_data_commitment;

            // All the elements from the merkle proof have to be concatenated
            let merkle_proof: Vec<u8> = batch_inclusion_data
                .batch_inclusion_proof
                .merkle_path
                .into_iter()
                .flatten()
                .collect();

            let eth_rpc_url = verify_inclusion_args.eth_rpc_url;

            let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url).unwrap();

            // FIXME(marian): We are passing an empty string as the private key password for the moment.
            // We should think how to handle this correctly.
            let service_manager = eth::aligned_service_manager(
                eth_rpc_provider,
                contract_address,
            )
            .await?;

            let call = service_manager.verify_batch_inclusion(
                verification_data_comm.proof_commitment,
                verification_data_comm.pub_input_commitment,
                verification_data_comm.proving_system_aux_data_commitment,
                verification_data_comm.proof_generator_addr,
                batch_inclusion_data.batch_merkle_root,
                merkle_proof.into(),
                batch_inclusion_data.verification_data_batch_index.into(),
            );

            match call.call().await {
                Ok(response) => {
                    if response {
                        info!("Your proof was verified in Aligned and included in the batch!");
                    } else {
                        info!("Your proof was not included in the batch.");
                    }
                }

                Err(err) => error!("Error while reading batch inclusion verification: {}", err),
            }
        }
    }

    Ok(())
}

async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    total_messages: usize,
    num_responses: Arc<Mutex<usize>>,
    batch_inclusion_data_directory_path: PathBuf,
) -> Result<(), BatcherClientError> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    std::fs::create_dir_all(&batch_inclusion_data_directory_path)
        .map_err(|e| BatcherClientError::IoError(batch_inclusion_data_directory_path.clone(), e))?;

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
                    info!("Proof submitted to aligned. See the batch in the explorer:\nhttps://explorer.alignedlayer.com/batches/0x{}", hex::encode(batch_inclusion_data.batch_merkle_root));

                    let batch_merkle_root = hex::encode(batch_inclusion_data.batch_merkle_root);
                    let batch_inclusion_data_file_name = batch_merkle_root
                        + "_"
                        + &batch_inclusion_data
                            .verification_data_batch_index
                            .to_string()
                        + ".json";

                    let batch_inclusion_data_path =
                        batch_inclusion_data_directory_path.join(&batch_inclusion_data_file_name);
                    let mut file = File::create(&batch_inclusion_data_path).unwrap();
                    file.write_all(data.as_slice()).unwrap();
                    info!(
                        "Batch inclusion data written into {}",
                        batch_inclusion_data_path.display()
                    );
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

fn verification_data_from_args(args: SubmitArgs) -> Result<VerificationData, BatcherClientError> {
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
        ProvingSystemId::Jolt => {
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

    let proof_generator_addr = Address::from_str(&args.proof_generator_addr).unwrap();

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
