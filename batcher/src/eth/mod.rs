use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use stream::EventStream;
use tokio::time::timeout;

use crate::config::ECDSAConfig;

abigen!(
    AlignedLayerServiceManagerContract,
    "./src/eth/abi/AlignedLayerServiceManager.json"
);

#[derive(Debug, Clone, EthEvent)]
pub struct BatchVerified {
    pub batch_merkle_root: [u8; 32],
}

pub type AlignedLayerServiceManager =
    AlignedLayerServiceManagerContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub type BatchVerifiedEventStream<'s> = EventStream<
    's,
    FilterWatcher<'s, Http, Log>,
    BatchVerifiedFilter,
    ContractError<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
>;

pub fn get_provider(eth_rpc_url: String) -> Result<Provider<Http>, anyhow::Error> {
    Provider::<Http>::try_from(eth_rpc_url).map_err(|err| anyhow::anyhow!(err))
}

pub async fn get_contract(
    provider: Provider<Http>,
    ecdsa_config: ECDSAConfig,
    contract_address: String,
) -> Result<AlignedLayerServiceManager, anyhow::Error> {
    let chain_id = provider.get_chainid().await?;

    // get private key from keystore
    let wallet = Wallet::decrypt_keystore(
        &ecdsa_config.private_key_store_path,
        &ecdsa_config.private_key_store_password,
    )?
    .with_chain_id(chain_id.as_u64());

    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    let service_manager =
        AlignedLayerServiceManager::new(H160::from_str(contract_address.as_str())?, signer);

    Ok(service_manager)
}

pub async fn create_new_task(
    service_manager: &AlignedLayerServiceManager,
    batch_merkle_root: [u8; 32],
    batch_data_pointer: String,
) -> Result<TransactionReceipt, anyhow::Error> {
    let call = service_manager.create_new_task(batch_merkle_root, batch_data_pointer);
    let pending_tx = call.send().await?;

    // Set a timeout for the transaction to be mined
    match timeout(Duration::from_secs(60), pending_tx).await {
        Ok(Ok(Some(receipt))) => Ok(receipt),
        Ok(Ok(None)) => Err(anyhow::anyhow!("Transaction was mined but no receipt was returned")),
        Ok(Err(e)) => Err(anyhow::anyhow!("Transaction failed: {:?}", e)),
        Err(_) => Err(anyhow::anyhow!("Transaction was not mined within 60 seconds")),
    }
}
