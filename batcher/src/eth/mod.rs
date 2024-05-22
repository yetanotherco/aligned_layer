use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;

use crate::config::ECDSAConfig;

abigen!(
    AlignedLayerServiceManagerContract,
    "./src/eth/abi/AlignedLayerServiceManager.json"
);

pub type AlignedLayerServiceManager =
    AlignedLayerServiceManagerContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

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
    service_manager: AlignedLayerServiceManager,
    batch_merkle_root: [u8; 32],
    batch_data_pointer: String,
) -> Result<TransactionReceipt, anyhow::Error> {
    let call = service_manager.create_new_task(batch_merkle_root, batch_data_pointer);
    let pending_tx = call.send().await?;

    match pending_tx.await? {
        Some(receipt) => Ok(receipt),
        None => Err(anyhow::anyhow!("Receipt not found")),
    }
}

pub async fn poll_new_blocks<F, Fut>(eth_ws_url: String, callback: F) -> Result<(), anyhow::Error>
where
    F: Fn(u64) -> Fut,
    Fut: Future,
{
    let provider = Provider::<Ws>::connect(eth_ws_url).await?;
    let mut stream = provider.subscribe_blocks().await?;
    while let Some(block) = stream.next().await {
        let block_number = block.number.unwrap();
        let block_number = u64::try_from(block_number).map_err(|err| anyhow::anyhow!(err))?;
        callback(block_number).await;
    }

    Ok(())
}
