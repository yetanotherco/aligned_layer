#![feature(slice_flatten)]
use std::io;

use aligned_sdk::core::types::{
    AlignedVerificationData, Network, PriceEstimate, ProvingSystemId, VerificationData,
};
use aligned_sdk::sdk::{deposit_to_aligned, estimate_fee};
use aligned_sdk::sdk::{get_next_nonce, submit_and_wait_verification};
use clap::Parser;
use dialoguer::Confirm;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, Bytes, H160, U256};
use sp1_sdk::{ProverClient, SP1Stdin};

abigen!(VerifierContract, "VerifierContract.json",);

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
    #[arg(short, long, default_value = "wss://batcher.alignedlayer.com")]
    batcher_url: String,
    #[arg(short, long, default_value = "holesky")]
    network: Network,
    #[arg(short, long)]
    verifier_contract_address: H160,
}

#[tokio::main]
async fn main() {
    println!("Welcome to the zkQuiz! Answer questions, generate a zkProof, and claim your NFT!");

    let args = Args::parse();
    let rpc_url = args.rpc_url.clone();

    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");

    let provider =
        Provider::<Http>::try_from(rpc_url.as_str()).expect("Failed to connect to provider");

    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain_id");

    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(chain_id.as_u64());

    let signer = SignerMiddleware::new(provider.clone(), wallet.clone());

    if Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Do you want to deposit 0.004eth in Aligned ?\nIf you already deposited Ethereum to Aligned before, this is not needed")
        .interact()
        .expect("Failed to read user input") {   

        deposit_to_aligned(U256::from(4000000000000000u128), signer.clone(), args.network).await
        .expect("Failed to pay for proof submission");
    }

    // Generate proof.
    let mut stdin = SP1Stdin::new();

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

    let Ok(proof) = client.prove(&pk, stdin).run() else {
        println!("Incorrect answers!");
        return;
    };

    println!("Proof generated successfully. Verifying proof...");
    client.verify(&proof, &vk).expect("verification failed");
    println!("Proof verified successfully.");

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

    let max_fee = estimate_fee(&rpc_url, PriceEstimate::Instant)
        .await
        .expect("failed to fetch gas price from the blockchain");

    let max_fee_string = ethers::utils::format_units(max_fee, 18).unwrap();

    if !Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(format!("Aligned will use at most {max_fee_string} eth to verify your proof. Do you want to continue?"))
        .interact()
        .expect("Failed to read user input")
    {   return; }

    let nonce = get_next_nonce(&rpc_url, wallet.address(), args.network)
        .await
        .expect("Failed to get next nonce");

        println!("Submitting your proof...");

    let aligned_verification_data = submit_and_wait_verification(
        &args.batcher_url,
        &rpc_url,
        args.network,
        &verification_data,
        max_fee,
        wallet.clone(),
        nonce,
    )
    .await
    .unwrap();

    println!(
        "Proof submitted and verified successfully on batch {}",
        hex::encode(aligned_verification_data.batch_merkle_root)
    );
    println!("Claiming NFT prize...");

    claim_nft_with_verified_proof(
        &aligned_verification_data,
        signer,
        &args.verifier_contract_address,
    )
    .await
    .expect("Claiming of NFT failed ...");
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

async fn claim_nft_with_verified_proof(
    aligned_verification_data: &AlignedVerificationData,
    signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    verifier_contract_addr: &Address,
) -> anyhow::Result<()> {
    let verifier_contract = VerifierContract::new(*verifier_contract_addr, signer.into());

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
