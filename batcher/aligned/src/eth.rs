use crate::eth::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

abigen!(
    AlignedLayerServiceManagerContract,
    "abi/AlignedLayerServiceManager.json"
);

pub type AlignedLayerServiceManager =
    AlignedLayerServiceManagerContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub async fn aligned_service_manager(
    provider: Provider<Http>,
    contract_address: &str,
    private_key_store_path: PathBuf,
    private_key_store_password: &str,
) -> AlignedLayerServiceManager {
    let chain_id = provider.get_chainid().await.unwrap();
    let wallet = Wallet::decrypt_keystore(private_key_store_path, private_key_store_password)
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let signer = Arc::new(SignerMiddleware::new(provider, wallet));
    AlignedLayerServiceManager::new(H160::from_str(contract_address).unwrap(), signer)
}
