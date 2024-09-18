use std::sync::Arc;

use ethers::prelude::*;

use crate::core::errors::VerificationError;

abigen!(
    AlignedLayerServiceManagerContract,
    "abi/AlignedLayerServiceManager.json",
    methods {
        verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256) as verify_batch_inclusion_legacy;
        verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address) as verify_batch_inclusion;
    },
);

type AlignedLayerServiceManager = AlignedLayerServiceManagerContract<Provider<Http>>;

pub async fn aligned_service_manager(
    provider: Provider<Http>,
    contract_address: H160,
) -> Result<AlignedLayerServiceManager, VerificationError> {
    let client = Arc::new(provider);

    Ok(AlignedLayerServiceManager::new(contract_address, client))
}
