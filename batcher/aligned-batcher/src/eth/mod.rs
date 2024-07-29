use std::iter::repeat;
use std::str::FromStr;
use std::sync::Arc;

use aligned_sdk::eth::batcher_payment_service::{BatcherPaymentServiceContract, SignatureData};
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use gas_escalator::{Frequency, GeometricGasPrice};
use log::info;

use crate::{config::ECDSAConfig, types::errors::BatcherError};

#[derive(Debug, Clone, EthEvent)]
pub struct BatchVerified {
    pub batch_merkle_root: [u8; 32],
}

pub type BatcherPaymentService = BatcherPaymentServiceContract<
    SignerMiddleware<GasEscalatorMiddleware<Provider<RetryClient<Http>>>, Wallet<SigningKey>>,
>;

const MAX_RETRIES: u32 = 15;
const INITIAL_BACKOFF: u64 = 1000;
const GAS_MULTIPLIER: f64 = 1.125;
const GAS_ESCALATOR_INTERVAL: u64 = 12; // seconds

pub fn get_provider(eth_rpc_url: String) -> Result<Provider<RetryClient<Http>>, anyhow::Error> {
    let provider = Http::from_str(eth_rpc_url.as_str())
        .map_err(|e| anyhow::Error::msg(format!("Failed to create provider: {}", e)))?;

    let client = RetryClient::new(
        provider,
        Box::<ethers::providers::HttpRateLimitRetryPolicy>::default(),
        MAX_RETRIES,
        INITIAL_BACKOFF,
    );

    Ok(Provider::<RetryClient<Http>>::new(client))
}

pub async fn create_new_task(
    payment_service: &BatcherPaymentService,
    batch_merkle_root: [u8; 32],
    batch_data_pointer: String,
    leaves: Vec<[u8; 32]>,
    signatures: Vec<SignatureData>,
    gas_for_aggregator: U256,
    gas_per_proof: U256,
) -> Result<TransactionReceipt, BatcherError> {
    // pad leaves to next power of 2
    let padded_leaves = pad_leaves(leaves);

    let call = payment_service.create_new_task(
        batch_merkle_root,
        batch_data_pointer,
        padded_leaves,
        signatures,
        gas_for_aggregator,
        gas_per_proof,
    );

    info!("Creating task for: {:x?}", batch_merkle_root);

    let pending_tx = call
        .send()
        .await
        .map_err(|e| BatcherError::TaskCreationError(e.to_string()))?;

    let receipt = pending_tx
        .await
        .map_err(|_| BatcherError::TransactionSendError)?
        .ok_or(BatcherError::ReceiptNotFoundError)?;

    Ok(receipt)
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

    let service_manager =
        BatcherPaymentService::new(H160::from_str(contract_address.as_str())?, signer);

    Ok(service_manager)
}

fn pad_leaves(leaves: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let leaves_len = leaves.len();
    let last_leaf = leaves[leaves_len - 1];
    leaves
        .into_iter()
        .chain(repeat(last_leaf).take(leaves_len.next_power_of_two() - leaves_len))
        .collect()
}
