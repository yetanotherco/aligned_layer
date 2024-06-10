use crate::eth::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

abigen!(
    AlignedLayerServiceManagerContract,
    "abi/AlignedLayerServiceManager.json"
);

pub type AlignedLayerServiceManager =
    AlignedLayerServiceManagerContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub async fn get_contract(
    provider: Provider<Http>,
    contract_address: &str,
) -> AlignedLayerServiceManager {
    // ) -> Result<AlignedLayerServiceManager, anyhow::Error> {

    let private_key_store_path = "anvil.ecdsa.key.json";
    let private_key_store_password = "";
    // let eth_rpc_url = "http://localhost:8545";

    // let eth_rpc_provider =
    //     Provider::<Http>::try_from(eth_rpc_url).unwrap();

    let chain_id = provider.get_chainid().await.unwrap();

    // get private key from keystore
    let wallet = Wallet::decrypt_keystore(private_key_store_path, private_key_store_password)
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    let service_manager =
        AlignedLayerServiceManager::new(H160::from_str(contract_address).unwrap(), signer);

    service_manager
}

pub async fn verify_inclusion(
    service_manager: &AlignedLayerServiceManager,
    batch_merkle_root: [u8; 32],
    proof_commitment: [u8; 32],
    // ) -> Result<TransactionReceipt, anyhow::Error> {
) -> TransactionReceipt {
    let call = service_manager.verify_batch_inclusion(batch_merkle_root, proof_commitment);
    let pending_tx = call.send().await.unwrap();

    match pending_tx.await.unwrap() {
        Some(receipt) => receipt,
        None => panic!(),
    }
}
