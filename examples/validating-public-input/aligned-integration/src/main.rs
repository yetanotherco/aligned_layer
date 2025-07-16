use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use aligned_sdk::common::errors::SubmitError;
use aligned_sdk::common::types::Network;
use aligned_sdk::common::types::{AlignedVerificationData, ProvingSystemId, VerificationData};
use aligned_sdk::verification_layer::{get_nonce_from_batcher, submit_and_wait_verification};
use clap::Parser;
use clap::ValueEnum;
use env_logger::Env;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use ethers::utils::hex;
use log::info;
use serde_json::json;

const PROOF_FILE_RISC0_PATH: &str =
    "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof";
const PUB_INPUT_RISC0_FILE_PATH: &str =
    "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub";
const IMAGE_ID_RISC0_PATH: &str =
    "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_id.bin";
const PROOF_SP1_FILE_PATH: &str = "../sp1/fibonacci/sp1_fibonacci.proof";
const PUB_INPUT_SP1_FILE_PATH: &str = "../sp1/fibonacci/sp1_fibonacci.pub";
const ELF_FILE_PATH: &str = "../sp1/fibonacci/sp1_fibonacci.elf";

const ANVIL_PRIVATE_KEY: &str =
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum ProvingSystemArg {
    #[clap(name = "SP1")]
    SP1,
    #[clap(name = "Risc0")]
    Risc0,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "wss://batcher.alignedlayer.com")]
    batcher_url: String,
    #[clap(flatten)]
    network: NetworkArg,
    #[arg(short, long)]
    keystore_path: Option<String>,
    #[arg(short, long)]
    proving_system: ProvingSystemArg,
    #[arg(
        short,
        long,
        default_value = "https://ethereum-holesky-rpc.publicnode.com"
    )]
    rpc_url: String,
}

#[derive(Debug, Clone, Copy)]
enum NetworkNameArg {
    Devnet,
    Holesky,
    HoleskyStage,
    Mainnet,
}

impl FromStr for NetworkNameArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "devnet" => Ok(NetworkNameArg::Devnet),
            "holesky" => Ok(NetworkNameArg::Holesky),
            "holesky-stage" => Ok(NetworkNameArg::HoleskyStage),
            "mainnet" => Ok(NetworkNameArg::Mainnet),
            _ => Err(
                "Unknown network. Possible values: devnet, holesky, holesky-stage, mainnet"
                    .to_string(),
            ),
        }
    }
}

#[derive(Debug, clap::Args, Clone)]
struct NetworkArg {
    #[arg(
        name = "The working network's name",
        long = "network",
        default_value = "devnet",
        help = "[possible values: devnet, holesky, holesky-stage, mainnet]"
    )]
    network: Option<NetworkNameArg>,
    #[arg(
        name = "Aligned Service Manager Contract Address",
        long = "aligned_service_manager",
        conflicts_with("The working network's name"),
        requires("Batcher Payment Service Contract Address"),
        requires("Batcher URL")
    )]
    aligned_service_manager_address: Option<String>,

    #[arg(
        name = "Batcher Payment Service Contract Address",
        long = "batcher_payment_service",
        conflicts_with("The working network's name"),
        requires("Aligned Service Manager Contract Address"),
        requires("Batcher URL")
    )]
    batcher_payment_service_address: Option<String>,

    #[arg(
        name = "Batcher URL",
        long = "batcher_url",
        conflicts_with("The working network's name"),
        requires("Aligned Service Manager Contract Address"),
        requires("Batcher Payment Service Contract Address")
    )]
    batcher_url: Option<String>,
}

