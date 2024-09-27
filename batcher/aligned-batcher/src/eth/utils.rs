use std::str::FromStr;

use ethers::providers::{Http, Provider, RetryClient};

const MAX_RETRIES: u32 = 15; // Max retries for the retry client. Will only retry on network errors
const INITIAL_BACKOFF: u64 = 1000; // Initial backoff for the retry client in milliseconds, will increase every retry
pub(crate) const GAS_MULTIPLIER: f64 = 1.125; // Multiplier for the gas price for gas escalator
pub(crate) const GAS_ESCALATOR_INTERVAL: u64 = 12; // Time in seconds between gas escalations

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
