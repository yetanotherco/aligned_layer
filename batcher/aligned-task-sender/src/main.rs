use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use aligned_sdk::communication::serialization::cbor_deserialize;
use aligned_sdk::communication::serialization::cbor_serialize;
use aligned_sdk::core::{
    errors::{AlignedError, SubmitError},
    types::{AlignedVerificationData, Chain, ProvingSystemId, VerificationData},
};
use aligned_sdk::sdk::get_chain_id;
use aligned_sdk::sdk::get_next_nonce;
use aligned_sdk::sdk::{get_vk_commitment, is_proof_verified, submit_multiple};

use clap::Parser;
use env_logger::Env;
use ethers::prelude::*;
use ethers::utils::format_ether;
use ethers::utils::hex;
use ethers::utils::parse_ether;
use k256::ecdsa::SigningKey;
use log::warn;
use log::{error, info};
use tokio::process::Command;
use tokio::time::{sleep, Duration};

const ANVIL_PRIVATE_KEY: &str = "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"; // Anvil address 9
const GROTH_16_PROOF_GENERATOR_FILE_PATH: &str = "scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go";
const GROTH_16_PROOF_DIR: &str = "scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs";
const PROOF_GENERATION_ADDR: &str = "0x66f9664f97F2b50F62D13eA064982f936dE76657";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TaskSenderArgs {
    #[arg(
        name = "Batcher connection address",
        long = "batcher_url",
        default_value = "ws://localhost:8080"
    )]
    batcher_url: String,
    #[arg(
        name = "Batcher Payment Service Eth Address",
        long = "payment_service_addr",
        default_value = "0x7969c5eD335650692Bc04293B07F5BF2e7A673C0"
    )]
    payment_service_addr: String,
    #[arg(
        name = "Ethereum RPC provider connection address",
        long = "rpc_url",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "Number of proofs per burst",
        long = "repetitions",
        default_value = "1"
    )]
    burst_size: usize,
    #[arg(name = "Burst Time", long = "burst", default_value = "10")]
    burst_time: u64, //TODO: add these options once it is working
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
    #[arg(name = "Path to local keystore", long = "keystore_path")]
    keystore_path: Option<PathBuf>,
    #[arg(name = "Private key", long = "private_key")]
    private_key: Option<String>,
    #[arg(
                           name = "Max Fee",
                           long = "max_fee",
                           default_value = "1300000000000000" // 13_000 gas per proof * 100 gwei gas price (upper bound)
                       )]
    max_fee: String, // String because U256 expects hex
    #[arg(name = "Nonce", long = "nonce")]
    nonce: Option<String>, // String because U256 expects hex
}

#[tokio::main]
async fn main() -> Result<(), AlignedError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: TaskSenderArgs = TaskSenderArgs::parse();

    let batch_inclusion_data_directory_path =
        PathBuf::from(&args.batch_inclusion_data_directory_path);

    std::fs::create_dir_all(&batch_inclusion_data_directory_path)
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_directory_path.clone(), e))?;

    let max_fee = U256::from_dec_str(&args.max_fee).map_err(|_| SubmitError::InvalidMaxFee)?;

    let burst_size = args.burst_size;

    let burst_time = args.burst_time;

    let connect_addr = args.batcher_url.clone();

    let keystore_path = &args.keystore_path;
    let private_key = &args.private_key;

    if keystore_path.is_some() && private_key.is_some() {
        warn!("Can't have a keystore path and a private key as input. Please use only one");
        return Ok(());
    }

    let mut wallet = if let Some(keystore_path) = keystore_path {
        let password = rpassword::prompt_password("Please enter your keystore password:")
            .map_err(|e| SubmitError::GenericError(e.to_string()))?;
        Wallet::decrypt_keystore(keystore_path, password)
            .map_err(|e| SubmitError::GenericError(e.to_string()))?
    } else if let Some(private_key) = private_key {
        private_key
            .parse::<LocalWallet>()
            .map_err(|e| SubmitError::GenericError(e.to_string()))?
    } else {
        warn!("Missing keystore used for payment. This proof will not be included if sent to Eth Mainnet");
        match LocalWallet::from_str(ANVIL_PRIVATE_KEY) {
            Ok(wallet) => wallet,
            Err(e) => {
                warn!(
                    "Failed to create wallet from anvil private key: {}",
                    e.to_string()
                );
                return Ok(());
            }
        }
    };

    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let eth_rpc_url = args.eth_rpc_url.clone();

    let chain_id = get_chain_id(eth_rpc_url.as_str()).await?;
    wallet = wallet.with_chain_id(chain_id);


    let batcher_eth_address = args.payment_service_addr.clone();

    let nonce = match &args.nonce {
        Some(nonce) => U256::from_dec_str(nonce).map_err(|_| SubmitError::InvalidNonce)?,
        None => {
            get_nonce(
                &eth_rpc_url,
                wallet.address(),
                &batcher_eth_address,
                burst_size,
            )
            .await?
        }
    };

    //TODO: We would have proof generation for all different kinds of things.
    std::fs::create_dir(GROTH_16_PROOF_DIR);

    let mut count = 1;
    tokio::spawn(async move {
        info!("Generating proof {} != 0", count);
        //TODO: use bindgen?
        Command::new("go run")
            .arg(GROTH_16_PROOF_GENERATOR_FILE_PATH)
            .arg(format!("{}", count))
            .spawn().expect("Failed to call Groth16 generation script")
            .wait().await;
        send_burst();
        //remove generated proofs to prevent bloat
        Command::new("rm")
            .arg(format!("{}/*",GROTH_16_PROOF_DIR))
            .spawn().expect("Failed to remove generated proof files")
            .wait()
            .await;
        sleep(Duration::from_millis(burst_time * 1000)).await;
        count += 1;
    });

    Ok(())
}

async fn send_burst(
    burst_size: usize, 
    burst_time: usize,
    count: usize,
    max_fee:, 
    wallet: Wallet<SigningKey>, 
    proof_generator_addr: Address,
    base_dir: PathBuf,
 ) -> Result<(), AlignedError> {

    let proof = read_file(!("{}/ineq_${}_groth16.proof", GROTH_16_PROOF_DIR, count))?;
    let public_input = read_file(format!("{}/ineq_${}_groth16.pub", GROTH_16_PROOF_DIR, count))?;
    let vk = read_file(format!("{}/ineq_${}_groth16.vk", GROTH_16_PROOF_DIR, count))?;
    let verification_key = ;
    let verification_data = VerificationData {
        proving_system: ProvingSystemId::Groth16Bn254,
        proof,
        Some(public_input),
        verification_key: Some(vk),
        vm_program_code: None,
        proof_generator_addr,
    };
    let verification_data_arr = vec![verification_data; burst_size];
    let max_fees = vec![max_fee; burst_size];
    let aligned_verification_data_vec = match submit_multiple(
        &connect_addr,
        chain,
        &verification_data_arr,
        &max_fees,
        wallet.clone(),
        nonce,
    )
    .await
    {
        Ok(aligned_verification_data_vec) => aligned_verification_data_vec,
        Err(e) => {
            let nonce_file = format!("nonce_{:?}.bin", wallet.address());

            handle_submit_err(e, nonce_file.as_str()).await;
            return Ok(());
        }
    };

    Ok(())
}

pub fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
    std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
}