impl From<NetworkArg> for Network {
    fn from(network_arg: NetworkArg) -> Self {
        let mut processed_network_argument = network_arg.clone();

        if network_arg.batcher_url.is_some()
            || network_arg.aligned_service_manager_address.is_some()
            || network_arg.batcher_payment_service_address.is_some()
        {
            processed_network_argument.network = None; // We need this because network is Devnet as default, which is not true for a Custom network
        }

        match processed_network_argument.network {
            None => Network::Custom(
                network_arg.aligned_service_manager_address.unwrap(),
                network_arg.batcher_payment_service_address.unwrap(),
                network_arg.batcher_url.unwrap(),
            ),
            Some(NetworkNameArg::Devnet) => Network::Devnet,
            Some(NetworkNameArg::Holesky) => Network::Holesky,
            Some(NetworkNameArg::HoleskyStage) => Network::HoleskyStage,
            Some(NetworkNameArg::Mainnet) => Network::Mainnet,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), SubmitError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    let provider =
        Provider::<Http>::try_from(args.rpc_url.as_str()).expect("Failed to connect to provider");

    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain_id");

    let network: Network = args.network.clone().into();
    let wallet = match network {
        Network::Holesky => {
            let keystore_password = rpassword::prompt_password("Enter keystore password: ")
                .expect("Failed to read keystore password");

            LocalWallet::decrypt_keystore(
                args.keystore_path.expect("Keystore path not present"),
                &keystore_password,
            )
            .expect("Failed to decrypt keystore")
            .with_chain_id(chain_id.as_u64())
        }
        Network::Devnet => LocalWallet::from_str(ANVIL_PRIVATE_KEY)
            .expect("Failed to create wallet")
            .with_chain_id(chain_id.as_u64()),
        _ => panic!("Unsupported network"),
    };

    let verification_data = match args.proving_system {
        ProvingSystemArg::Risc0 => {
            let proof = read_file(PathBuf::from(PROOF_FILE_RISC0_PATH)).unwrap_or_default();
            let pub_input = read_file(PathBuf::from(PUB_INPUT_RISC0_FILE_PATH));
            let image_id = read_file(PathBuf::from(IMAGE_ID_RISC0_PATH));
            let proof_generator_addr = wallet.address();

            VerificationData {
                proving_system: ProvingSystemId::Risc0,
                proof,
                pub_input,
                verification_key: None,
                vm_program_code: image_id,
                proof_generator_addr,
            }
        }
        ProvingSystemArg::SP1 => {
            let proof = read_file(PathBuf::from(PROOF_SP1_FILE_PATH)).unwrap_or_default();
            let pub_input = read_file(PathBuf::from(PUB_INPUT_SP1_FILE_PATH));
            let elf = read_file(PathBuf::from(ELF_FILE_PATH));
            let proof_generator_addr = wallet.address();

            VerificationData {
                proving_system: ProvingSystemId::SP1,
                proof,
                pub_input,
                verification_key: None,
                vm_program_code: elf,
                proof_generator_addr,
            }
        }
    };

    // Set a fee of 0.01 Eth
    let max_fee = U256::from(100_000_000_000_000u128);

    let nonce = get_nonce_from_batcher(network.clone(), wallet.address())
        .await
        .expect("Failed to get next nonce");

    info!("Submitting Fibonacci proof to Aligned and waiting for verification...");
    let aligned_verification_data = submit_and_wait_verification(
        &args.rpc_url,
        network.clone().into(),
        &verification_data,
        max_fee,
        wallet,
        nonce,
    )
    .await?;

    let batch_inclusion_data_directory_path = PathBuf::from("batch_inclusion_data");

    info!(
        "Saving verification data to {:?}",
        batch_inclusion_data_directory_path
    );

    info!("Proof submitted to aligned. See the batch in the explorer:");

    info!(
        "https://explorer.alignedlayer.com/batches/0x{}",
        hex::encode(aligned_verification_data.batch_merkle_root)
    );

    save_response(
        batch_inclusion_data_directory_path,
        &aligned_verification_data,
        verification_data.pub_input.as_ref().unwrap(),
    )?;

    Ok(())
}

fn read_file(file_name: PathBuf) -> Option<Vec<u8>> {
    std::fs::read(file_name).ok()
}

fn save_response(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
    pub_input: &[u8],
) -> Result<(), SubmitError> {
    std::fs::create_dir_all(&batch_inclusion_data_directory_path)
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_directory_path.clone(), e))?;

    let batch_merkle_root = &hex::encode(aligned_verification_data.batch_merkle_root)[..8];
    let batch_inclusion_data_file_name = batch_merkle_root.to_owned()
        + "_"
        + &aligned_verification_data.index_in_batch.to_string()
        + ".json";

    let batch_inclusion_data_path =
        batch_inclusion_data_directory_path.join(batch_inclusion_data_file_name);

    let merkle_proof = aligned_verification_data
        .batch_inclusion_proof
        .merkle_path
        .iter()
        .map(hex::encode)
        .collect::<Vec<String>>()
        .join("");
    let data = json!({
            "proof_commitment": hex::encode(aligned_verification_data.verification_data_commitment.proof_commitment),
            "pub_input_commitment": hex::encode(aligned_verification_data.verification_data_commitment.pub_input_commitment),
            "program_id_commitment": hex::encode(aligned_verification_data.verification_data_commitment.proving_system_aux_data_commitment),
            "proof_generator_addr": hex::encode(aligned_verification_data.verification_data_commitment.proof_generator_addr),
            "batch_merkle_root": hex::encode(aligned_verification_data.batch_merkle_root),
            "pub_input": hex::encode(pub_input),
            "verification_data_batch_index": aligned_verification_data.index_in_batch,
            "merkle_proof": merkle_proof,
    });

    let mut file = File::create(&batch_inclusion_data_path)
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;
    file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;
    let current_dir = env::current_dir().expect("Failed to get current directory");

    info!(
        "Saved batch inclusion data to {:?}",
        current_dir.join(batch_inclusion_data_path)
    );

    Ok(())
}
