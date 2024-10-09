use ethers::utils::parse_ether;
use futures_util::join;
use k256::ecdsa::SigningKey;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use std::thread::{self};
use std::time::Duration;
use tokio_tungstenite::connect_async;

use aligned_sdk::core::{
    errors::{AlignedError, SubmitError},
    types::{Network, ProvingSystemId, VerificationData},
};
use aligned_sdk::sdk::get_next_nonce;
use aligned_sdk::sdk::submit_multiple;
use aligned_sdk::sdk::{deposit_to_aligned, get_chain_id};

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
const WALLETS_DIR: &str = "../../../scripts/test_files/wallets";

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    // TODO make this a subcommand
    #[arg(name = "Action", long = "action", default_value = "test_connection")]
    action: Action,
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
    #[arg(
        name = "The number of proofs to generate in generate-proofs",
        long = "number-of-proofs"
    )]
    number_of_proofs: Option<usize>,
    #[arg(
        name = "The amount to deposit to the wallets in generate-and-fund-wallets",
        long = "amount-to-deposit"
    )]
    amount_to_deposit: Option<String>,
    #[arg(
        name = "The amount to deposit to aligned in generate-and-fund-wallets",
        long = "amount-to-deposit-to-aligned"
    )]
    amount_to_deposit_to_aligned: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Action {
    GenerateProofs,
    CleanProofs,
    TestConnections,
    InfiniteProofs,
    MultipleSendersInfiniteProofs,
    GenerateAndFundWallets,
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "generate-proofs" => Ok(Action::GenerateProofs),
            "test-connections" => Ok(Action::TestConnections),
            "infinite-proofs" => Ok(Action::InfiniteProofs),
            "multiple-senders-infinite-proofs" => Ok(Action::MultipleSendersInfiniteProofs),
            "clean-proofs" => Ok(Action::CleanProofs),
            "generate-and-fund-wallets" => Ok(Action::GenerateAndFundWallets),
            _ => Err("Invalid action".to_string()),
        }
    }
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
    let keystore_path = args.keystore_path;
    let private_key = args.private_key;
    if keystore_path.is_some() && private_key.is_some() {
        warn!("Can't have a keystore path and a private key as input. Please use only one");
        return Ok(());
    }

    match args.action {
        Action::TestConnections => {
            infinitely_hang_connection(args.batcher_url, args.num_senders).await
        }
        Action::InfiniteProofs => {
            let wallet = get_sender_from_keystore_or_private_key(
                keystore_path,
                private_key,
                args.eth_rpc_url.clone(),
            )
            .await
            .unwrap();

            let sender = Sender { wallet };
            send_infinite_proofs(
                vec![sender],
                base_dir.clone(),
                args.eth_rpc_url,
                args.batcher_url,
                args.network.into(),
                args.burst_size,
                args.burst_time as u64,
                max_fee,
            )
            .await;
        }
        Action::GenerateAndFundWallets => {
            let funding_wallet = get_sender_from_keystore_or_private_key(
                keystore_path,
                private_key,
                args.eth_rpc_url.clone(),
            )
            .await
            .unwrap();
            generate_and_fund_wallets(
                funding_wallet,
                base_dir,
                args.num_senders,
                args.amount_to_deposit
                    .expect("Amount to deposit not provided"),
                args.amount_to_deposit_to_aligned
                    .expect("Amount to deposit to aligned not provided"),
                args.eth_rpc_url.clone(),
                args.network.into(),
            )
            .await;
        }
        Action::MultipleSendersInfiniteProofs => {
            send_multiple_senders_infinite_proofs(
                base_dir,
                args.num_senders,
                args.eth_rpc_url,
                args.batcher_url,
                args.network.into(),
                args.burst_size,
                args.burst_time as u64,
                max_fee,
            )
            .await;
        }
        Action::GenerateProofs => generate_proofs(
            args.number_of_proofs
                .expect("Number of proofs not provided provided"),
        )?,
        Action::CleanProofs => {
            if let Err(e) = std::fs::remove_dir_all(GROTH_16_PROOF_DIR) {
                error!("Failed to remove {}: {}", GROTH_16_PROOF_DIR, e);
            }
        }
    }

    Ok(())
}

struct Sender {
    wallet: Wallet<SigningKey>,
}

async fn get_sender_from_keystore_or_private_key(
    keystore_path: Option<PathBuf>,
    private_key: Option<String>,
    eth_rpc_url: String,
) -> Result<Wallet<SigningKey>, SubmitError> {
    let wallet = if let Some(keystore_path) = keystore_path {
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
                return Err(SubmitError::WalletSignerError(e.to_string()));
            }
        }
    };
    let chain_id = get_chain_id(&eth_rpc_url).await.map_err(|e| {
                    SubmitError::GenericError(format!(
                        "Failed to retrieve chain id, verify `eth_rpc_url` is correct or local testnet is running on \"http://localhost:8545\": {}", e
                    ))
            })?;
    let wallet = wallet.with_chain_id(chain_id);

    Ok(wallet)
}

