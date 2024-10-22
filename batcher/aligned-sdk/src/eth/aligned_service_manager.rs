use std::sync::Arc;

use ethers::prelude::*;

use crate::core::errors::VerificationError;

abigen!(
    AlignedLayerServiceManagerContract,
    "abi/AlignedLayerServiceManager.json",
    methods {
        verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256) as verify_batch_inclusion_legacy;
        verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address) as verify_batch_inclusion;
        disabledVerifiers() as disabled_verifiers;
    },
);

type AlignedLayerServiceManager = AlignedLayerServiceManagerContract<Provider<Http>>;

pub async fn aligned_service_manager(
    provider: Provider<Http>,
    contract_address: H160,
) -> Result<AlignedLayerServiceManager, VerificationError> {
    let client = Arc::new(provider);

    // Verify that the contract has code at the given address
    let code = client
        .get_code(contract_address, None)
        .await
        .map_err(|e| VerificationError::EthereumProviderError(e.to_string()))?;
    if code.is_empty() {
        return Err(VerificationError::EthereumNotAContract(contract_address));
    }

    Ok(AlignedLayerServiceManager::new(contract_address, client))
}
