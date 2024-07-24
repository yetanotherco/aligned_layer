#![feature(slice_flatten)]

use std::io;
use std::str::FromStr;
use std::sync::Arc;

use aligned_sdk::sdk::{submit, verify_proof_onchain};
use aligned_sdk::types::{AlignedVerificationData, Chain, ProvingSystemId, VerificationData};
use clap::Parser;
use dialoguer::Confirm;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Bytes, H160, U256};
use sp1_sdk::{ProverClient, SP1Stdin};

abigen!(
    VerifierContract,
    "VerifierContract.json",
);

const BATCHER_URL: &str = "wss://batcher.alignedlayer.com";
const BATCHER_PAYMENTS_ADDRESS: &str = "0x815aeCA64a974297942D2Bbf034ABEe22a38A003";
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    keystore_path: String,
    #[arg(
        short,
        long,
        default_value = "https://ethereum-holesky-rpc.publicnode.com"
    )]
    rpc_url: String,
    #[arg(short, long)]
    verifier_contract_address: H160,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");

    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(17000u64);

    // Generate proof.
    let mut stdin = SP1Stdin::new();

    println!("Welcome to the quiz! Please answer the following questions to generate a proof for the program.");
    println!(
        "You will be asked 3 questions. Please answer with the corresponding letter (a, b or c)."
    );

    let mut user_awnsers = "".to_string();
    let question1 = "Who invented bitcoin";
    let answers1 = ["Sreeram Kannan", "Vitalik Buterin", "Satoshi Nakamoto"];
    user_awnsers.push(ask_question(question1, &answers1));

    let question2 = "What is the largest ocean on Earth?";
    let answers2 = ["Atlantic", "Indian", "Pacific"];
    user_awnsers.push(ask_question(question2, &answers2));

    let question3 = "What is the most aligned color";
    let answers3 = ["Green", "Red", "Blue"];
    user_awnsers.push(ask_question(question3, &answers3));

    stdin.write(&user_awnsers);
    println!("Generating Proof ");

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    match client.prove_compressed(&pk, stdin) {
        Ok(proof) => {
            println!("Proof generated successfully. Verifying proof...");

            client
                .verify_compressed(&proof, &vk)
                .expect("verification failed");

            println!("Proof verified successfully.");

            let rpc_url = args.rpc_url.clone();

            let provider = Provider::<Http>::try_from(rpc_url.as_str())
                .expect("Failed to connect to provider");

            let signer = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

            // Pay for proof submission
            pay_batcher(wallet.address(), signer.clone())
                .await
                .expect("Failed to pay for proof submission");

            println!("Payment successful. Submitting proof...");

            // Serialize proof into bincode (format used by sp1)
            let proof = bincode::serialize(&proof).expect("Failed to serialize proof");

            let verification_data = VerificationData {
                proving_system: ProvingSystemId::SP1,
                proof,
                proof_generator_addr: wallet.address(),
                vm_program_code: Some(ELF.to_vec()),
                verification_key: None,
                pub_input: None,
            };

            match submit_proof_and_wait_for_verification(
                verification_data,
                wallet.clone(),
                rpc_url.clone(),
            ).await {
                Ok(aligned_verification_data) => {
                    println!("Proof verified in Aligned, claiming prize...");

                    if let Err(e) = verify_batch_inclusion(
                        aligned_verification_data.clone(),
                        signer.clone(),
                        args.verifier_contract_address,
                    )
                        .await
                    {
                        println!("Failed to claim prize: {:?}", e);
                    }
                }
                Err(e) => {
                    println!("Proof verification failed: {:?}", e);
                }
            }
        }
        Err(_) => {
            println!("Proof generation failed. Incorrect answer");
        }
    }
}

fn ask_question(question: &str, answers: &[&str]) -> char {
    println!("{}", question);
    for (i, answer) in answers.iter().enumerate() {
        println!("{}. {}", (b'a' + i as u8) as char, answer);
    }

    read_answer()
}

