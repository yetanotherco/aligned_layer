use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use aligned_sdk::core::{
    errors::{AlignedError, SubmitError},
    types::{Chain, ProvingSystemId, VerificationData},
};
use aligned_sdk::sdk::get_chain_id;
use aligned_sdk::sdk::get_next_nonce;
use aligned_sdk::sdk::submit_multiple;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use env_logger::Env;
use ethers::prelude::*;
use ethers::utils::hex;
use k256::ecdsa::SigningKey;
use log::warn;
use log::{error, info};
use std::process::Command;
use tokio::time::{sleep, Duration};

const ANVIL_PRIVATE_KEY: &str = "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"; // Anvil address 9
const GROTH_16_PROOF_GENERATOR_FILE_PATH: &str =
    "../../scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go";
const GROTH_16_PROOF_DIR: &str =
    "../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs";

use crate::TaskSenderCommands::SendInfinite;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TaskSenderArgs {
    #[clap(subcommand)]
    pub command: TaskSenderCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum TaskSenderCommands {
    #[clap(about = "Send Infinite Proofs to the batcher")]
    SendInfinite(SubmitArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct SubmitArgs {
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
        long = "burst_size",
        default_value = "10"
    )]
    burst_size: usize,
    #[arg(name = "Burst Time", long = "burst_time", default_value = "3")]
    burst_time: usize,
    #[arg(
        name = "Proof generator address",
        long = "proof_generator_addr",
        default_value = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
    )] // defaults to anvil address 1
    proof_generator_addr: String,
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
    #[arg(
        name = "The Ethereum network's name",
        long = "chain",
        default_value = "devnet"
    )]
    chain: ChainArg,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ChainArg {
    Devnet,
    Holesky,
    HoleskyStage,
}

