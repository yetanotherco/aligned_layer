#![feature(slice_flatten)]

use std::io;
use std::sync::Arc;

use aligned_sdk::sdk::{submit, verify_proof_onchain};
use aligned_sdk::types::{AlignedVerificationData, Chain, ProvingSystemId, VerificationData};
use clap::Parser;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::abigen;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Bytes, H160, U256};
use log::info;
use sp1_sdk::{ProverClient, SP1Stdin};
use tokio::io::AsyncWriteExt;

abigen!(
    VerifierContract,
    "../../contracts/out/VerifierContract.sol/VerifierContract.json"
);

const BATCHER_URL: &str = "wss://stage.batcher.alignedlayer.com";
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    keystore_path: String,
    #[arg(short, long, default_value = "https://ethereum-holesky-rpc.publicnode.com")]
    rpc_url: Option<String>,
    #[arg(short, long)]
    verifier_contract_address: H160,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");

    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore");

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
            // Verify proof.
            client
                .verify_compressed(&proof, &vk)
                .expect("verification failed");

            // Serialize proof into bincode (format used by sp1)
            let proof = bincode::serialize(&proof).expect("Failed to serialize proof");

            println!("Proof verified successfully. Submitting proof to the batcher...");

            let verification_data = VerificationData {
                proving_system: ProvingSystemId::SP1,
                proof,
                verification_key: Some(ELF.to_vec()),
                proof_generator_addr: wallet.address(),
                vm_program_code: None,
                pub_input: None,
            };

            match submit(BATCHER_URL, &verification_data, wallet.clone()).await {
                Ok(Some(aligned_verification_data)) => {
                    println!("Proof submitted successfully");
                    let rpc_url = args.rpc_url.unwrap_or_default();

                    if let Err(e) =
                        wait_for_proof_to_be_verified(aligned_verification_data.clone(), rpc_url.clone())
                            .await
                    {
                        println!("Proof verification failed: {:?}", e);
                    }

                    info!("Proof verified in Aligned, claiming prize...");
                    let provider = ethers::providers::Provider::connect(rpc_url.as_str()).await
                        .expect("Failed to connect to provider");

                    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

                    let verifier_contract =
                        VerifierContract::new(args.verifier_contract_address, signer.clone());

                    let index_in_batch = U256::from(aligned_verification_data.index_in_batch);

                    match verifier_contract.verify_batch_inclusion(
                        aligned_verification_data.verification_data_commitment.proof_commitment,
                        aligned_verification_data.verification_data_commitment.pub_input_commitment,
                        aligned_verification_data.verification_data_commitment.proving_system_aux_data_commitment,
                        aligned_verification_data.verification_data_commitment.proof_generator_addr,
                        aligned_verification_data.batch_merkle_root,
                        Bytes::from(aligned_verification_data.batch_inclusion_proof.merkle_path.as_slice().flatten().to_vec()),
                        index_in_batch,
                    ).await {
                        Ok(tx) => {
                            println!("Prize claimed successfully. Transaction hash: {:x}", tx);
                        }
                        Err(e) => {
                            println!("Failed to claim prize: {:?}", e);
                        }
                    }
                    // verifier_contract.verify_batch_inclusion(
                    //
                    //     aligned_verification_data.,
                    //     aligned_verification_data.,
                    // )
                }
                Ok(None) => {
                    println!("Proof submission failed, no verification data");
                }
                Err(e) => {
                    println!("Proof submission failed: {:?}", e);
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

    return read_answer();
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

async fn wait_for_proof_to_be_verified(
    verification_data: AlignedVerificationData,
    rpc_url: String,
) -> anyhow::Result<()> {
    for _ in 0..10 {
        if let Ok(_) =
            verify_proof_onchain(verification_data.clone(), Chain::Holesky, rpc_url.as_str()).await
        {
            return Ok(());
        }

        info!("Proof not verified yet. Waiting 10 seconds before checking again...");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }

    anyhow::bail!("Proof not verified after 10 attempts");
}