async fn generate_and_fund_wallets(
    funding_wallet: Wallet<SigningKey>,
    base_dir: PathBuf,
    num_wallets: usize,
    amount_to_deposit: String,
    amount_to_deposit_aligned: String,
    eth_rpc_url: String,
    network: Network,
) {
    info!("Creating and funding wallets");
    if let Err(e) = std::fs::create_dir_all(WALLETS_DIR) {
        error!("Could not create wallets directory, err: {}", e.to_string());
        return;
    }
    let Ok(eth_rpc_provider) = Provider::<Http>::try_from(eth_rpc_url.clone()) else {
        error!("Could not connect to eth rpc");
        return;
    };
    let Ok(chain_id) = get_chain_id(&eth_rpc_url).await else {
        error!("Could not get chain id");
        return;
    };

    for i in 0..num_wallets {
        // this is necessary because of the move
        let eth_rpc_provider = eth_rpc_provider.clone();
        let funding_wallet = funding_wallet.clone();
        let amount_to_deposit = amount_to_deposit.clone();
        let amount_to_deposit_aligned = amount_to_deposit_aligned.clone();
        let base_dir = base_dir.clone();

        let wallet = Wallet::new(&mut thread_rng());
        let signer = SignerMiddleware::new(eth_rpc_provider.clone(), funding_wallet.clone());
        let amount_to_deposit =
            parse_ether(&amount_to_deposit).expect("Ether format should be: XX.XX");
        info!("Depositing {}wei to wallet {}", amount_to_deposit, i);
        let tx = TransactionRequest::new()
            .from(funding_wallet.address())
            .to(wallet.address())
            .value(amount_to_deposit);

        let pending_transaction = match signer.send_transaction(tx, None).await {
            Ok(tx) => tx,
            Err(err) => {
                error!("Could not fund wallet {}", err.to_string());
                return;
            }
        };
        if let Err(err) = pending_transaction.await {
            error!("Could not fund wallet {}", err.to_string());
        }
        let wallet = wallet.with_chain_id(chain_id);
        info!("Wallet {} funded", i);

        let amount_to_deposit_to_aligned =
            parse_ether(&amount_to_deposit_aligned).expect("Ether format should be: XX.XX");
        info!(
            "Depositing {}wei to aligned {}",
            amount_to_deposit_to_aligned, i
        );
        let signer = SignerMiddleware::new(eth_rpc_provider.clone(), wallet.clone());
        if let Err(err) = deposit_to_aligned(amount_to_deposit_to_aligned, signer, network).await {
            error!("Could not deposit to aligned, err: {:?}", err);
            return;
        }
        info!("Successfully deposited to aligned for wallet {}", i);

        info!("Storing private key");
        let file_path = base_dir.join(format!("{}/private_key-{}", WALLETS_DIR, i));

        if let Err(err) = std::fs::write(&file_path, wallet.signer().to_bytes()) {
            error!("Could not store private key: {}", err.to_string());
        } else {
            info!("Private key stored in {}", file_path.to_str().unwrap());
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn send_multiple_senders_infinite_proofs(
    base_dir: PathBuf,
    num_senders: usize,
    eth_rpc_url: String,
    batcher_url: String,
    network: Network,
    burst_size: usize,
    burst_time: u64,
    max_fee: U256,
) {
    info!("Loading wallets");
    let mut senders = vec![];
    let Ok(chain_id) = get_chain_id(&eth_rpc_url).await else {
        error!("Could not get chain_id");
        return;
    };

    // now here we need to load the senders
    for i in 0..num_senders {
        let file_path = base_dir.join(format!("{}/private_key-{}", WALLETS_DIR, i));
        let Ok(private_key_str) = std::fs::read(file_path) else {
            error!("Could not read private key");
            return;
        };
        let wallet = Wallet::from_bytes(private_key_str.as_slice()).expect("Invalid private key");
        let wallet = wallet.with_chain_id(chain_id);
        let sender = Sender { wallet };

        info!("Wallet {} loaded", i);
        senders.push(sender);
    }

    send_infinite_proofs(
        senders,
        base_dir,
        eth_rpc_url,
        batcher_url,
        network,
        burst_size,
        burst_time,
        max_fee,
    )
    .await;
}

#[allow(clippy::too_many_arguments)]
async fn send_infinite_proofs(
    senders: Vec<Sender>,
    base_dir: PathBuf,
    eth_rpc_url: String,
    batcher_url: String,
    network: Network,
    burst_size: usize,
    burst_time: u64,
    max_fee: U256,
) {
    if senders.is_empty() {
        return;
    }

    let verification_data =
        get_verification_data_from_generated(50, base_dir.clone(), senders[0].wallet.address());

    let mut handles = vec![];

    for sender in senders {
        // set the sender wallet address as the proof generator
        let verification_data: Vec<VerificationData> = verification_data
            .iter()
            .map(|d| VerificationData {
                proof_generator_addr: sender.wallet.address(),
                ..d.clone()
            })
            .collect();

        // this is necessary because of the move
        let eth_rpc_url = eth_rpc_url.clone();
        let batcher_url = batcher_url.clone();
        let wallet = sender.wallet.clone();

        let handle = tokio::spawn(async move {
            infinitely_send_proofs_from(
                verification_data,
                wallet,
                eth_rpc_url,
                network,
                batcher_url,
                burst_size,
                burst_time,
                max_fee,
            )
            .await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = join!(handle);
    }
}

fn generate_proofs(number: usize) -> Result<(), SubmitError> {
    std::fs::create_dir_all(GROTH_16_PROOF_DIR)
        .map_err(|e| SubmitError::IoError(PathBuf::from(GROTH_16_PROOF_DIR), e))?;

    let count = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for _ in 0..number {
        let count = count.clone();
        let handle = thread::spawn(move || {
            let count = count.fetch_add(1, Ordering::Relaxed);
            Command::new("go")
                .arg("run")
                .arg(GROTH_16_PROOF_GENERATOR_FILE_PATH)
                .arg(format!("{:?}", count))
                .status()
                .unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    Ok(())
}

/// Returns the corresponding verification data for the generated groth proofs
/// You'll probably have to change the address as the one  is the same for all
fn get_verification_data_from_generated(
    number: u64,
    base_dir: PathBuf,
    default_addr: Address,
) -> Vec<VerificationData> {
    let mut verifications_data = vec![];

    for i in 0..number {
        let proof_path = base_dir.join(format!("{}/ineq_{}_groth16.proof", GROTH_16_PROOF_DIR, i));
        let public_input_path =
            base_dir.join(format!("{}/ineq_{}_groth16.pub", GROTH_16_PROOF_DIR, i));
        let vk_path = base_dir.join(format!("{}/ineq_{}_groth16.vk", GROTH_16_PROOF_DIR, i));

        let Ok(proof) = std::fs::read(&proof_path) else {
            continue;
        };
        let Ok(public_input) = std::fs::read(&public_input_path) else {
            continue;
        };
        let Ok(vk) = std::fs::read(&vk_path) else {
            continue;
        };

        let verification_data = VerificationData {
            proving_system: ProvingSystemId::Groth16Bn254,
            proof,
            pub_input: Some(public_input),
            verification_key: Some(vk),
            vm_program_code: None,
            proof_generator_addr: default_addr,
        };
        verifications_data.push(verification_data);
    }

    verifications_data
}

#[allow(clippy::too_many_arguments)]
async fn infinitely_send_proofs_from(
    verification_data: Vec<VerificationData>,
    wallet: Wallet<SigningKey>,
    eth_rpc_url: String,
    network: Network,
    batcher_url: String,
    burst_size: usize,
    burst_time: u64,
    max_fee: U256,
) {
    let mut nonce = get_next_nonce(&eth_rpc_url, wallet.address(), network)
        .await
        .unwrap_or(U256::zero());
    loop {
        let max_fees = vec![max_fee; burst_size];
        let verification_data: Vec<_> = verification_data
            .choose_multiple(&mut thread_rng(), burst_size)
            .cloned()
            .collect();
        info!(
            "Sending {:?} Proofs to Aligned Batcher on {:?}",
            burst_size, network
        );
        let batcher_url = batcher_url.clone();

        if let Err(e) = submit_multiple(
            &batcher_url.clone(),
            network,
            &verification_data.clone(),
            &max_fees,
            wallet.clone(),
            nonce,
        )
        .await
        {
            error!("Error submitting proofs to aligned: {:?}", e);
        };
        info!(
            "{:?} Proofs to the Aligned Batcher on{:?}",
            burst_size, network
        );
        nonce += U256::from(burst_size);
        tokio::time::sleep(Duration::from_secs(burst_time)).await;
    }
}

async fn infinitely_hang_connection(batcher_url: String, num_senders: usize) {
    info!("Going to only open a connection");
    let mut handlers = vec![];

    for i in 0..num_senders {
        let ws_url = batcher_url.clone();
        let handle = tokio::spawn(async move {
            let conn = connect_async(ws_url).await;
            if let Ok((mut ws_stream, _)) = conn {
                info!("Opened connection for {}", i);
                while let Some(msg) = ws_stream.next().await {
                    match msg {
                        Ok(message) => info!("Received message: {:?}", message),
                        Err(e) => {
                            info!("WebSocket error: {}", e);
                            break;
                        }
                    }
                }
            } else {
                error!("Could not connect to socket, err {:?}", conn.err());
            }
        });
        handlers.push(handle);
    }

    for handle in handlers {
        let _ = join!(handle);
    }
}
