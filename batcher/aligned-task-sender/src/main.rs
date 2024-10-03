use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
use tokio::runtime::Handle;

use aligned_sdk::core::{
    errors::{AlignedError, SubmitError},
    types::{Network, ProvingSystemId, VerificationData},
};
use aligned_sdk::sdk::get_chain_id;
use aligned_sdk::sdk::get_next_nonce;
use aligned_sdk::sdk::submit_multiple;

use clap::Parser;
use clap::ValueEnum;
use env_logger::Env;
use ethers::prelude::*;
use log::warn;
use log::{error, info};
use std::process::Command;

const ANVIL_PRIVATE_KEY: &str = "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"; // Anvil address 9
const GROTH_16_PROOF_GENERATOR_FILE_PATH: &str =
    "../../scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go";
const GROTH_16_PROOF_DIR: &str =
    "../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs";

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        name = "Ethereum RPC provider connection address",
        long = "rpc_url",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "Batcher connection address",
        long = "batcher-url",
        default_value = "ws://localhost:8080"
    )]
    batcher_url: String,
    #[arg(
        name = "Number of proofs per burst",
        long = "burst-size",
        default_value = "10"
    )]
    burst_size: usize,
    #[arg(name = "Burst Time", long = "burst-time", default_value = "3")]
    burst_time: usize,
    #[arg(
        name = "Number of spawned infinite task senders",
        long = "num-senders",
        default_value = "1"
    )]
    num_senders: usize,
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
        long = "max-fee",
        default_value = "1300000000000000" // 13_000 gas per proof * 100 gwei gas price (upper bound)
    )]
    max_fee: String, // String because U256 expects hex
    #[arg(name = "Nonce", long = "nonce")]
    nonce: Option<String>, // String because U256 expects hex
    #[arg(
        name = "The Ethereum network's name",
        long = "network",
        default_value = "devnet"
    )]
    network: NetworkArg,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum NetworkArg {
    Devnet,
    Holesky,
    HoleskyStage,
}

impl From<NetworkArg> for Network {
    fn from(chain_arg: NetworkArg) -> Self {
        match chain_arg {
            NetworkArg::Devnet => Network::Devnet,
            NetworkArg::Holesky => Network::Holesky,
            NetworkArg::HoleskyStage => Network::HoleskyStage,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AlignedError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let args = Args::parse();

    let max_fee = U256::from_dec_str(&args.max_fee).map_err(|_| SubmitError::InvalidMaxFee)?;

    let burst_size = args.burst_size;

    let burst_time = args.burst_time;

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

    let chain_id = get_chain_id(&args.eth_rpc_url).await.map_err(|e| {
                    SubmitError::GenericError(format!(
                        "Failed to retrieve chain id, verify `eth_rpc_url` is cor/rect or local testnet is running on \"http://localhost:8545\": {}", e
                    ))
            })?;
    wallet = wallet.with_chain_id(chain_id);

    let proof_generator_addr = Address::from_str(&args.proof_generator_addr).map_err(|e| {
        SubmitError::InvalidEthereumAddress(format!("Error while parsing address: {}", e))
    })?;
    std::fs::create_dir_all(GROTH_16_PROOF_DIR)
        .map_err(|e| SubmitError::IoError(PathBuf::from(GROTH_16_PROOF_DIR), e))?;

    let count = Arc::new(AtomicU64::new(1));
    // Generate 50 groth16 data in parallel.
    // TODO(pat): persist between runs.
    // TODO(pat): add go bindings so proof info is generate and stored them directly in memory.
    let threads: Vec<_> = (0..50)
        .map(|_| {
            let base_dir = base_dir.clone();
            let count = count.clone();
            thread::spawn(move || {
                let count = count.fetch_add(1, Ordering::Relaxed);
                Command::new("go")
                    .arg("run")
                    .arg(GROTH_16_PROOF_GENERATOR_FILE_PATH)
                    .arg(format!("{:?}", count))
                    .status()
                    .unwrap();

                let proof_path = base_dir.join(format!(
                    "{}/ineq_{}_groth16.proof",
                    GROTH_16_PROOF_DIR, count
                ));
                let public_input_path =
                    base_dir.join(format!("{}/ineq_{}_groth16.pub", GROTH_16_PROOF_DIR, count));
                let vk_path =
                    base_dir.join(format!("{}/ineq_{}_groth16.vk", GROTH_16_PROOF_DIR, count));

                let proof = std::fs::read(&proof_path)
                    .map_err(|e| SubmitError::IoError(proof_path, e))
                    .unwrap();
                let public_input = std::fs::read(&public_input_path)
                    .map_err(|e| SubmitError::IoError(public_input_path, e))
                    .unwrap();
                let vk = std::fs::read(&vk_path)
                    .map_err(|e| SubmitError::IoError(vk_path, e))
                    .unwrap();
                VerificationData {
                    proving_system: ProvingSystemId::Groth16Bn254,
                    proof,
                    pub_input: Some(public_input),
                    verification_key: Some(vk),
                    vm_program_code: None,
                    proof_generator_addr,
                }
            })
        })
        .collect();

    let cached_verification_data: Vec<VerificationData> =
        threads.into_iter().map(|t| t.join().unwrap()).collect();

    // We operate over a local network each thread sources its nonce by incrementing the global network nonce.
    // When multiple senders are spawned we just increment the atomic to grab the nonce.
    let network = args.network.into();
    let latest_nonce = get_next_nonce(&args.eth_rpc_url, wallet.address(), network)
        .await?
        .as_u64();
    let global_nonce = Arc::new(AtomicU64::new(latest_nonce));
    let threads = (0..args.num_senders)
        .map(|sender_id| {
            let batcher_url = args.batcher_url.clone();
            let nonce = global_nonce.clone();
            let handle = Handle::current();
            let cached_verification_data: Vec<VerificationData> = cached_verification_data.clone();
            let wallet = wallet.clone();
            thread::spawn(move || {
                info!("Task Sender Started {}", sender_id);
                loop {
                    let max_fees = vec![max_fee; args.burst_size];
                    //TODO(pat): not sure how slow this is... open to faster alternatives
                    let verification_data: Vec<_> = cached_verification_data
                        .choose_multiple(&mut thread_rng(), burst_size)
                        .cloned()
                        .collect();
                    info!(
                        "Sending {:?} Proofs to {:?} Aligned Batcher",
                        burst_size, network
                    );
                    let nonce = nonce.fetch_add(burst_size as u64, Ordering::Relaxed);
                    let batcher_url = batcher_url.clone();
                    let wallet = wallet.clone();
                    handle.spawn(async move {
                        if let Err(e) = submit_multiple(
                            &batcher_url.clone(),
                            network,
                            &verification_data.clone(),
                            &max_fees,
                            wallet.clone(),
                            U256::from(nonce),
                        )
                        .await
                        {
                            error!("Error submitting proofs to aligned: {:?}", e);
                        };
                        info!(
                            "{:?} Proofs to the Aligned Batcher {:?} on Holesky",
                            burst_size, network
                        );
                    });
                    std::thread::sleep(Duration::from_secs(burst_time as u64));
                }
            })
        })
        .collect::<Vec<_>>();

    for t in threads {
        if let Err(e) = t.join() {
            error!("Thread panicked: {:?}", e);
            return Ok(());
        }
    }

    //Clean infinite_proofs directory on exit
    if let Err(e) = std::fs::remove_dir_all(GROTH_16_PROOF_DIR) {
        error!("Failed to remove {}: {}", GROTH_16_PROOF_DIR, e);
    }
    Ok(())
}
