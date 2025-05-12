use std::str::FromStr;

use alloy::{
    network::EthereumWallet, primitives::Address, providers::ProviderBuilder,
    rpc::types::TransactionReceipt, signers::local::PrivateKeySigner, sol,
};

sol!(
    #[sol(rpc)]
    StateTransition,
    "abi/StateTransition.json"
);

pub async fn send_state_transition_to_chain(
    public_inputs: Vec<u8>,
    merkle_proof: Vec<[u8; 32]>,
    eth_rpc_url: String,
    state_transition_address: String,
    private_key: String,
) -> TransactionReceipt {
    let rpc_url = eth_rpc_url.parse().expect("RPC URL should be valid");
    let signer = PrivateKeySigner::from_str(&private_key)
        .expect("Keystore signer should be `cast wallet` compliant");
    let wallet = EthereumWallet::from(signer);

    let rpc_provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
    let state_transition_contract = StateTransition::new(
        Address::from_str(&state_transition_address)
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
