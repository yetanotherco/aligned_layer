use std::iter::repeat;
use std::str::FromStr;
use std::sync::Arc;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use stream::EventStream;

use crate::config::ECDSAConfig;

abigen!(
    AlignedLayerServiceManagerContract,
    "./src/eth/abi/AlignedLayerServiceManager.json",
);

abigen!(
    BatcherPaymentServiceContract,
    "./src/eth/abi/BatcherPaymentService.json",
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

pub type BatcherPaymentService =
    BatcherPaymentServiceContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

pub fn get_provider(eth_rpc_url: String) -> Result<Provider<Http>, anyhow::Error> {
    Provider::<Http>::try_from(eth_rpc_url).map_err(|err| anyhow::anyhow!(err))
}

pub async fn get_service_manager(
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
    payment_service: &BatcherPaymentService,
    batch_merkle_root: [u8; 32],
    batch_data_pointer: String,
    leaves: Vec<[u8; 32]>,
    signatures: Vec<Signature>,
    nonces: Vec<[u8; 32]>,
    gas_for_aggregator: U256,
    gas_per_proof: U256,
) -> Result<TransactionReceipt, anyhow::Error> {
    let signatures = signatures
        .iter()
        .enumerate()
        .map(|(i, signature)| signature_data_from_signature(signature, nonces[i]))
        .collect::<Vec<SignatureData>>();

    // pad leaves to next power of 2
    let leaves_len = leaves.len();
    let last_leaf = leaves[leaves_len - 1];
    let leaves = leaves
        .into_iter()
        .chain(repeat(last_leaf).take(leaves_len.next_power_of_two() - leaves_len))
        .collect::<Vec<[u8; 32]>>();

    let call = payment_service.create_new_task(
        batch_merkle_root,
        batch_data_pointer,
        leaves,
        signatures,
        gas_for_aggregator,
        gas_per_proof,
    );
    let pending_tx = call.send().await?;

    match pending_tx.await? {
        Some(receipt) => Ok(receipt),
        None => Err(anyhow::anyhow!("Receipt not found")),
    }
}

pub async fn get_batcher_payment_service(
    provider: Provider<Http>,
    ecdsa_config: ECDSAConfig,
    contract_address: String,
) -> Result<BatcherPaymentService, anyhow::Error> {
    let chain_id = provider.get_chainid().await?;

    // get private key from keystore
    let wallet = Wallet::decrypt_keystore(
        &ecdsa_config.private_key_store_path,
        &ecdsa_config.private_key_store_password,
    )?
    .with_chain_id(chain_id.as_u64());

    let signer = Arc::new(SignerMiddleware::new(provider, wallet));

    let service_manager =
        BatcherPaymentService::new(H160::from_str(contract_address.as_str())?, signer);

    Ok(service_manager)
}

fn signature_data_from_signature(signature: &Signature, nonce: [u8; 32]) -> SignatureData {
    let mut r = [0u8; 32];
    signature.r.to_big_endian(&mut r);

    let mut s = [0u8; 32];
    signature.s.to_big_endian(&mut s);

    let nonce = U256::from_big_endian(nonce.as_slice());

    SignatureData {
        v: signature.v as u8,
        r,
        s,
        nonce,
    }
}
