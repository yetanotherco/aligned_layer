use std::{str::FromStr, sync::Arc};

use aligned_sdk::eth::aligned_service_manager::AlignedLayerServiceManagerContract;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{Signer, Wallet},
    types::H160,
};

use crate::config::ECDSAConfig;

pub type SignerMiddlewareT = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;

pub type ServiceManager = AlignedLayerServiceManagerContract<SignerMiddlewareT>;

pub async fn get_service_manager(
    provider: Provider<Http>,
    ecdsa_config: ECDSAConfig,
    contract_address: String,
) -> Result<ServiceManager, anyhow::Error> {
    let chain_id = provider.get_chainid().await?;

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