impl From<ChainArg> for Chain {
    fn from(chain_arg: ChainArg) -> Self {
        match chain_arg {
            ChainArg::Devnet => Chain::Devnet,
            ChainArg::Holesky => Chain::Holesky,
            ChainArg::HoleskyStage => Chain::HoleskyStage,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AlignedError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let args: TaskSenderArgs = TaskSenderArgs::parse();

    match args.command {
        SendInfinite(submit_args) => {
            let max_fee =
                U256::from_dec_str(&submit_args.max_fee).map_err(|_| SubmitError::InvalidMaxFee)?;

            let burst_size = submit_args.burst_size;

            let burst_time = submit_args.burst_time;

            let keystore_path = &submit_args.keystore_path;
            let private_key = &submit_args.private_key;

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

            let eth_rpc_url = submit_args.eth_rpc_url.clone();

            let chain_id = get_chain_id(eth_rpc_url.as_str()).await.map_err(|e| {
                    SubmitError::GenericError(format!(
                        "Failed to retrieve chain id, verify `eth_rpc_url` is cor/rect or local testnet is running on \"http://localhost:8545\": {}", e
                    ))
            })?;
            wallet = wallet.with_chain_id(chain_id);

            let proof_generator_addr = Address::from_str(&submit_args.proof_generator_addr)
                .map_err(|e| {
                    SubmitError::InvalidEthereumAddress(format!(
                        "Error while parsing address: {}",
                        e
                    ))
                })?;
            std::fs::create_dir_all(GROTH_16_PROOF_DIR)
                .map_err(|e| SubmitError::IoError(PathBuf::from(GROTH_16_PROOF_DIR), e))?;

            let mut count = 1;
            //TODO: sort out error messages
            if let Err(e) = tokio::spawn(async move {
                loop {
                    info!("Generating proof {} != 0", count);
                    if let Err(e) = Command::new("go")
                        .arg("run")
                        .arg(GROTH_16_PROOF_GENERATOR_FILE_PATH)
                        .arg(format!("{:?}", count))
                        .status()
                    {
                        error!("Failed to generate Groth16 Proofs: {:?}", e);
                        return Ok::<(), AlignedError>(());
                    }
                    let (max_fees, verification_data) = match get_verification_data(
                        burst_size,
                        count,
                        max_fee,
                        proof_generator_addr,
                        &base_dir,
                    )
                    .await
                    {
                        Ok(value) => value,
                        Err(e) => {
                            error!("Failed to create Verification Data: {:?}", e);
                            return Ok(());
                        }
                    };
                    if let Err(e) =
                        send_burst(&max_fees, &verification_data, &wallet, submit_args.clone())
                            .await
                    {
                        error!("Failed to send Proof Burst: {:?}", e);
                        return Ok(());
                    };
                    // To prevent continously creating proofs we clear the directory after sending a burst.
                    std::fs::remove_dir_all(GROTH_16_PROOF_DIR).unwrap_or_else(|e| {
                        error!("Failed to send Clear Proof Directory: {:?}", e);
                    });
                    std::fs::create_dir_all(GROTH_16_PROOF_DIR).unwrap_or_else(|e| {
                        error!("Failed to Create Proof Directory and Clearing: {:?}", e);
                    });
                    sleep(Duration::from_secs(burst_time as u64)).await;
                    count += 1;
                }
            })
            .await
            {
                error!("Proof Stream Exited: {:?}", e);
            }

            //Clean infinite_proofs directory on exit
            if let Err(e) = std::fs::remove_dir_all(GROTH_16_PROOF_DIR) {
                error!("Failed to remove {}: {}", GROTH_16_PROOF_DIR, e);
            }
        }
    }

    Ok(())
}

async fn get_verification_data(
    burst_size: usize,
    count: usize,
    max_fee: U256,
    proof_generator_addr: Address,
    base_dir: &Path,
) -> Result<(Vec<U256>, Vec<VerificationData>), AlignedError> {
    let proof = read_file(base_dir.join(format!(
        "{}/ineq_{}_groth16.proof",
        GROTH_16_PROOF_DIR, count
    )))?;
    let public_input =
        read_file(base_dir.join(format!("{}/ineq_{}_groth16.pub", GROTH_16_PROOF_DIR, count)))?;
    let vk = read_file(base_dir.join(format!("{}/ineq_{}_groth16.vk", GROTH_16_PROOF_DIR, count)))?;
    let verification_data = VerificationData {
        proving_system: ProvingSystemId::Groth16Bn254,
        proof,
        pub_input: Some(public_input),
        verification_key: Some(vk),
        vm_program_code: None,
        proof_generator_addr,
    };
    Ok((
        vec![max_fee; burst_size],
        vec![verification_data; burst_size],
    ))
}

async fn send_burst(
    max_fees: &[U256],
    verification_data: &[VerificationData],
    wallet: &Wallet<SigningKey>,
    args: SubmitArgs,
) -> Result<(), AlignedError> {
    let chain: Chain = args.chain.clone().into();
    let eth_rpc_url = args.eth_rpc_url.clone();
    let batcher_url: String = args.batcher_url;

    let batcher_addr = args.payment_service_addr.clone();

    let nonce = get_nonce(
        &eth_rpc_url,
        wallet.address(),
        &batcher_addr,
        verification_data.len(),
    )
    .await?;
    info!(
        "Sending {:?} Proofs to {:?} Aligned Batcher",
        verification_data.len(),
        chain
    );
    let aligned_verification_data_vec = match submit_multiple(
        &batcher_url,
        chain,
        verification_data,
        max_fees,
        wallet.clone(),
        nonce,
    )
    .await
    {
        Ok(aligned_verification_data_vec) => aligned_verification_data_vec,
        Err(e) => {
            error!("Error submitting proofs to aligned: {:?}", e);
            let nonce_file = format!("nonce_{:?}.bin", wallet.address());

            delete_file(&nonce_file).unwrap_or_else(|e| {
                error!("Error while deleting nonce file: {}", e);
            });
            return Ok(());
        }
    };

    let mut unique_batch_merkle_roots = HashSet::new();

    for aligned_verification_data in aligned_verification_data_vec {
        unique_batch_merkle_roots.insert(aligned_verification_data.batch_merkle_root);
    }

    match unique_batch_merkle_roots.len() {
        1 => info!("Proofs submitted to aligned. See the batch in the explorer:"),
        _ => info!("Proofs submitted to aligned. See the batches in the explorer:"),
    }

    for batch_merkle_root in unique_batch_merkle_roots {
        info!(
            "https://explorer.alignedlayer.com/batches/0x{}",
            hex::encode(batch_merkle_root)
        );
    }

    Ok(())
}

//Persits and extrapolate this to sdk
async fn get_nonce(
    eth_rpc_url: &str,
    address: Address,
    batcher_contract_addr: &str,
    proof_count: usize,
) -> Result<U256, AlignedError> {
    let nonce = get_next_nonce(eth_rpc_url, address, batcher_contract_addr).await?;

    let nonce_file = format!("nonce_{:?}.bin", address);

    let local_nonce = read_file(PathBuf::from(nonce_file.clone())).unwrap_or(vec![0u8; 32]);
    let local_nonce = U256::from_big_endian(local_nonce.as_slice());

    let nonce = if local_nonce > nonce {
        local_nonce
    } else {
        nonce
    };

    let mut nonce_bytes = [0; 32];

    (nonce + U256::from(proof_count)).to_big_endian(&mut nonce_bytes);

    write_file(nonce_file.as_str(), &nonce_bytes)?;

    Ok(nonce)
}

//TODO: Duplicated from `batcher/aligned`.
fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
    std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
}

fn write_file(file_name: &str, content: &[u8]) -> Result<(), SubmitError> {
    std::fs::write(file_name, content)
        .map_err(|e| SubmitError::IoError(PathBuf::from(file_name), e))
}

fn delete_file(file_name: &str) -> Result<(), io::Error> {
    std::fs::remove_file(file_name)
}
