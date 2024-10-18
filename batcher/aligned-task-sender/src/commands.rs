use aligned_sdk::core::types::{ProvingSystemId, VerificationData};
use aligned_sdk::sdk::{deposit_to_aligned, get_next_nonce, submit_multiple};
use ethers::prelude::*;
use ethers::utils::parse_ether;
use futures_util::StreamExt;
use k256::ecdsa::SigningKey;
use log::{error, info};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
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

    let count = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];
    for i in 0..args.number_of_proofs {
        let count = count.clone();
        let dir_to_save_proofs = args.dir_to_save_proofs.clone();

        let handle = thread::spawn(move || {
            let count = count.fetch_add(1, Ordering::Relaxed);
            match args.proof_type {
                ProofType::Groth16 => {
                    let dir_to_save_proofs =
                        format!("{}/groth16_{}/", dir_to_save_proofs.clone(), i);

                    // we need to create the directory as the go script does not handle it
                    std::fs::create_dir(dir_to_save_proofs.clone())
                        .expect("Could not create directory");

                    Command::new("go")
                        .arg("run")
                        .arg(GROTH_16_PROOF_GENERATOR_FILE_PATH)
                        .arg(format!("{:?}", count))
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

        let wallet = Wallet::new(&mut thread_rng()).with_chain_id(chain_id.as_u64());
        info!("Generated wallet {} with address {:?}", i, wallet.address());
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

        let amount_to_deposit_to_aligned =
            parse_ether(&amount_to_deposit_aligned).expect("Ether format should be: XX.XX");
        info!(
            "Depositing {}wei to aligned {}",
            amount_to_deposit_to_aligned, i
        );
        let signer = SignerMiddleware::new(eth_rpc_provider.clone(), wallet.clone());
        if let Err(err) =
            deposit_to_aligned(amount_to_deposit_to_aligned, signer, args.network.into()).await
        {
            error!("Could not deposit to aligned, err: {:?}", err);
            return;
        }
        info!("Successfully deposited to aligned for wallet {}", i);

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

    for i in 0..args.num_senders {
        let ws_url = args.batcher_url.clone();
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

struct Sender {
    wallet: Wallet<SigningKey>,
}

pub async fn infinite_proofs(args: SendInfiniteProofsArgs) {
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
    for (i, line) in reader.lines().enumerate() {
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

        info!("Wallet {} loaded", i);
        senders.push(sender);
    }

    if senders.is_empty() {
        error!("No wallets in file");
        return;
    }
    info!("All wallets loaded");

    info!("Loading proofs verification data");
    let verification_data =
        get_verification_data_from_generated(args.proofs_dir, senders[0].wallet.address());
    if verification_data.is_empty() {
        error!("Verification data empty, not continuing");
        return;
    }
    info!("Proofs loaded!");

    let max_fee = U256::from_dec_str(&args.max_fee).expect("Invalid max fee");

    let mut handles = vec![];
    info!("Starting senders!");
    for (i, sender) in senders.iter().enumerate() {
        // set the sender wallet address as the proof generator
        let verification_data: Vec<VerificationData> = verification_data
            .iter()
            .map(|d| VerificationData {
                proof_generator_addr: sender.wallet.address(),
                ..d.clone()
            })
            .collect();

        // this is necessary because of the move
        let eth_rpc_url = args.eth_rpc_url.clone();
        let batcher_url = args.batcher_url.clone();
        let wallet = sender.wallet.clone();

        let handle = tokio::spawn(async move {
            info!("Sender {} started", i);
            let mut nonce = get_next_nonce(&eth_rpc_url, wallet.address(), args.network.into())
                .await
                .unwrap_or(U256::zero());
            loop {
                let max_fees = vec![max_fee; args.burst_size];
                let verification_data: Vec<_> = verification_data
                    .choose_multiple(&mut thread_rng(), args.burst_size)
                    .cloned()
                    .collect();
                info!(
                    "Sending {:?} Proofs to Aligned Batcher on {:?} from sender {}",
                    args.burst_size, args.network, i
                );
                let batcher_url = batcher_url.clone();

                if let Err(e) = submit_multiple(
                    &batcher_url.clone(),
                    args.network.into(),
                    &verification_data.clone(),
                    &max_fees,
                    wallet.clone(),
                    nonce,
                )
                .await
                {
                    error!(
                        "Error submitting proofs to aligned: {:?} from sender {}",
                        e, i
                    );
                };
                info!(
                    "{:?} Proofs to the Aligned Batcher on{:?} sent from sender {}",
                    args.burst_size, args.network, i
                );
                nonce += U256::from(args.burst_size);
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
fn get_verification_data_from_generated(
    dir_path: String,
    default_addr: Address,
) -> Vec<VerificationData> {
    let mut verifications_data = vec![];

    let dir = std::fs::read_dir(dir_path).expect("Directory does not exists");

    for entry in dir {
        let dir = entry.unwrap().path();
        if dir.is_dir() {
            let dirname = dir.to_str().unwrap();
            // todo(marcos): this should be improved if we want to support more proofs
            // currently we stored the proofs on subdirs with a prefix for the proof type
            // and here we check the subdir name and based on build the verification data accordingly
            if dirname.contains("groth16") {
                for entry in fs::read_dir(dir).expect("Can't read directory") {
                    let entry = entry.expect("Invalid file");
                    let proof_path = entry.path().with_extension("proof");
                    let public_input_path = entry.path().with_extension("pub");
                    let vk_path = entry.path().with_extension("vk");

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
    }

    verifications_data
}
