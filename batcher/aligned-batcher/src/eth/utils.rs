use std::str::FromStr;
use std::sync::Arc;

use crate::{
    config::ECDSAConfig,
    retry::{
        batcher_retryables::{get_current_nonce_retryable, get_gas_price_retryable},
        retry_function,
    },
};
use aligned_sdk::common::constants::{
    ETHEREUM_CALL_BACKOFF_FACTOR, ETHEREUM_CALL_MAX_RETRIES, ETHEREUM_CALL_MAX_RETRY_DELAY,
    ETHEREUM_CALL_MIN_RETRY_DELAY, GAS_PRICE_INCREMENT_PERCENTAGE_PER_ITERATION,
    OVERRIDE_GAS_PRICE_PERCENTAGE_MULTIPLIER, PERCENTAGE_DIVIDER,
};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use log::error;

use super::payment_service::SignerMiddlewareT;

pub fn get_provider(eth_rpc_url: String) -> Result<Provider<Http>, anyhow::Error> {
    let provider = Http::from_str(eth_rpc_url.as_str())
        .map_err(|e| anyhow::Error::msg(format!("Failed to create provider: {}", e)))?;
    Ok(Provider::new(provider))
}

pub async fn get_batcher_signer(
    provider: Provider<Http>,
    ecdsa_config: ECDSAConfig,
) -> anyhow::Result<Arc<SignerMiddlewareT>> {
    let chain_id = provider.get_chainid().await?;

    // get private key from keystore
    let wallet = Wallet::decrypt_keystore(
        &ecdsa_config.private_key_store_path,
        &ecdsa_config.private_key_store_password,
    )?
    .with_chain_id(chain_id.as_u64());

    let signer = Arc::new(SignerMiddleware::new(provider, wallet));
    Ok(signer)
}

/// Calculates an increased gas price for retrying a transaction override.
/// The gas price rises with each retry by applying a multiplier based on the iteration count.
pub fn calculate_bumped_gas_price(
    previous_gas_price: U256,
    current_gas_price: U256,
    iteration: usize,
) -> U256 {
    let override_gas_multiplier = U256::from(OVERRIDE_GAS_PRICE_PERCENTAGE_MULTIPLIER)
        + (GAS_PRICE_INCREMENT_PERCENTAGE_PER_ITERATION * iteration);
    let bumped_previous_gas_price =
        previous_gas_price * override_gas_multiplier / U256::from(PERCENTAGE_DIVIDER);

    let bumped_current_gas_price =
        current_gas_price * override_gas_multiplier / U256::from(PERCENTAGE_DIVIDER);
    // Return the maximum of the previous and current gas prices
    // to avoid sending a transaction with a gas price lower than the previous one.
    bumped_current_gas_price.max(bumped_previous_gas_price)
}

/// Gets the current nonce from Ethereum.
/// Retries on recoverable errors using exponential backoff up to `ETHEREUM_CALL_MAX_RETRIES` times:
/// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs).
pub async fn get_current_nonce(
    eth_http_provider: &Provider<Http>,
    eth_http_provider_fallback: &Provider<Http>,
    addr: H160,
) -> Result<U256, ProviderError> {
    retry_function(
        || get_current_nonce_retryable(eth_http_provider, eth_http_provider_fallback, addr),
        ETHEREUM_CALL_MIN_RETRY_DELAY,
        ETHEREUM_CALL_BACKOFF_FACTOR,
        ETHEREUM_CALL_MAX_RETRIES,
        ETHEREUM_CALL_MAX_RETRY_DELAY,
    )
    .await
    .map_err(|e| {
        error!("Could't get nonce: {:?}", e);
        e.inner()
    })
}

/// Gets the current gas price from Ethereum.
/// Retries on recoverable errors using exponential backoff up to `ETHEREUM_CALL_MAX_RETRIES` times:
/// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs).
pub async fn get_gas_price(
    eth_http_provider: &Provider<Http>,
    eth_http_provider_fallback: &Provider<Http>,
) -> Result<U256, ProviderError> {
    retry_function(
        || get_gas_price_retryable(eth_http_provider, eth_http_provider_fallback),
        ETHEREUM_CALL_MIN_RETRY_DELAY,
        ETHEREUM_CALL_BACKOFF_FACTOR,
        ETHEREUM_CALL_MAX_RETRIES,
        ETHEREUM_CALL_MAX_RETRY_DELAY,
    )
    .await
    .map_err(|e| {
        error!("Could't get gas price: {e}");
        e.inner()
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bumped_gas_price_initial_iteration() {
        let previous_gas_price = U256::from(1000);
        let current_gas_price = U256::from(1200);
        let iteration = 0;

        let expected = U256::from(1440); // (1200 * (120 + 0)) / 100

        assert_eq!(
            calculate_bumped_gas_price(previous_gas_price, current_gas_price, iteration),
            expected
        );
    }

    #[test]
    fn test_get_bumped_gas_price_with_iteration() {
        let previous_gas_price = U256::from(1000);
        let current_gas_price = U256::from(1200);
        let iteration = 2;

        let expected = U256::from(1560); // (1200 * (120 + 10) / 100

        assert_eq!(
            calculate_bumped_gas_price(previous_gas_price, current_gas_price, iteration),
            expected
        );
    }

    #[test]
    fn test_get_bumped_gas_price_previous_higher() {
        let previous_gas_price = U256::from(1500);
        let current_gas_price = U256::from(1200);
        let iteration = 1;

        let expected = U256::from(1875); // (1500 * (120 + 5) / 100

        assert_eq!(
            calculate_bumped_gas_price(previous_gas_price, current_gas_price, iteration),
            expected
        );
    }
}
