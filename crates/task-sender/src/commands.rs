use aligned_sdk::common::types::{Network, ProvingSystemId, VerificationData};
use aligned_sdk::verification_layer::{
    deposit_to_aligned, get_nonce_from_batcher, submit_multiple,
};
use ethers::prelude::*;
use ethers::utils::parse_ether;
use k256::ecdsa::SigningKey;
use log::{debug, error, info, warn};
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

    // Generate all wallets first
    let mut wallets = Vec::new();
    let mut wallet_private_keys = Vec::new();

    info!("Generating {} wallets...", args.number_of_wallets);
    for i in 0..args.number_of_wallets {
        let wallet = Wallet::new(&mut thread_rng()).with_chain_id(chain_id.as_u64());
        info!("Generated wallet {} with address {:?}", i, wallet.address());

        let signer_bytes = wallet.signer().to_bytes();
        let secret_key_hex = ethers::utils::hex::encode(signer_bytes);
        wallet_private_keys.push(secret_key_hex);
        wallets.push(wallet);
    }

    // Get base nonce for funding wallet to avoid nonce conflicts
    let mut current_nonce = match eth_rpc_provider
        .get_transaction_count(
            funding_wallet.address(),
            Some(ethers::types::BlockNumber::Pending.into()),
        )
        .await
    {
        Ok(nonce) => nonce,
        Err(err) => {
            error!("Could not get base nonce for funding wallet: {}", err);
            return;
        }
    };

    let batch_size = 25;
    let amount_to_deposit =
        parse_ether(&args.amount_to_deposit).expect("Ether format should be: XX.XX");
    let amount_to_deposit_to_aligned =
        parse_ether(&args.amount_to_deposit_to_aligned).expect("Ether format should be: XX.XX");

    let mut total_successful = 0;
    let total_batches = args.number_of_wallets.div_ceil(batch_size);

    // Process wallets in batches
    for (batch_idx, wallet_chunk) in wallets.chunks(batch_size).enumerate() {
        info!(
            "Processing batch {} of {} ({} wallets)...",
            batch_idx + 1,
            total_batches,
            wallet_chunk.len()
        );

        // Refresh nonce for each batch to avoid stale nonce issues
        current_nonce = match eth_rpc_provider
            .get_transaction_count(
                funding_wallet.address(),
                Some(ethers::types::BlockNumber::Pending.into()),
            )
            .await
        {
            Ok(nonce) => {
                info!("Batch {}: Using fresh nonce {}", batch_idx + 1, nonce);
                nonce
            }
            Err(err) => {
                error!("Could not get fresh nonce for batch {}: {}", batch_idx + 1, err);
                current_nonce // Use previous nonce as fallback
            }
        };

        // ETH funding phase for this batch
        info!(
            "Batch {}: Starting ETH funding transactions...",
            batch_idx + 1
        );
        let mut eth_funding_handles = Vec::new();

        for (chunk_idx, wallet) in wallet_chunk.iter().enumerate() {
            let global_idx = batch_idx * batch_size + chunk_idx;
            let eth_rpc_provider = eth_rpc_provider.clone();
            let funding_wallet = funding_wallet.clone();
            let wallet_address = wallet.address();
            let nonce = current_nonce + U256::from(chunk_idx);

            let handle = tokio::spawn(async move {

                info!(
                    "Submitting ETH funding transaction for wallet {} with nonce {}",
                    global_idx, nonce
                );
                let signer = SignerMiddleware::new(eth_rpc_provider, funding_wallet.clone());
                
                // Get current gas price and bump it by 20% to avoid replacement issues
                let base_gas_price = match signer.provider().get_gas_price().await {
                    Ok(price) => price,
                    Err(_) => U256::from(20_000_000_000u64), // 20 gwei fallback
                };
                let bumped_gas_price = base_gas_price * 120 / 100; // 20% bump
                
                let tx = TransactionRequest::new()
                    .from(funding_wallet.address())
                    .to(wallet_address)
                    .value(amount_to_deposit)
                    .nonce(nonce)
                    .gas_price(bumped_gas_price);

                let result = {
                    match signer.send_transaction(tx, None).await {
                        Ok(pending_tx) => {
                            info!(
                                "ETH funding transaction submitted for wallet {}",
                                global_idx
                            );
                            pending_tx.await
                        }
                        Err(err) => {
                            error!(
                                "Could not submit ETH funding transaction for wallet {}: {}",
                                global_idx, err
                            );
                            return None;
                        }
                    }
                };

                match result {
                    Ok(receipt) => {
                        if let Some(receipt) = receipt {
                            info!(
                                "ETH funding confirmed for wallet {} (tx: {:?})",
                                global_idx, receipt.transaction_hash
                            );
                        } else {
                            info!(
                                "ETH funding confirmed for wallet {} (no receipt)",
                                global_idx
                            );
                        }
                        Some(global_idx)
                    }
                    Err(err) => {
                        error!("ETH funding failed for wallet {}: {}", global_idx, err);
                        None
                    }
                }
            });
            eth_funding_handles.push(handle);
        }

        // Wait for ETH funding to complete
        let mut funded_indices = Vec::new();
        for handle in eth_funding_handles {
            if let Ok(Some(idx)) = handle.await {
                funded_indices.push(idx);
            }
        }

        info!(
            "Batch {}: ETH funding completed for {} out of {} wallets",
            batch_idx + 1,
            funded_indices.len(),
            wallet_chunk.len()
        );

        if funded_indices.is_empty() {
            warn!(
                "Batch {}: No wallets were funded, skipping Aligned deposits",
                batch_idx + 1
            );
            current_nonce += U256::from(wallet_chunk.len());
            continue;
        }

        // Aligned deposit phase for funded wallets in this batch
        info!(
            "Batch {}: Starting Aligned deposit transactions...",
            batch_idx + 1
        );
        let mut aligned_deposit_handles = Vec::new();

        for &idx in &funded_indices {
            let wallet = wallets[idx].clone();
            let eth_rpc_provider = eth_rpc_provider.clone();
            let network = args.network.clone();

            let handle = tokio::spawn(async move {

                info!("Submitting Aligned deposit for wallet {}", idx);
                let signer = SignerMiddleware::new(eth_rpc_provider, wallet);

                match deposit_to_aligned(amount_to_deposit_to_aligned, signer, network.into()).await
                {
                    Ok(_) => {
                        info!("Successfully deposited to aligned for wallet {}", idx);
                        Ok(idx)
                    }
                    Err(err) => {
                        error!("Could not deposit to aligned for wallet {}: {:?}", idx, err);
                        Err(idx)
                    }
                }
            });
            aligned_deposit_handles.push(handle);
        }

        // Wait for Aligned deposits to complete and write private keys immediately
        let mut batch_successful = 0;
        for handle in aligned_deposit_handles {
            if let Ok(Ok(idx)) = handle.await {
                let wallet_address = wallets[idx].address();
                let private_key = &wallet_private_keys[idx];
                
                // Write to original file (private key only) for compatibility
                if let Err(err) = writeln!(file, "{}", private_key) {
                    error!("Could not store private key for wallet {}: {}", idx, err);
                    continue;
                }
                
                // Write to new file (private_key;address format)
                let detailed_filepath = format!("{}.detailed", args.private_keys_filepath);
                let detailed_file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&detailed_filepath);
                
                match detailed_file {
                    Ok(mut f) => {
                        if let Err(err) = writeln!(f, "{};{:?}", private_key, wallet_address) {
                            error!("Could not store detailed info for wallet {}: {}", idx, err);
                        } else {
                            info!("Wallet {} stored: private key and address saved", idx);
                            batch_successful += 1;
                        }
                    }
                    Err(err) => {
                        error!("Could not open detailed file {}: {}", detailed_filepath, err);
                        // Still count as successful since main file was written
                        info!("Private key for wallet {} stored (detailed file failed)", idx);
                        batch_successful += 1;
                    }
                }
            }
        }

        total_successful += batch_successful;
        current_nonce += U256::from(wallet_chunk.len());

        info!(
            "Batch {} completed: {} wallets successfully funded and deposited (Total: {} / {})",
            batch_idx + 1,
            batch_successful,
            total_successful,
            args.number_of_wallets
        );

        // Optional: Small delay between batches (commented out for speed)
        // if batch_idx + 1 < total_batches {
        //     tokio::time::sleep(Duration::from_millis(50)).await;
        // }
    }

    info!(
        "All batches completed! Successfully created and funded {} wallets out of {} requested",
        total_successful, args.number_of_wallets
    );
    info!(
        "Private keys for {} successful wallets stored in:",
        total_successful
    );
    info!("  - {} (private keys only, for compatibility)", args.private_keys_filepath);
    info!("  - {}.detailed (private_key;address format)", args.private_keys_filepath);
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
