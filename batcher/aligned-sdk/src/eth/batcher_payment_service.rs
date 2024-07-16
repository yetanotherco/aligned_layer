use std::str::FromStr;
use std::sync::Arc;

use ethers::prelude::*;

use crate::core::errors::VerificationError;

abigen!(
    BatcherPaymentServiceContract,
    "abi/BatcherPaymentService.json"
);

pub type BatcherPaymentService = BatcherPaymentServiceContract<Provider<Http>>;

pub async fn batcher_payment_service(
    provider: Provider<Http>,
    contract_address: &str,
) -> Result<BatcherPaymentService, VerificationError> {
    let client = Arc::new(provider);
    let contract_addr = H160::from_str(contract_address)
        .map_err(|e| VerificationError::HexDecodingError(e.to_string()))?;

    Ok(BatcherPaymentService::new(contract_addr, client))
}
