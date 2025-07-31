use aligned_sdk::common::types::{Network, ProvingSystemId, VerificationData};
use aligned_sdk::verification_layer::{
    deposit_to_aligned, get_nonce_from_batcher, submit_multiple,
};
use ethers::prelude::*;
use ethers::utils::parse_ether;
use k256::ecdsa::SigningKey;
use log::{debug, error, info};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use tokio::join;
use tokio_tungstenite::connect_async;

use crate::structs::{
    GenerateAndFundWalletsArgs, GenerateProofsArgs, InfiniteProofType, ProofType,
    SendInfiniteProofsArgs, TestConnectionsArgs,
};

const GROTH_16_PROOF_GENERATOR_FILE_PATH: &str =
    "../../scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go";
pub async fn generate_proofs(args: GenerateProofsArgs) {
    std::fs::create_dir_all(args.dir_to_save_proofs.clone()).expect("Could not create directory");

    let mut handles = vec![];
    for i in 1..args.number_of_proofs + 1 {
        let dir_to_save_proofs = args.dir_to_save_proofs.clone();

        let handle = thread::spawn(move || {
            match args.proof_type {
                ProofType::Groth16 => {
                    let dir_to_save_proofs =
                        format!("{}/groth16_{}/", dir_to_save_proofs.clone(), i);

                    // we need to create the directory as the go script does not handle it
                    if let Err(e) = fs::create_dir(dir_to_save_proofs.clone()) {
                        if e.kind() != ErrorKind::AlreadyExists {
                            eprintln!("Error creating directory: {}", e);
                            // Handle or log the error, but don't panic.
                        }
                    }

                    Command::new("go")
                        .arg("run")
                        .arg(GROTH_16_PROOF_GENERATOR_FILE_PATH)
                        .arg(format!("{:?}", i))
                        .arg(dir_to_save_proofs)
                        .status()
                        .unwrap();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

pub async fn generate_and_fund_wallets(args: GenerateAndFundWalletsArgs) {
    if matches!(args.network.clone().into(), Network::Devnet) {
        let Ok(eth_rpc_provider) = Provider::<Http>::try_from(args.eth_rpc_url.clone()) else {
            error!("Could not connect to eth rpc");
            return;
        };
        let Ok(chain_id) = eth_rpc_provider.get_chainid().await else {
            error!("Could not get chain id");
            return;
        };
        let amount_to_deposit_to_aligned =
            parse_ether(&args.amount_to_deposit_to_aligned).expect("Ether format should be: XX.XX");

        let file = match File::open(&args.private_keys_filepath) {
            Ok(f) => f,
            Err(err) => {
                error!("Could not open private keys file: {}", err);
                return;
            }
        };
        let file_reader = BufReader::new(file);

        let mut handles = vec![];
        for (i, line) in file_reader.lines().enumerate() {
            // Load the wallet
            let private_key_str = line.unwrap();
            let wallet = Wallet::from_str(private_key_str.trim())
                .expect("Invalid private key")
                .with_chain_id(chain_id.as_u64());

            // Send funds to aligned from the wallet
            let funded_wallet_signer =
                SignerMiddleware::new(eth_rpc_provider.clone(), wallet.clone());
            tokio::time::sleep(Duration::from_millis(50)).await; // To avoid overloading the RPC
            let network = args.network.clone();
            let handle = tokio::spawn(async move {
                if let Err(err) = deposit_to_aligned(
                    amount_to_deposit_to_aligned,
                    funded_wallet_signer.clone(),
                    network.into(),
                )
                .await
                {
                    error!("Could not deposit to aligned, err: {:?}", err);
                    return;
                }
                info!("Successfully deposited to aligned for wallet {}", i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("The task panicked");
        }

        info!("All wallets funded");

        return;
    }

    info!("Creating and funding wallets");
    let Ok(eth_rpc_provider) = Provider::<Http>::try_from(args.eth_rpc_url.clone()) else {
        error!("Could not connect to eth rpc");
        return;
    };
    let Ok(chain_id) = eth_rpc_provider.get_chainid().await else {
        error!("Could not get chain id");
        return;
    };

    let file = File::create(&args.private_keys_filepath);
    let mut file = match file {
        Ok(f) => f,
        Err(err) => {
            error!("Could not open private keys file: {}", err);
            return;
        }
    };

    let funding_wallet = args
        .funding_wallet_private_key
        .parse::<Wallet<SigningKey>>()
        .expect("Invalid private key")
        .with_chain_id(chain_id.as_u64());

    for i in 0..args.number_of_wallets {
        // this is necessary because of the move
        let eth_rpc_provider = eth_rpc_provider.clone();
        let funding_wallet = funding_wallet.clone();
        let amount_to_deposit = args.amount_to_deposit.clone();
        let amount_to_deposit_aligned = args.amount_to_deposit_to_aligned.clone();

        // Generate new wallet
        let wallet = Wallet::new(&mut thread_rng()).with_chain_id(chain_id.as_u64());
        info!("Generated wallet {} with address {:?}", i, wallet.address());

        // Fund the wallet
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
                error!("Could not fund wallet {}", err);
                return;
            }
        };
        if let Err(err) = pending_transaction.await {
            error!("Could not fund wallet {}", err);
        }
        info!("Wallet {} funded", i);

        // Deposit to aligned
        let amount_to_deposit_to_aligned =
            parse_ether(&amount_to_deposit_aligned).expect("Ether format should be: XX.XX");
        info!(
            "Depositing {}wei to aligned {}",
            amount_to_deposit_to_aligned, i
        );
        let signer = SignerMiddleware::new(eth_rpc_provider.clone(), wallet.clone());
        if let Err(err) = deposit_to_aligned(
            amount_to_deposit_to_aligned,
            signer,
            args.network.clone().into(),
        )
        .await
        {
            error!("Could not deposit to aligned, err: {:?}", err);
            return;
        }
        info!("Successfully deposited to aligned for wallet {}", i);

        // Store private key
        info!("Storing private key");
        let signer_bytes = wallet.signer().to_bytes();
        let secret_key_hex = ethers::utils::hex::encode(signer_bytes);

        if let Err(err) = writeln!(file, "{}", secret_key_hex) {
            error!("Could not store private key: {}", err);
        } else {
            info!("Private key {} stored", i);
        }
    }
}

/// infinitely hangs connections
pub async fn test_connection(args: TestConnectionsArgs) {
    info!("Going to only open a connection");
    let mut handlers = vec![];
    let network: Network = args.network.into();
    let ws_url_string = network.get_batcher_url().to_string();

    for i in 0..args.num_senders {
        let ws_url = ws_url_string.clone();
        let handle = tokio::spawn(async move {
            let conn = connect_async(ws_url).await;
            if let Ok((mut ws_stream, _)) = conn {
                info!("Opened connection for {}", i);
                while let Some(msg) = ws_stream.next().await {
                    match msg {
                        Ok(message) => debug!("Received message: {:?}", message),
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

struct Sender {
    wallet: Wallet<SigningKey>,
}

async fn load_senders_from_file(
    eth_rpc_url: &str,
    private_keys_filepath: &str,
) -> Result<Vec<Sender>, String> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|_| "Could not connect to eth rpc".to_string())?;
    let chain_id = eth_rpc_provider
        .get_chainid()
        .await
        .map_err(|_| "Could not get chain id".to_string())?;

    let file = File::open(private_keys_filepath)
        .map_err(|err| format!("Could not open private keys file: {}", err))?;

    let reader = BufReader::new(file);
    let mut senders = vec![];

    for line in reader.lines() {
        let private_key_str =
            line.map_err(|err| format!("Could not read line from private keys file: {}", err))?;
        let wallet = Wallet::from_str(private_key_str.trim())
            .map_err(|_| "Invalid private key".to_string())?
            .with_chain_id(chain_id.as_u64());
        let sender = Sender { wallet };
        senders.push(sender);
    }

    if senders.is_empty() {
        return Err("No wallets in file".to_string());
    }

    Ok(senders)
}

async fn run_infinite_proof_sender(
    senders: Vec<Sender>,
    verification_data: Vec<VerificationData>,
    network: Network,
    burst_size: usize,
    burst_time_secs: u64,
    max_fee: U256,
    random_address: bool,
) {
    let mut handles = vec![];

    for (i, sender) in senders.iter().enumerate() {
        let wallet = sender.wallet.clone();
        let verification_data = verification_data.clone();
        let network_clone = network.clone();

        let handle = tokio::spawn(async move {
            loop {
                let n = network_clone.clone();
                let mut result = Vec::with_capacity(burst_size);
                let nonce = get_nonce_from_batcher(n.clone(), wallet.address())
                    .await
                    .inspect_err(|e| {
                        error!(
                            "Could not get nonce: {:?}, for sender {:?}",
                            e,
                            wallet.address()
                        )
                    })
                    .unwrap();
                while result.len() < burst_size {
                    let samples = verification_data
                        .choose_multiple(&mut thread_rng(), burst_size - result.len());
                    for mut sample in samples.cloned() {
                        // Randomize proof generator address if requested
                        if random_address {
                            sample.proof_generator_addr = Address::random();
                        } else if sample.proof_generator_addr == Address::zero() {
                            // If it was set to zero (template), use wallet address
                            sample.proof_generator_addr = wallet.address();
                        }
                        result.push(sample);
                    }
                }
                let verification_data_to_send = result;

                info!(
                    "Sending {:?} Proofs to Aligned Batcher on {:?} from sender {}, nonce: {}, address: {:?}",
                    burst_size, n, i, nonce, wallet.address(),
                );

                let aligned_verification_data = submit_multiple(
                    n,
                    &verification_data_to_send.clone(),
                    max_fee,
                    wallet.clone(),
                    nonce,
                )
                .await;

                for aligned_verification_data in aligned_verification_data {
                    match aligned_verification_data {
                        Ok(_) => {
                            debug!("Response received for sender {}", i);
                        }
                        Err(e) => {
                            error!(
                                "Error submitting proofs to aligned: {:?} from sender {}",
                                e, i
                            );
                        }
                    }
                }
                info!("All responses received for sender {}", i);

                tokio::time::sleep(Duration::from_secs(burst_time_secs)).await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = join!(handle);
    }
}

pub async fn send_infinite_proofs(args: SendInfiniteProofsArgs) {
    if matches!(args.network.clone().into(), Network::Holesky) {
        error!("Network not supported this infinite proof sender");
        return;
    }

    // Load wallets using shared function
    info!("Loading wallets");
    let senders = match load_senders_from_file(&args.eth_rpc_url, &args.private_keys_filepath).await
    {
        Ok(senders) => senders,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };
    info!("All wallets loaded");

    // Load verification data based on proof type
    let verification_data = match &args.proof_type {
        InfiniteProofType::GnarkGroth16 { proofs_dir } => {
            info!("Loading Groth16 proofs from directory structure");
            let data = get_verification_data_from_proofs_folder(
                proofs_dir.clone(),
                senders[0].wallet.address(),
            );
            if data.is_empty() {
                error!("Verification data empty, not continuing");
                return;
            }
            data
        }
        InfiniteProofType::Risc0 {
            proof_path,
            bin_path,
            pub_path,
        } => {
            info!("Loading RISC Zero proof files");
            let Ok(proof) = std::fs::read(proof_path) else {
                error!("Could not read proof file: {}", proof_path);
                return;
            };
            let Ok(vm_program) = std::fs::read(bin_path) else {
                error!("Could not read bin file: {}", bin_path);
                return;
            };
            let pub_input = if let Some(pub_path) = pub_path {
                std::fs::read(pub_path).ok()
            } else {
                None
            };

            // Create template verification data (without proof_generator_addr)
            vec![VerificationData {
                proving_system: ProvingSystemId::Risc0,
                proof,
                pub_input,
                verification_key: None,
                vm_program_code: Some(vm_program),
                proof_generator_addr: Address::zero(), // Will be set randomly in the loop
            }]
        }
    };

    info!("Proofs loaded!");

    let max_fee = U256::from_dec_str(&args.max_fee).expect("Invalid max fee");
    let network: Network = args.network.into();

    info!("Starting senders!");
    run_infinite_proof_sender(
        senders,
        verification_data,
        network,
        args.burst_size,
        args.burst_time_secs,
        max_fee,
        args.random_address,
    )
    .await;
}

fn load_groth16_proof_files(
    dir_path: &std::path::Path,
    base_name: &str,
) -> Option<VerificationData> {
    let proof_path = dir_path.join(format!("{}.proof", base_name));
    let public_input_path = dir_path.join(format!("{}.pub", base_name));
    let vk_path = dir_path.join(format!("{}.vk", base_name));

    let proof = std::fs::read(&proof_path).ok()?;
    let public_input = std::fs::read(&public_input_path).ok()?;
    let vk = std::fs::read(&vk_path).ok()?;

    Some(VerificationData {
        proving_system: ProvingSystemId::GnarkGroth16Bn254,
        proof,
        pub_input: Some(public_input),
        verification_key: Some(vk),
        vm_program_code: None,
        proof_generator_addr: Address::zero(), // Will be set later
    })
}

fn load_from_subdirectories(dir_path: &str) -> Vec<VerificationData> {
    let mut verifications_data = vec![];
    let dir = std::fs::read_dir(dir_path).expect("Directory does not exist");

    for entry in dir.flatten() {
        let proof_folder_dir = entry.path();
        if proof_folder_dir.is_dir() && proof_folder_dir.to_str().unwrap().contains("groth16") {
            // Get the first file to determine the base name
            if let Some(first_file) = fs::read_dir(&proof_folder_dir)
                .ok()
                .and_then(|dir| dir.flatten().map(|e| e.path()).find(|path| path.is_file()))
            {
                if let Some(base_name) = first_file.file_stem().and_then(|s| s.to_str()) {
                    if let Some(verification_data) =
                        load_groth16_proof_files(&proof_folder_dir, base_name)
                    {
                        verifications_data.push(verification_data);
                    }
                }
            }
        }
    }

    verifications_data
}

fn load_from_flat_directory(dir_path: &str) -> Vec<VerificationData> {
    let mut verifications_data = vec![];
    let mut base_names = std::collections::HashSet::new();

    // Collect all unique base names from .proof files
    if let Ok(dir) = std::fs::read_dir(dir_path) {
        for entry in dir.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("proof") {
                if let Some(base_name) = path.file_stem().and_then(|s| s.to_str()) {
                    base_names.insert(base_name.to_string());
                }
            }
        }
    }

    // Load verification data for each base name
    let dir_path = std::path::Path::new(dir_path);
    for base_name in base_names {
        if let Some(verification_data) = load_groth16_proof_files(dir_path, &base_name) {
            verifications_data.push(verification_data);
        }
    }

    verifications_data
}

/// Returns the corresponding verification data for the generated proofs directory
fn get_verification_data_from_proofs_folder(
    dir_path: String,
    default_addr: Address,
) -> Vec<VerificationData> {
    info!("Reading proofs from {:?}", dir_path);

    // Check if we have subdirectories with groth16 in the name
    let has_groth16_subdirs = std::fs::read_dir(&dir_path)
        .map(|dir| {
            dir.flatten().any(|entry| {
                entry.path().is_dir() && entry.path().to_str().unwrap().contains("groth16")
            })
        })
        .unwrap_or(false);

    let mut verifications_data = if has_groth16_subdirs {
        load_from_subdirectories(&dir_path)
    } else {
        load_from_flat_directory(&dir_path)
    };

    // Set the default address for all verification data
    for data in &mut verifications_data {
        data.proof_generator_addr = default_addr;
    }

    verifications_data
}
