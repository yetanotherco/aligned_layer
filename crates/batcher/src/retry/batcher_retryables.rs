use std::time::Duration;

use ethers::prelude::*;
use log::{info, warn};
use tokio::time::timeout;

use crate::{
    eth::{
        payment_service::{BatcherPaymentService, CreateNewTaskFeeParams, SignerMiddlewareT},
        utils::get_current_nonce,
    },
    retry::RetryError,
    types::errors::{BatcherError, TransactionSendError},
};

pub async fn get_user_balance_retryable(
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
    addr: &Address,
) -> Result<U256, RetryError<String>> {
    if let Ok(balance) = payment_service.user_balances(*addr).call().await {
        return Ok(balance);
    };

    payment_service_fallback
        .user_balances(*addr)
        .call()
        .await
        .map_err(|e| {
            warn!("Failed to get balance for address {:?}. Error: {e}", addr);
            RetryError::Transient(e.to_string())
        })
}

pub async fn get_user_nonce_from_ethereum_retryable(
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
    addr: Address,
) -> Result<U256, RetryError<String>> {
    if let Ok(nonce) = payment_service.user_nonces(addr).call().await {
        return Ok(nonce);
    }
    payment_service_fallback
        .user_nonces(addr)
        .call()
        .await
        .map_err(|e| {
            warn!("Error getting user nonce: {e}");
            RetryError::Transient(e.to_string())
        })
}

pub async fn get_current_nonce_retryable(
    eth_http_provider: &Provider<Http>,
    eth_http_provider_fallback: &Provider<Http>,
    addr: Address,
) -> Result<U256, RetryError<ProviderError>> {
    match eth_http_provider.get_transaction_count(addr, None).await {
        Ok(current_nonce) => Ok(current_nonce),
        Err(_) => eth_http_provider_fallback
            .get_transaction_count(addr, None)
            .await
            .map_err(|e| {
                warn!("Error getting user nonce: {e}");
                RetryError::Transient(e)
            }),
    }
}

pub async fn user_balance_is_unlocked_retryable(
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
    addr: &Address,
) -> Result<bool, RetryError<()>> {
    if let Ok(unlock_block) = payment_service.user_unlock_block(*addr).call().await {
        return Ok(unlock_block != U256::zero());
    }
    if let Ok(unlock_block) = payment_service_fallback
        .user_unlock_block(*addr)
        .call()
        .await
    {
        return Ok(unlock_block != U256::zero());
    }
    warn!("Failed to get user locking state {:?}", addr);
    Err(RetryError::Transient(()))
}

pub async fn get_gas_price_retryable(
    eth_http_provider: &Provider<Http>,
    eth_http_provider_fallback: &Provider<Http>,
) -> Result<U256, RetryError<ProviderError>> {
    match eth_http_provider.get_gas_price().await {
        Ok(gas_price) => Ok(gas_price),
        Err(_) => eth_http_provider_fallback
            .get_gas_price()
            .await
            .map_err(|e| {
                warn!("Failed to get fallback gas price: {e:?}");
                RetryError::Transient(e)
            }),
    }
}

pub async fn create_new_task_retryable(
    batch_merkle_root: [u8; 32],
    batch_data_pointer: String,
    proofs_submitters: Vec<Address>,
    fee_params: CreateNewTaskFeeParams,
    transaction_wait_timeout: u64,
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
) -> Result<TransactionReceipt, RetryError<BatcherError>> {
    info!("Creating task for: 0x{}", hex::encode(batch_merkle_root));
    let call_fallback;
    let call = payment_service
        .create_new_task(
            batch_merkle_root,
            batch_data_pointer.clone(),
            proofs_submitters.clone(),
            fee_params.fee_for_aggregator,
            fee_params.fee_per_proof,
            fee_params.respond_to_task_fee_limit,
        )
        .gas_price(fee_params.gas_price);

    let pending_tx = match call.send().await {
        Ok(pending_tx) => pending_tx,
        Err(ContractError::Revert(err)) => {
            // Since transaction was reverted, we don't want to retry with fallback.
            warn!("Transaction reverted {:?}", err);
            return Err(RetryError::Permanent(BatcherError::TransactionSendError(
                TransactionSendError::from(err),
            )));
        }
        _ => {
            call_fallback = payment_service_fallback
                .create_new_task(
                    batch_merkle_root,
                    batch_data_pointer,
                    proofs_submitters,
                    fee_params.fee_for_aggregator,
                    fee_params.fee_per_proof,
                    fee_params.respond_to_task_fee_limit,
                )
                .gas_price(fee_params.gas_price);
            match call_fallback.send().await {
                Ok(pending_tx) => pending_tx,
                Err(ContractError::Revert(err)) => {
                    warn!("Transaction reverted {:?}", err);
                    return Err(RetryError::Permanent(BatcherError::TransactionSendError(
                        TransactionSendError::from(err),
                    )));
                }
                Err(err) => {
                    return Err(RetryError::Transient(BatcherError::TransactionSendError(
                        TransactionSendError::Generic(err.to_string()),
                    )))
                }
            }
        }
    };

    // timeout to prevent a deadlock while waiting for the transaction to be included in a block.
    timeout(Duration::from_millis(transaction_wait_timeout), pending_tx)
        .await
        .map_err(|e| {
            warn!("Error while waiting for batch inclusion: {e}");
            RetryError::Permanent(BatcherError::ReceiptNotFoundError)
        })?
        .map_err(|e| {
            warn!("Error while waiting for batch inclusion: {e}");
            RetryError::Permanent(BatcherError::ReceiptNotFoundError)
        })?
        .ok_or(RetryError::Permanent(BatcherError::ReceiptNotFoundError))
}

