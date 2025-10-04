use std::str::FromStr;

use aligned_sdk::common::types::AlignedVerificationData;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::LocalSigner,
    sol,
};

use crate::config::EnvConfig;

sol!(
    #[sol(rpc)]
    FibonacciValidator,
    "abi/FibonacciValidator.json"
);

pub async fn update_number_on_contract(
    config: EnvConfig,
    pub_input_bytes: Vec<u8>,
    aligned_verification_data: AlignedVerificationData,
) -> String {
    let rpc_url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
    let signer = LocalSigner::from_str(&config.private_key)
        .expect("Keystore signer should be `cast wallet` compliant");
    let wallet = EthereumWallet::from(signer);

    let rpc_provider = ProviderBuilder::new()
        .wallet(wallet.clone())
        .on_http(rpc_url);
    let fibonacci_contract = FibonacciValidator::new(
        Address::from_str(&config.fibonacci_contract_address)
            .expect("State transition address should be valid"),
        rpc_provider,
    );

    let mut merkle_proof_bytes: Vec<u8> = vec![];

    for leaf in aligned_verification_data.batch_inclusion_proof.merkle_path {
        merkle_proof_bytes.extend_from_slice(&leaf);
    }

    let res = fibonacci_contract
        .setNewNumber(
            aligned_verification_data
                .verification_data_commitment
                .proof_commitment
                .into(),
            pub_input_bytes.into(),
            wallet.default_signer().address().into(),
            aligned_verification_data.batch_merkle_root.into(),
            merkle_proof_bytes.into(),
            U256::from(aligned_verification_data.index_in_batch as u64),
        )
        .send()
        .await
        .expect("tx to not revert");

    res.get_receipt()
        .await
        .expect("To get receipt")
        .transaction_hash
        .to_string()
}
