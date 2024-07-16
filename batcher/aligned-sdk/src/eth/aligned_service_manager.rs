use std::str::FromStr;
use std::sync::Arc;

use ethers::prelude::*;

use crate::core::errors::VerificationError;

abigen!(
    AlignedLayerServiceManagerContract,
    "abi/AlignedLayerServiceManager.json"
);

type AlignedLayerServiceManager = AlignedLayerServiceManagerContract<Provider<Http>>;

pub async fn aligned_service_manager(
    provider: Provider<Http>,
    contract_address: &str,
) -> Result<AlignedLayerServiceManager, VerificationError> {
    let client = Arc::new(provider);
    let contract_addr = H160::from_str(contract_address)
        .map_err(|e| VerificationError::HexDecodingError(e.to_string()))?;

    Ok(AlignedLayerServiceManager::new(contract_addr, client))
}
