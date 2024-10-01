use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use aligned_sdk::core::errors::SubmitError;
use aligned_sdk::core::types::Network;
use aligned_sdk::core::types::{AlignedVerificationData, ProvingSystemId, VerificationData};
use aligned_sdk::sdk::{get_next_nonce, submit_and_wait_verification};
use clap::Parser;
use env_logger::Env;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, U256};
use ethers::utils::hex;
use log::info;

const BATCHER_URL: &str = "wss://batcher.alignedlayer.com";
const RPC_URL: &str = "https://ethereum-holesky-rpc.publicnode.com";
const PROOF_FILE_PATH: &str = "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof";
const PUB_INPUT_FILE_PATH: &str = "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub";
const IMAGE_ID_FILE_PATH: &str =
    "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_id.bin";
const PROOF_GENERATOR_ADDRESS: &str = "0x66f9664f97F2b50F62D13eA064982f936dE76657";
const NETWORK: Network = Network::Holesky;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    keystore_path: String,
}

#[tokio::main]
async fn main() -> Result<(), SubmitError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");

    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(17000u64);

    let proof = read_file(PathBuf::from(PROOF_FILE_PATH)).unwrap_or_default();

    let pub_input = read_file(PathBuf::from(PUB_INPUT_FILE_PATH));

    let image_id = read_file(PathBuf::from(IMAGE_ID_FILE_PATH));

    let pub_input_hex = hex::encode(pub_input.as_ref().unwrap());

    info!("Pub input bytes as hex: 0x{}", pub_input_hex);

    let proof_generator_addr = Address::from_str(PROOF_GENERATOR_ADDRESS).unwrap();

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::Risc0,
        proof,
        pub_input,
        verification_key: None,
        vm_program_code: image_id,
        proof_generator_addr,
    };

    // Set a fee of 0.1 Eth
    let max_fee = U256::from(5) * U256::from(100_000_000_000_000_000u128);

    let nonce = get_next_nonce(RPC_URL, wallet.address(), NETWORK)
        .await
        .expect("Failed to get next nonce");


    info!("Submitting Fibonacci proof to Aligned and waiting for verification...");
    let aligned_verification_data = submit_and_wait_verification(
        BATCHER_URL,
        RPC_URL,
        NETWORK,
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
    )?;

    Ok(())
}

fn read_file(file_name: PathBuf) -> Option<Vec<u8>> {
    std::fs::read(file_name).ok()
}

fn save_response(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
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

    let data = serde_json::to_vec(&aligned_verification_data).unwrap();

    let mut file = File::create(&batch_inclusion_data_path)
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;
    file.write_all(data.as_slice())
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;

    let current_dir = env::current_dir().expect("Failed to get current directory");

    info!(
        "Saved batch inclusion data to {:?}",
        current_dir.join(batch_inclusion_data_path)
    );

    Ok(())
}
