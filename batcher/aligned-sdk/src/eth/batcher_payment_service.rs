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
