use std::str::FromStr;
use std::sync::Arc;

use aligned_sdk::eth::batcher_payment_service::BatcherPaymentServiceContract;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use gas_escalator::{Frequency, GeometricGasPrice};
use log::info;

use crate::{config::ECDSAConfig, types::errors::BatcherSendError};

use super::utils::{GAS_ESCALATOR_INTERVAL, GAS_MULTIPLIER};

#[derive(Debug, Clone, EthEvent)]
pub struct BatchVerified {
    pub batch_merkle_root: [u8; 32],
}

pub type SignerMiddlewareT =
    SignerMiddleware<GasEscalatorMiddleware<Provider<RetryClient<Http>>>, Wallet<SigningKey>>;

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
    provider: Provider<RetryClient<Http>>,
    ecdsa_config: ECDSAConfig,
    contract_address: String,
) -> Result<BatcherPaymentService, anyhow::Error> {
    let chain_id = provider.get_chainid().await?;

    let escalator = GeometricGasPrice::new(GAS_MULTIPLIER, GAS_ESCALATOR_INTERVAL, None::<u64>);

    let provider = GasEscalatorMiddleware::new(provider, escalator, Frequency::PerBlock);

    // get private key from keystore
    let wallet = Wallet::decrypt_keystore(
        &ecdsa_config.private_key_store_path,
        &ecdsa_config.private_key_store_password,
    )?
    .with_chain_id(chain_id.as_u64());

    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    let payment_service =
        BatcherPaymentService::new(H160::from_str(contract_address.as_str())?, signer);

    Ok(payment_service)
}

pub async fn try_create_new_task(
    batch_merkle_root: [u8; 32],
    batch_data_pointer: String,
    proofs_submitters: Vec<Address>,
    fee_params: CreateNewTaskFeeParams,
    payment_service: &BatcherPaymentService,
) -> Result<TransactionReceipt, BatcherSendError> {
    let call = payment_service
        .create_new_task(
            batch_merkle_root,
            batch_data_pointer,
            proofs_submitters,
            fee_params.fee_for_aggregator,
            fee_params.fee_per_proof,
            fee_params.respond_to_task_fee_limit,
        )
        .gas_price(fee_params.gas_price);

    info!("Creating task for: {}", hex::encode(batch_merkle_root));

    let pending_tx = call.send().await.map_err(|err| match err {
        ContractError::Revert(err) => BatcherSendError::TransactionReverted(err.to_string()),
        _ => BatcherSendError::UnknownError(err.to_string()),
    })?;

    pending_tx
        .await
        .map_err(|err| BatcherSendError::UnknownError(err.to_string()))?
        .ok_or(BatcherSendError::ReceiptNotFound)
}
