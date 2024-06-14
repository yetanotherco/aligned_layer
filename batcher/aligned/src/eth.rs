use crate::eth::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use crate::errors::BatcherClientError;

abigen!(
    AlignedLayerServiceManagerContract,
    "abi/AlignedLayerServiceManager.json"
);

pub type AlignedLayerServiceManager =
    AlignedLayerServiceManagerContract<Provider<Http>>;

pub async fn aligned_service_manager(
    provider: Provider<Http>,
    contract_address: &str,
) -> Result<AlignedLayerServiceManager, BatcherClientError> {
    let chain_id = provider.get_chainid().await?;

    // let wallet = Wallet::decrypt_keystore(private_key_store_path, private_key_store_password)?
    //     .with_chain_id(chain_id.as_u64());

    // let signer = Arc::new(SignerMiddleware::new(provider, wallet));
    let client = Arc::new(provider);

    let contract_addr = H160::from_str(contract_address)
        .map_err(|e| BatcherClientError::EthError(e.to_string()))?;

    Ok(AlignedLayerServiceManager::new(contract_addr, client))
}
