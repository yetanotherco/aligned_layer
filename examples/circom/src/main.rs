use std::{env, process::Command};

use circom_example::{aligned, config::EnvConfig, eth};

#[tokio::main]
async fn main() {
    println!("===============================");
    println!("Starting proof generation...");
    println!("===============================");

    // Run proof generation script
    let status = Command::new("bash")
        .arg("./circuits/generate_proof.sh")
        .status()
        .expect("failed to execute process");

    if status.success() {
        println!("Proof generation script executed successfully");
    } else {
        println!("Script failed with status: {:?}", status.code());
        return;
    }

    // Handle custom .env file
    let args: Vec<String> = env::args().collect();
    let custom_env = if args.len() > 1 {
        let env_file = args[1].to_string();
        println!("Using custom .env file: {env_file}");
        Some(env_file)
    } else {
        println!("Using default .env file (if present)");
        None
    };

    let config: EnvConfig = EnvConfig::new(custom_env);

    // Load circuit artifacts
    println!("-------------------------------");
    println!("Loading circuit artifacts...");
    println!("-------------------------------");

    let proof = std::fs::read("circuits/proof.json").expect("proof.json should exist");
    let vk = std::fs::read("circuits/verification_key.json")
        .expect("verification_key.json should exist");
    let public_inputs_file =
        std::fs::read("circuits/public.json").expect("public.json should exist");
    let pub_inputs: Vec<String> =
        serde_json::from_slice(&public_inputs_file).expect("could not parse inputs json");
    let decoded_inputs = aligned_sdk::common::utils::encode_circom_pub_inputs(&pub_inputs)
        .expect("inputs should be decoded");

    println!("Submitting proof to Aligned...");
    let aligned_verification_data =
        aligned::submit_proof_to_aligned(config.clone(), proof, vk, decoded_inputs.clone()).await;

    println!("Updating on-chain contract with proof result...");
    let receipt =
        eth::update_number_on_contract(config, decoded_inputs, aligned_verification_data).await;

    println!("Done. Transaction receipt hash: {}", receipt);
}
