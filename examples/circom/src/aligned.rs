use std::str::FromStr;

use aligned_sdk::{
    common::types::{AlignedVerificationData, Signer, VerificationData, Wallet},
    verification_layer::{estimate_fee, get_chain_id},
};

use crate::config::EnvConfig;

pub async fn submit_proof_to_aligned(
    config: EnvConfig,
    proof: Vec<u8>,
    vk: Vec<u8>,
    public_inputs: Vec<u8>,
) -> AlignedVerificationData {
    let chain_id = get_chain_id(&config.eth_rpc_url)
        .await
        .expect("To query chain id from rpc");
    let wallet = Wallet::from_str(&config.private_key)
        .expect("Keystore to be `cast wallet` compliant")
        .with_chain_id(chain_id);

    let verification_data = VerificationData {
        proof_generator_addr: wallet.address(),
        proving_system: aligned_sdk::common::types::ProvingSystemId::CircomGroth16Bn256,
        proof,
        vm_program_code: None,
        pub_input: Some(public_inputs),
        verification_key: Some(vk),
    };

    let nonce = aligned_sdk::verification_layer::get_nonce_from_batcher(
        config.network.clone(),
        wallet.address(),
    )
    .await
    .expect("Retrieve nonce from aligned batcher");

    let max_fee = estimate_fee(
        &config.eth_rpc_url,
        aligned_sdk::common::types::FeeEstimationType::Instant,
    )
    .await
    .expect("Max fee to be retrieved");

    aligned_sdk::verification_layer::submit_and_wait_verification(
        &config.eth_rpc_url,
        config.network.clone(),
        &verification_data,
        max_fee,
        wallet,
        nonce,
    )
    .await
    .expect("Proof to be sent")
}
