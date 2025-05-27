use std::str::FromStr;
use std::sync::Arc;

use aligned_sdk::eth::batcher_payment_service::BatcherPaymentServiceContract;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;

#[derive(Debug, Clone, EthEvent)]
pub struct BatchVerified {
    pub batch_merkle_root: [u8; 32],
}

pub type SignerMiddlewareT = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;

pub type BatcherPaymentService = BatcherPaymentServiceContract<SignerMiddlewareT>;

#[derive(Debug, Clone)]
pub struct CreateNewTaskFeeParams {
    pub fee_for_aggregator: U256,
    pub fee_per_proof: U256,
    pub gas_price: U256,
    pub respond_to_task_fee_limit: U256,
}

impl CreateNewTaskFeeParams {
    pub fn new(
        fee_for_aggregator: U256,
        fee_per_proof: U256,
        gas_price: U256,
        respond_to_task_fee_limit: U256,
    ) -> Self {
        CreateNewTaskFeeParams {
            fee_for_aggregator,
            fee_per_proof,
            gas_price,
            respond_to_task_fee_limit,
        }
    }
}

pub async fn get_batcher_payment_service(
    signer: Arc<SignerMiddlewareT>,
    contract_address: String,
) -> Result<BatcherPaymentService, anyhow::Error> {
    let payment_service =
        BatcherPaymentService::new(H160::from_str(contract_address.as_str())?, signer);
    Ok(payment_service)
}
