use aligned_sdk::core::{
    constants::INSTANT_MAX_FEE_BATCH_SIZE,
    types::{AlignedVerificationData, Signer, SigningKey, VerificationData, Wallet},
};
use primitive_types::U256;

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

    let aligned_verification_data = aligned_sdk::sdk::submit(
        network,
        &verification_data,
        U256::from(INSTANT_MAX_FEE_BATCH_SIZE as u64),
        wallet,
        nonce,
    )
    .await
    .expect("Proof to be sent");

    aligned_verification_data
}

pub async fn wait_until_proof_is_aggregated() -> u64 {
    // let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/your-api-key";
    // let ws = WsConnect::new(rpc_url);
    // let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

    // let uniswap_token_address =
    //     Address::from_str("1f9840a85d5aF5bf1D1762F925BDADdC4201F984").unwrap();

    // let filter = Filter::new()
    //     .address(uniswap_token_address)
    //     // By specifying an `event` or `event_signature` we listen for a specific event of the
    //     // contract. In this case the `Transfer(address,address,uint256)` event.
    //     .event("Transfer(address,address,uint256)")
    //     .from_block(BlockNumberOrTag::Latest);

    // // Subscribe to logs.
    // let sub = provider.subscribe_logs(&filter).await.unwrap();
    // let mut stream = sub.into_stream();

    // while let Some(log) = stream.next().await {
    //     println!("Uniswap token logs: {log:?}");
    // }

    0
}
