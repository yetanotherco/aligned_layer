use std::str::FromStr;
use std::sync::Arc;
use ethers::prelude::*;
use ethers::prelude::k256::ecdsa::SigningKey;

abigen!(AlignedLayerServiceManagerContract, "./src/eth/abi/AlignedLayerServiceManager.json");

pub type AlignedLayerServiceManager = AlignedLayerServiceManagerContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub async fn get_contract() -> Result<AlignedLayerServiceManager, anyhow::Error> {
    let provider = Provider::<Http>::try_from("http://localhost:8545")?;
    let chain_id = provider.get_chainid().await?;

    let wallet = Wallet::from_str("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")?
        .with_chain_id(chain_id.as_u64());

    let signer =
        Arc::new(SignerMiddleware::new(provider, wallet));


    let service_manager = AlignedLayerServiceManager::new(
        H160::from_str("0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690")?,
        signer,
    );

    Ok(service_manager)
}

pub async fn create_new_task(service_manager: AlignedLayerServiceManager, merkle_root: [u8; 32], batch_data_pointer: String) -> Result<TransactionReceipt, anyhow::Error> {
    let call = service_manager.create_new_task(merkle_root, batch_data_pointer);
    let pending_tx = call.send().await?;

    match pending_tx.await? {
        Some(receipt) => Ok(receipt),
        None => Err(anyhow::anyhow!("Receipt not found")),
    }
}