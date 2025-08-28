use std::process::Command;

use circom_example::{aligned, config::EnvConfig, eth};

#[tokio::main]
async fn main() {
    // Adjust the path to your script if it's not in the same directory
    let status = Command::new("bash")
        .arg("./circuits/generate_proof.sh")
        .status()
        .expect("failed to execute process");

    if status.success() {
        println!("Script executed successfully!");
    } else {
        println!("Script failed with status: {:?}", status.code());
    }

    let config: EnvConfig = EnvConfig {
        eth_rpc_url: "http://localhost:8545".to_string(),
        private_key_store_path: "devnet_keystore.json".to_string(),
        private_key_store_password: "".to_string(),
        fibonacci_contract_address: "".to_string(),
        network: aligned_sdk::common::types::Network::Devnet,
    };
    let proof = std::fs::read("circuits/proof.json").expect("proof to be created");
    let vk =
        std::fs::read("circuits/verification_key.json").expect("verification key to be created");
    let public_inputs_file =
        std::fs::read("circuits/public.json").expect("public inputs to be created");
    let pub_inputs: Vec<String> =
        serde_json::from_slice(&public_inputs_file).expect("parse inputs json");
    let decoded_inputs = aligned_sdk::common::utils::encode_circom_pub_inputs(&pub_inputs)
        .expect("Inputs to be decoded");

    let aligned_verification_data =
        aligned::submit_proof_to_aligned(config.clone(), proof, vk, decoded_inputs.clone()).await;

    let receipt =
        eth::update_number_on_contract(config, decoded_inputs, aligned_verification_data).await;

    println!("RECEIPT HASH {}", receipt);
}
