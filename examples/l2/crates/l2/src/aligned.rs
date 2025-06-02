use aligned_sdk::{
    aggregation_layer::ProofStatus,
    common::types::{AlignedVerificationData, Signer, VerificationData, Wallet},
    verification_layer::{estimate_fee, get_chain_id},
};
use sp1_sdk::{HashableKey, SP1VerifyingKey};

use crate::config::Config;

pub async fn send_proof_to_be_verified_on_aligned(
    config: &Config,
    proof: &sp1_sdk::SP1ProofWithPublicValues,
    vm_program_code: Vec<u8>,
) -> AlignedVerificationData {
    let proof = bincode::serialize(proof).expect("Serialize sp1 proof to binary");
    let chain_id = get_chain_id(&config.eth_rpc_url)
        .await
        .expect("To query chain id from rpc");
    let wallet = Wallet::decrypt_keystore(
        &config.private_key_store_path,
        &config.private_key_store_password,
    )
    .expect("Keystore to be `cast wallet` compliant")
    .with_chain_id(chain_id);

    let verification_data = VerificationData {
        proof_generator_addr: wallet.address(),
        proving_system: aligned_sdk::common::types::ProvingSystemId::SP1,
        proof,
        vm_program_code: Some(vm_program_code),
        pub_input: None,
        verification_key: None,
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

    aligned_sdk::verification_layer::submit(
        config.network.clone(),
        &verification_data,
        max_fee,
        wallet,
        nonce,
    )
    .await
    .expect("Proof to be sent")
}

pub async fn check_proof_proof_aggregation_status(
    config: &Config,
    proof: &sp1_sdk::SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> ProofStatus {
    let verification_data = aligned_sdk::aggregation_layer::AggregationModeVerificationData::SP1 {
        vk: vk.hash_bytes(),
        public_inputs: proof.public_values.to_vec(),
    };

    let proof_status = aligned_sdk::aggregation_layer::check_proof_verification(
        &verification_data,
        config.network.clone(),
        config.eth_rpc_url.clone(),
        config.beacon_client_url.clone(),
        // By default it looks back 24 hours
        None,
    )
    .await
    .expect("Get merkle path for proof");

    proof_status
}
