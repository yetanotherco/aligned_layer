use std::sync::Arc;

use ethers::prelude::*;

use crate::core::errors::VerificationError;

abigen!(
    ProofAggregationServiceContract,
    "abi/AlignedProofAggregationService.json"
);

pub type AlignedProofAggregationService = ProofAggregationServiceContract<Provider<Http>>;

pub async fn aligned_proof_aggregation_service(
    provider: Provider<Http>,
    contract_address: H160,
) -> Result<AlignedProofAggregationService, VerificationError> {
    let client = Arc::new(provider);

    // Verify that the contract has code at the given address
    let code = client
        .get_code(contract_address, None)
        .await
        .map_err(|e| VerificationError::EthereumProviderError(e.to_string()))?;
    if code.is_empty() {
        return Err(VerificationError::EthereumNotAContract(contract_address));
    }

    Ok(AlignedProofAggregationService::new(
        contract_address,
        client,
    ))
}
