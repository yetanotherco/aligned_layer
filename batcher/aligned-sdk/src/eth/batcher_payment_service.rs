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
    contract_address: H160,
) -> Result<BatcherPaymentService, VerificationError> {
    let client = Arc::new(provider);

    // Verify that the contract has code at the given address
    let code = client
        .get_code(contract_address, None)
        .await
        .map_err(|e| VerificationError::EthereumProviderError(e.to_string()))?;
    if code.is_empty() {
        return Err(VerificationError::EthereumNotAContract(contract_address));
    }

    Ok(BatcherPaymentService::new(contract_address, client))
}

impl SignatureData {
    pub fn new(signature: &Signature, nonce: U256, max_fee: U256) -> Self {
        let mut signature_bytes = [0u8; 65];

        signature.r.to_big_endian(&mut signature_bytes[0..32]);

        signature.s.to_big_endian(&mut signature_bytes[32..64]);

        signature_bytes[64] = signature.v as u8;

        let signature_bytes = Bytes::from(signature_bytes);

        SignatureData {
            signature: signature_bytes,
            nonce,
            max_fee,
        }
    }
}
