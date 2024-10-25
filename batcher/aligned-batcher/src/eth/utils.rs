use std::str::FromStr;
use std::sync::Arc;

use crate::config::ECDSAConfig;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};

use super::payment_service::SignerMiddlewareT;
use gas_escalator::{Frequency, GeometricGasPrice};

pub(crate) const GAS_MULTIPLIER: f64 = 1.125; // Multiplier for the gas price for gas escalator
pub(crate) const GAS_ESCALATOR_INTERVAL: u64 = 12; // Time in seconds between gas escalations

pub fn get_provider(eth_rpc_url: String) -> Result<Provider<Http>, anyhow::Error> {
    let provider = Http::from_str(eth_rpc_url.as_str())
        .map_err(|e| anyhow::Error::msg(format!("Failed to create provider: {}", e)))?;
    Ok(Provider::new(provider))
}

pub async fn get_batcher_signer(
    provider: Provider<Http>,
    ecdsa_config: ECDSAConfig,
) -> Result<Arc<SignerMiddlewareT>, anyhow::Error> {
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
    Ok(signer)
}
