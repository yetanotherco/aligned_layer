use std::str::FromStr;
use std::sync::Arc;

use aligned_sdk::eth::batcher_payment_service::BatcherPaymentServiceContract;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use gas_escalator::{Frequency, GeometricGasPrice};

use crate::config::ECDSAConfig;

#[derive(Debug, Clone, EthEvent)]
pub struct BatchVerified {
    pub batch_merkle_root: [u8; 32],
}

pub type BatcherPaymentService = BatcherPaymentServiceContract<
    SignerMiddleware<GasEscalatorMiddleware<Provider<RetryClient<Http>>>, Wallet<SigningKey>>,
>;

const MAX_RETRIES: u32 = 15; // Max retries for the retry client. Will only retry on network errors
const INITIAL_BACKOFF: u64 = 1000; // Initial backoff for the retry client in milliseconds, will increase every retry
const GAS_MULTIPLIER: f64 = 1.125; // Multiplier for the gas price for gas escalator
const GAS_ESCALATOR_INTERVAL: u64 = 12; // Time in seconds between gas escalations

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
