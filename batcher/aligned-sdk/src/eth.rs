use std::str::FromStr;
use std::sync::Arc;

use ethers::prelude::*;

use crate::errors::VerificationError;

abigen!(
    AlignedLayerServiceManagerContract,
    "abi/AlignedLayerServiceManager.json"
);

pub type AlignedLayerServiceManager = AlignedLayerServiceManagerContract<Provider<Http>>;

pub async fn aligned_service_manager(
    provider: Provider<Http>,
    contract_address: &str,
) -> Result<AlignedLayerServiceManager, VerificationError> {
    let client = Arc::new(provider);
    let contract_addr = H160::from_str(contract_address)
        .map_err(|e| VerificationError::ParsingError(e.to_string()))?;

    Ok(AlignedLayerServiceManager::new(contract_addr, client))
}
