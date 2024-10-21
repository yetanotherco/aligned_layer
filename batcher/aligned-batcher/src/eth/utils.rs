use std::str::FromStr;

use ethers::{
    providers::{Http, Provider, Ws},
    types::U256,
};
use log::warn;

use crate::retry::RetryError;
use ethers::prelude::Middleware;

pub(crate) const GAS_MULTIPLIER: f64 = 1.125; // Multiplier for the gas price for gas escalator
pub(crate) const GAS_ESCALATOR_INTERVAL: u64 = 12; // Time in seconds between gas escalations

pub fn get_provider(eth_rpc_url: String) -> Result<Provider<Http>, anyhow::Error> {
    let provider = Http::from_str(eth_rpc_url.as_str())
        .map_err(|e| anyhow::Error::msg(format!("Failed to create provider: {}", e)))?;
    Ok(Provider::new(provider))
}

pub async fn get_gas_price_retryable(
    eth_ws_provider: &Provider<Ws>,
    eth_ws_provider_fallback: &Provider<Ws>,
) -> Result<U256, RetryError<String>> {
    if let Ok(gas_price) = eth_ws_provider
        .get_gas_price()
        .await
        .inspect_err(|e| warn!("Failed to get gas price. Trying with fallback: {e:?}"))
    {
        return Ok(gas_price);
    }

    eth_ws_provider_fallback.get_gas_price().await.map_err(|e| {
        warn!("Failed to get fallback gas price: {e:?}");
        RetryError::Transient(e.to_string())
    })
}
