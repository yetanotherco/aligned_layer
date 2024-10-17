use std::{str::FromStr, sync::Arc};

use aligned_sdk::eth::aligned_service_manager::AlignedLayerServiceManagerContract;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::{
        gas_escalator::{Frequency, GeometricGasPrice},
        GasEscalatorMiddleware, SignerMiddleware,
    },
    providers::{Http, Middleware, Provider, RetryClient},
    signers::{Signer, Wallet},
    types::H160,
};

use crate::config::ECDSAConfig;

use super::utils::{GAS_ESCALATOR_INTERVAL, GAS_MULTIPLIER};

pub type SignerMiddlewareT =
    SignerMiddleware<GasEscalatorMiddleware<Provider<RetryClient<Http>>>, Wallet<SigningKey>>;

pub type ServiceManager = AlignedLayerServiceManagerContract<SignerMiddlewareT>;

pub async fn get_service_manager(
    provider: Provider<RetryClient<Http>>,
    ecdsa_config: ECDSAConfig,
    contract_address: String,
) -> Result<ServiceManager, anyhow::Error> {
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

    let service_manager = ServiceManager::new(H160::from_str(contract_address.as_str())?, signer);

    Ok(service_manager)
}
