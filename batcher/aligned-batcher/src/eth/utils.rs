use std::str::FromStr;

use ethers::providers::{Http, Provider};

pub(crate) const GAS_MULTIPLIER: f64 = 1.125; // Multiplier for the gas price for gas escalator
pub(crate) const GAS_ESCALATOR_INTERVAL: u64 = 12; // Time in seconds between gas escalations

pub fn get_provider(eth_rpc_url: String) -> Result<Provider<Http>, anyhow::Error> {
    let provider = Http::from_str(eth_rpc_url.as_str())
        .map_err(|e| anyhow::Error::msg(format!("Failed to create provider: {}", e)))?;
    Ok(Provider::new(provider))
}
