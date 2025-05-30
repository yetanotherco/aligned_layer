use std::str::FromStr;

use alloy::{
    network::EthereumWallet, primitives::Address, providers::ProviderBuilder,
    rpc::types::TransactionReceipt, signers::local::LocalSigner, sol,
};

use crate::config::Config;

sol!(
    #[sol(rpc)]
    StateTransition,
    "abi/StateTransition.json"
);

pub async fn send_state_transition_to_chain(
    config: &Config,
    public_inputs: Vec<u8>,
    merkle_proof: Vec<[u8; 32]>,
) -> TransactionReceipt {
    let rpc_url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
    let signer = LocalSigner::decrypt_keystore(
        &config.private_key_store_path,
        &config.private_key_store_password,
    )
    .expect("Keystore signer should be `cast wallet` compliant");
    let wallet = EthereumWallet::from(signer);

    let rpc_provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
    let state_transition_contract = StateTransition::new(
        Address::from_str(&config.state_transition_contract_address)
            .expect("State transition address should be valid"),
        rpc_provider,
    );

    let merkle_proof = merkle_proof.iter().map(|e| e.into()).collect();

    let res = state_transition_contract
        .updateState(public_inputs.into(), merkle_proof)
        .send()
        .await
        .expect("State transition tx to not revert");

    res.get_receipt().await.expect("To get receipt")
}
