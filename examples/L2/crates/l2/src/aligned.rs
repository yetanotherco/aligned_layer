use aligned_sdk::{
    core::types::{AlignedVerificationData, Signer, VerificationData, Wallet},
    sdk::{estimate_fee, get_chain_id},
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
};
use futures_util::StreamExt;
use sp1_sdk::{HashableKey, SP1VerifyingKey};

use crate::Config;

pub async fn send_proof_to_be_verified_on_aligned(
    config: &Config,
    proof: &sp1_sdk::SP1ProofWithPublicValues,
    vm_program_code: Vec<u8>,
) -> AlignedVerificationData {
    let proof = bincode::serialize(proof).expect("Serialize sp1 proof to binary");
    let chain_id = get_chain_id(&config.eth_rpc_url).await.expect("To query chain id from rpc");
    let wallet = Wallet::decrypt_keystore(
        &config.private_key_store_path,
        &config.private_key_store_password,
    )
    .expect("Keystore to be `cast wallet` compliant")
    .with_chain_id(chain_id);

    let verification_data = VerificationData {
        proof_generator_addr: wallet.address(),
        proving_system: aligned_sdk::core::types::ProvingSystemId::SP1,
        proof,
        vm_program_code: Some(vm_program_code),
        pub_input: None,
        verification_key: None,
    };

    let nonce = aligned_sdk::sdk::get_nonce_from_batcher(config.network.clone(), wallet.address())
        .await
        .expect("Retrieve nonce from aligned batcher");

    let max_fee = estimate_fee(
        &config.eth_rpc_url,
        aligned_sdk::core::types::FeeEstimationType::Instant,
    )
    .await
    .expect("Max fee to be retrieved");

    aligned_sdk::sdk::submit(
        config.network.clone(),
        &verification_data,
        max_fee,
        wallet,
        nonce,
    )
    .await
    .expect("Proof to be sent")
}

pub async fn wait_until_proof_is_aggregated(
    config: &Config,
    proof: &sp1_sdk::SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> Vec<[u8; 32]> {
    let ws_rpc_url = &config.ws_eth_rpc_url;
    let ws = WsConnect::new(ws_rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    let aligned_proof_agg_address =
        Address::from(config.network.get_aligned_proof_agg_service_address().0);

    let filter = Filter::new()
        .address(aligned_proof_agg_address)
        .event("AggregatedProofVerified(bytes32,bytes32)")
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await.unwrap();
    let mut stream = sub.into_stream();

    let verification_data = aligned_sdk::sdk::aggregation::AggregationModeVerificationData::SP1 {
        vk: vk.hash_bytes(),
        public_inputs: proof.public_values.to_vec(),
    };

    let mut merkle_path = vec![];

    while stream.next().await.is_some() {
        if let Some(merkle_proof) = aligned_sdk::sdk::aggregation::get_merkle_path_for_proof(
            config.network.clone(),
            config.eth_rpc_url.clone(),
            config.beacon_client_url.clone(),
            None,
            &verification_data,
        )
        .await
        .expect("Get merkle path for proof")
        {
            merkle_path = merkle_proof;
            break;
        };
    }

    merkle_path
}