pub async fn simulate_create_new_task_retryable(
    batch_merkle_root: [u8; 32],
    batch_data_pointer: String,
    proofs_submitters: Vec<Address>,
    fee_params: CreateNewTaskFeeParams,
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
) -> Result<(), RetryError<BatcherError>> {
    info!("Simulating task for: 0x{}", hex::encode(batch_merkle_root));
    let simulation_fallback;
    let simulation = payment_service
        .create_new_task(
            batch_merkle_root,
            batch_data_pointer.clone(),
            proofs_submitters.clone(),
            fee_params.fee_for_aggregator,
            fee_params.fee_per_proof,
            fee_params.respond_to_task_fee_limit,
        )
        .gas_price(fee_params.gas_price);
    // sends an `eth_call` request to the node
    match simulation.call().await {
        Ok(_) => {
            info!(
                "Simulation task for: 0x{} succeeded.",
                hex::encode(batch_merkle_root)
            );
            Ok(())
        }
        Err(ContractError::Revert(err)) => {
            // Since transaction was reverted, we don't want to retry with fallback.
            warn!("Simulated transaction reverted {:?}", err);
            Err(RetryError::Permanent(BatcherError::TransactionSendError(
                TransactionSendError::from(err),
            )))
        }
        _ => {
            simulation_fallback = payment_service_fallback
                .create_new_task(
                    batch_merkle_root,
                    batch_data_pointer,
                    proofs_submitters,
                    fee_params.fee_for_aggregator,
                    fee_params.fee_per_proof,
                    fee_params.respond_to_task_fee_limit,
                )
                .gas_price(fee_params.gas_price);
            match simulation_fallback.call().await {
                Ok(_) => Ok(()),
                Err(ContractError::Revert(err)) => {
                    warn!("Simulated transaction reverted {:?}", err);
                    Err(RetryError::Permanent(BatcherError::TransactionSendError(
                        TransactionSendError::from(err),
                    )))
                }
                Err(err) => Err(RetryError::Transient(BatcherError::TransactionSendError(
                    TransactionSendError::Generic(err.to_string()),
                ))),
            }
        }
    }
}

pub async fn cancel_create_new_task_retryable(
    batcher_signer: &SignerMiddlewareT,
    batcher_signer_fallback: &SignerMiddlewareT,
    bumped_gas_price: U256,
    transaction_wait_timeout: u64,
) -> Result<TransactionReceipt, RetryError<ProviderError>> {
    let batcher_addr = batcher_signer.address();

    let current_nonce = get_current_nonce(
        batcher_signer.provider(),
        batcher_signer_fallback.provider(),
        batcher_addr,
    )
    .await
    .map_err(RetryError::Transient)?;

    let tx = TransactionRequest::new()
        .to(batcher_addr)
        .value(U256::zero())
        .nonce(current_nonce)
        .gas_price(bumped_gas_price);

    let pending_tx = match batcher_signer.send_transaction(tx.clone(), None).await {
        Ok(pending_tx) => pending_tx,
        Err(_) => batcher_signer_fallback
            .send_transaction(tx.clone(), None)
            .await
            .map_err(|e| RetryError::Transient(ProviderError::CustomError(e.to_string())))?,
    };

    // timeout to prevent a deadlock while waiting for the transaction to be included in a block.
    timeout(Duration::from_millis(transaction_wait_timeout), pending_tx)
        .await
        .map_err(|e| {
            warn!("Timeout while waiting for transaction inclusion: {e}");
            RetryError::Transient(ProviderError::CustomError(format!(
                "Timeout while waiting for transaction inclusion: {e}"
            )))
        })?
        .map_err(|e| {
            warn!("Error while waiting for tx inclusion: {e}");
            RetryError::Transient(e)
        })?
        .ok_or(RetryError::Transient(ProviderError::CustomError(
            "Receipt not found".to_string(),
        )))
}
