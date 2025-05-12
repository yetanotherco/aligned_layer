use aligned_sdk::{
    core::types::{AlignedVerificationData, Signer, SigningKey, VerificationData, Wallet},
    sdk::estimate_fee,
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
};
use futures_util::StreamExt;
use sp1_sdk::{HashableKey, SP1VerifyingKey};

pub async fn send_proof_to_be_verified_on_aligned(
    proof: &sp1_sdk::SP1ProofWithPublicValues,
    vm_program_code: Vec<u8>,
    network: aligned_sdk::core::types::Network,
    wallet: Wallet<SigningKey>,
) -> AlignedVerificationData {
    let proof = bincode::serialize(proof).expect("Serialize sp1 proof to binary");

    let verification_data = VerificationData {
        proof_generator_addr: wallet.address(),
        proving_system: aligned_sdk::core::types::ProvingSystemId::SP1,
        proof,
        vm_program_code: Some(vm_program_code),
        pub_input: None,
        verification_key: None,
    };

    let nonce = aligned_sdk::sdk::get_nonce_from_batcher(network.clone(), wallet.address())
        .await
        .expect("Retrieve nonce from aligned batcher");

    let max_fee = estimate_fee(
        "https://ethereum-holesky-rpc.publicnode.com".into(),
        aligned_sdk::core::types::FeeEstimationType::Instant,
    )
    .await
    .expect("Max fee to be retrieved");

    let aligned_verification_data =
        aligned_sdk::sdk::submit(network, &verification_data, max_fee, wallet, 0.into())
            .await
            .expect("Proof to be sent");

    aligned_verification_data
}

pub async fn wait_until_proof_is_aggregated(
    network: aligned_sdk::core::types::Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    proof: &sp1_sdk::SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> Option<Vec<[u8; 32]>> {
    let rpc_url = "";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    let aligned_proof_agg_address =
        Address::from(network.get_aligned_proof_agg_service_address().0);

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

    let mut merkle_path = None;

    while let Some(_) = stream.next().await {
        merkle_path = aligned_sdk::sdk::aggregation::get_merkle_path_for_proof(
            network.clone(),
            eth_rpc_url.clone(),
            beacon_client_url.clone(),
            None,
            &verification_data,
        )
        .await
        .unwrap();
    }

    merkle_path
}
