use aligned_sdk::core::types::{Network, ProvingSystemId, VerificationData};
use aligned_sdk::sdk::{deposit_to_aligned, get_nonce_from_batcher, submit_multiple};
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
    GenerateAndFundWalletsArgs, GenerateProofsArgs, ProofType, SendInfiniteProofsArgs,
    TestConnectionsArgs,
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
                error!("Could not open private keys file: {}", err.to_string());
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
            error!("Could not open private keys file: {}", err.to_string());
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
                error!("Could not fund wallet {}", err.to_string());
                return;
            }
        };
        if let Err(err) = pending_transaction.await {
            error!("Could not fund wallet {}", err.to_string());
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
            error!("Could not store private key: {}", err.to_string());
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

pub async fn send_infinite_proofs(args: SendInfiniteProofsArgs) {
    if matches!(args.network.clone().into(), Network::Holesky) {
        error!("Network not supported this infinite proof sender");
        return;
    }

    info!("Loading wallets");
    let mut senders = vec![];
    let Ok(eth_rpc_provider) = Provider::<Http>::try_from(args.eth_rpc_url.clone()) else {
        error!("Could not connect to eth rpc");
        return;
    };
    let Ok(chain_id) = eth_rpc_provider.get_chainid().await else {
        error!("Could not get chain id");
        return;
    };

    let file = match File::open(&args.private_keys_filepath) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not open private keys file: {}", err.to_string());
            return;
        }
    };

    let reader = BufReader::new(file);

    // now here we need to load the senders from the provided files
    for line in reader.lines() {
        let private_key_str = match line {
            Ok(line) => line,
            Err(err) => {
                error!(
                    "Could not read line from private keys file: {}",
                    err.to_string()
                );
                return;
            }
        };
        let wallet = Wallet::from_str(private_key_str.trim()).expect("Invalid private key");
        let wallet = wallet.with_chain_id(chain_id.as_u64());
        let sender = Sender { wallet };

        // info!("Wallet {} loaded", i);
        senders.push(sender);
    }

    if senders.is_empty() {
        error!("No wallets in file");
        return;
    }
    info!("All wallets loaded");

    info!("Loading proofs verification data");
    let verification_data =
        get_verification_data_from_proofs_folder(args.proofs_dir, senders[0].wallet.address());
    if verification_data.is_empty() {
        error!("Verification data empty, not continuing");
        return;
    }
    info!("Proofs loaded!");

    let max_fee = U256::from_dec_str(&args.max_fee).expect("Invalid max fee");

    let mut handles = vec![];
    let network: Network = args.network.into();
    info!("Starting senders!");
    for (i, sender) in senders.iter().enumerate() {
        let wallet = sender.wallet.clone();
        let verification_data = verification_data.clone();
        // this is necessary because of the move
        let network_clone = network.clone();

        // a thread to send tasks from each loaded wallet:
        let handle = tokio::spawn(async move {
            loop {
                let n = network_clone.clone();
                let mut result = Vec::with_capacity(args.burst_size);
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
                while result.len() < args.burst_size {
                    let samples = verification_data
                        .choose_multiple(&mut thread_rng(), args.burst_size - result.len());
                    result.extend(samples.cloned());
                }
                let verification_data_to_send = result;

                info!(
                    "Sending {:?} Proofs to Aligned Batcher on {:?} from sender {}, nonce: {}, address: {:?}",
                    args.burst_size, n.clone(), i, nonce, wallet.address(),
                );

                let aligned_verification_data = submit_multiple(
                    n.clone(),
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

                tokio::time::sleep(Duration::from_secs(args.burst_time_secs)).await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = join!(handle);
    }
}

/// Returns the corresponding verification data for the generated proofs directory
fn get_verification_data_from_proofs_folder(
    dir_path: String,
    default_addr: Address,
) -> Vec<VerificationData> {
    let mut verifications_data = vec![];

    info!("Reading proofs from {:?}", dir_path);

    let dir = std::fs::read_dir(dir_path).expect("Directory does not exists");

    for proof_folder in dir {
        // each proof_folder is a dir called groth16_n
        let proof_folder_dir = proof_folder.unwrap().path();
        if proof_folder_dir.is_dir() {
            // todo(marcos): this should be improved if we want to support more proofs
            // currently we stored the proofs on subdirs with a prefix for the proof type
            // and here we check the subdir name and based on build the verification data accordingly
            if proof_folder_dir.to_str().unwrap().contains("groth16") {
                // Get the first file from the folder
                let first_file = fs::read_dir(proof_folder_dir.clone())
                    .expect("Can't read proofs directory")
                    .filter_map(|entry| entry.ok().map(|e| e.path()))
                    .find(|path| path.is_file()) // Find any valid file
                    .expect("No valid proof files found");

                // Extract the base name (file stem) without extension
                let base_name = first_file
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .expect("Failed to extract base name");

                // Generate the paths for the other files
                let proof_path = proof_folder_dir.join(format!("{}.proof", base_name));
                let public_input_path = proof_folder_dir.join(format!("{}.pub", base_name));
                let vk_path = proof_folder_dir.join(format!("{}.vk", base_name));

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
        }
    }

    verifications_data
}