fn is_valid_answer(answer: char) -> bool {
    answer == 'a' || answer == 'b' || answer == 'c'
}

fn read_answer() -> char {
    loop {
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read from stdin");

        answer = answer.trim().to_string();
        if answer.len() != 1 {
            println!("Please enter a valid answer (a, b or c)");
            continue;
        }

        let c = answer.chars().next().unwrap();
        if !is_valid_answer(c) {
            println!("Please enter a valid answer (a, b or c)");
            continue;
        }

        return c;
    }
}

async fn submit_proof_and_wait_for_verification(
    verification_data: VerificationData,
    wallet: Wallet<SigningKey>,
    rpc_url: String,
) -> anyhow::Result<AlignedVerificationData> {
    let res = submit(BATCHER_URL, &verification_data, wallet.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit proof for verification: {:?}", e))?;

    match res {
        Some(aligned_verification_data) => {
            println!(
                "Proof submitted successfully on batch {}, waiting for verification...",
                hex::encode(aligned_verification_data.batch_merkle_root)
            );

            for _ in 0..10 {
                if verify_proof_onchain(
                    aligned_verification_data.clone(),
                    Chain::Holesky,
                    rpc_url.as_str(),
                )
                    .await
                    .is_ok_and(|r| r)
                {
                    return Ok(aligned_verification_data);
                }

                println!("Proof not verified yet. Waiting 10 seconds before checking again...");
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }

            anyhow::bail!("Proof verification failed");
        }
        None => {
            anyhow::bail!("Proof submission failed, no verification data");
        }
    }
}

async fn pay_batcher(
    from: Address,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> anyhow::Result<()> {
    if !Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("We are going to pay 0.004eth for the proof submission to aligned. Do you want to continue?")
        .interact()
        .expect("Failed to read user input")
    {
        anyhow::bail!("Payment cancelled")
    }

    let addr = Address::from_str(BATCHER_PAYMENTS_ADDRESS).map_err(|e| anyhow::anyhow!(e))?;

    let tx = TransactionRequest::new()
        .from(from)
        .to(addr)
        .value(4000000000000000u128);

    match signer
        .send_transaction(tx, None)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send tx {}", e))?
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit tx {}", e))?
    {
        Some(receipt) => {
            println!(
                "Payment sent. Transaction hash: {:x}",
                receipt.transaction_hash
            );
            Ok(())
        }
        None => {
            anyhow::bail!("Payment failed");
        }
    }
}

async fn verify_batch_inclusion(
    aligned_verification_data: AlignedVerificationData,
    signer: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    verifier_contract_addr: Address,
) -> anyhow::Result<()> {
    let verifier_contract = VerifierContract::new(verifier_contract_addr, signer);

    let index_in_batch = U256::from(aligned_verification_data.index_in_batch);
    let merkle_path = Bytes::from(
        aligned_verification_data
            .batch_inclusion_proof
            .merkle_path
            .as_slice()
            .flatten()
            .to_vec(),
    );

    let receipt = verifier_contract
        .verify_batch_inclusion(
            aligned_verification_data
                .verification_data_commitment
                .proof_commitment,
            aligned_verification_data
                .verification_data_commitment
                .pub_input_commitment,
            aligned_verification_data
                .verification_data_commitment
                .proving_system_aux_data_commitment,
            aligned_verification_data
                .verification_data_commitment
                .proof_generator_addr,
            aligned_verification_data.batch_merkle_root,
            merkle_path,
            index_in_batch,
        )
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send tx {}", e))?
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit tx {}", e))?;

    match receipt {
        Some(receipt) => {
            println!(
                "Prize claimed successfully. Transaction hash: {:x}",
                receipt.transaction_hash
            );
            Ok(())
        }
        None => {
            anyhow::bail!("Failed to claim prize: no receipt");
        }
    }
}
