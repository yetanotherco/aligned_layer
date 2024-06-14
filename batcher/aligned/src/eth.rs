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
    let client = Arc::new(provider);
    let contract_addr = H160::from_str(contract_address)
        .map_err(|e| BatcherClientError::EthError(e.to_string()))?;

    Ok(AlignedLayerServiceManager::new(contract_addr, client))
}
