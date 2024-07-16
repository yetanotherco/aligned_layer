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
    pub fn new(signature: &Signature, nonce: [u8; 32]) -> Self {
        let mut r = [0u8; 32];
        signature.r.to_big_endian(&mut r);

        let mut s = [0u8; 32];
        signature.s.to_big_endian(&mut s);

        let nonce = U256::from_big_endian(nonce.as_slice());

        SignatureData {
            v: signature.v as u8,
            r,
            s,
            nonce,
        }
    }
}
