use crate::{
    communication::{
        batch::await_batch_verification,
        messaging::{receive, send_messages, ResponseStream},
        protocol::check_protocol_version,
        serialization::cbor_serialize,
    },
    core::{
        constants::{
            ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF, CONSTANT_GAS_COST,
            MAX_FEE_BATCH_PROOF_NUMBER, MAX_FEE_DEFAULT_PROOF_NUMBER,
        },
        errors,
        types::{
            AlignedVerificationData, Network, PriceEstimate, ProvingSystemId, VerificationData,
            VerificationDataCommitment,
        },
    },
    eth::{
        aligned_service_manager::aligned_service_manager,
        batcher_payment_service::batcher_payment_service,
    },
};

use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    prelude::k256::ecdsa::SigningKey,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Wallet},
    types::{Address, H160, U256},
};
use sha3::{Digest, Keccak256};
use std::{str::FromStr, sync::Arc};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use log::{debug, info};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde_json::json;
/// Submits multiple proofs to the batcher to be verified in Aligned and waits for the verification on-chain.
/// # Arguments
/// * `batcher_url` - The url of the batcher to which the proof will be submitted.
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - An array of verification data of each proof.
/// * `max_fees` - An array of the maximum fee that the submitter is willing to pay for each proof verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See `get_next_nonce`.
/// * `payment_service_addr` - The address of the payment service contract.
/// # Returns
/// * An array of aligned verification data obtained when submitting the proof.
/// # Errors
/// * `MissingRequiredParameter` if the verification data vector is empty.
/// * `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
/// * `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
/// * `SerializationError` if there is an error deserializing the message sent from the batcher.
/// * `WebSocketConnectionError` if there is an error connecting to the batcher.
/// * `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `HexDecodingError` if there is an error decoding the Aligned service manager contract address.
/// * `BatchVerificationTimeout` if there is a timeout waiting for the batch verification.
/// * `InvalidSignature` if the signature is invalid.
/// * `InvalidNonce` if the nonce is invalid.
/// * `InvalidMaxFee` if the max fee is invalid.
/// * `InvalidProof` if the proof is invalid.
/// * `ProofTooLarge` if the proof is too large.
/// * `InsufficientBalance` if the sender balance is insufficient or unlocked
/// * `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
/// * `GenericError` if the error doesn't match any of the previous ones.
#[allow(clippy::too_many_arguments)] // TODO: Refactor this function, use NoncedVerificationData
pub async fn submit_multiple_and_wait_verification(
    batcher_url: &str,
    eth_rpc_url: &str,
    network: Network,
    verification_data: &[VerificationData],
    max_fees: &[U256],
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Vec<AlignedVerificationData>, errors::SubmitError> {
    let aligned_verification_data = submit_multiple(
        batcher_url,
        network,
        verification_data,
        max_fees,
        wallet,
        nonce,
    )
    .await?;

    for aligned_verification_data_item in aligned_verification_data.iter() {
        await_batch_verification(aligned_verification_data_item, eth_rpc_url, network).await?;
    }

    Ok(aligned_verification_data)
}

/// Returns the estimated `max_fee` depending on the batch inclusion preference of the user, based on the max priority gas price.
/// NOTE: The `max_fee` is computed from an rpc nodes max priority gas price.
/// To estimate the `max_fee` of a batch we use a compute the `max_fee` with respect to a batch of ~32 proofs present.
/// The `max_fee` estimates therefore are:
/// * `Min`: Specifies a `max_fee` equivalent to the cost of 1 proof in a 32 proof batch.
///        This estimates the lowest possible `max_fee` the user should specify for there proof with lowest priority.
/// * `Default`: Specifies a `max_fee` equivalent to the cost of 10 proofs in a 32 proof batch.
///        This estimates the `max_fee` the user should specify for inclusion within the batch.
/// * `Instant`: specifies a `max_fee` equivalent to the cost of all proofs within in a 32 proof batch.
///        This estimates the `max_fee` the user should specify to pay for the entire batch of proofs and have there proof included instantly.
/// # Arguments
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `estimate` - Enum specifying the type of price estimate: MIN, DEFAULT, INSTANT.
/// # Returns
/// The estimated `max_fee` in gas for a proof based on the users `PriceEstimate` as a `U256`.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumGasPriceError` if there is an error retrieving the Ethereum gas price.
pub async fn estimate_fee(
    eth_rpc_url: &str,
    estimate: PriceEstimate,
) -> Result<U256, errors::MaxFeeEstimateError> {
    // Price of 1 proof in 32 proof batch
    let fee_per_proof = fee_per_proof(eth_rpc_url, MAX_FEE_BATCH_PROOF_NUMBER).await?;

    let proof_price = match estimate {
        PriceEstimate::Min => fee_per_proof,
        PriceEstimate::Default => U256::from(MAX_FEE_DEFAULT_PROOF_NUMBER) * fee_per_proof,
        PriceEstimate::Instant => U256::from(MAX_FEE_BATCH_PROOF_NUMBER) * fee_per_proof,
    };
    Ok(proof_price)
}

/// Returns the computed `max_fee` for a proof based on the number of proofs in a batch (`num_proofs_per_batch`) and
/// number of proofs (`num_proofs`) in that batch the user would pay for i.e (`num_proofs` / `num_proofs_per_batch`).
/// NOTE: The `max_fee` is computed from an rpc nodes max priority gas price.
/// # Arguments
/// * `eth_rpc_url` - The URL of the users Ethereum RPC node.
/// * `num_proofs` - number of proofs in a batch the user would pay for.
/// * `num_proofs_per_batch` - number of proofs within a batch.
/// # Returns
/// * The calculated `max_fee` as a `U256`.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumGasPriceError` if there is an error retrieving the Ethereum gas price.
pub async fn compute_max_fee(
    eth_rpc_url: &str,
    num_proofs: usize,
    num_proofs_per_batch: usize,
) -> Result<U256, errors::MaxFeeEstimateError> {
    let fee_per_proof = fee_per_proof(eth_rpc_url, num_proofs_per_batch).await?;
    Ok(fee_per_proof * num_proofs)
}

/// Returns the `fee_per_proof` based on the current gas price for a batch compromised of `num_proofs_per_batch`
/// i.e. (1 / `num_proofs_per_batch`).
// NOTE: The `fee_per_proof` is computed from an rpc nodes max priority gas price.
/// # Arguments
/// * `eth_rpc_url` - The URL of the users Ethereum RPC node.
/// * `num_proofs_per_batch` - number of proofs within a batch.
/// # Returns
/// * The fee per proof of a batch as a `U256`.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumGasPriceError` if there is an error retrieving the Ethereum gas price.
pub async fn fee_per_proof(
    eth_rpc_url: &str,
    num_proofs_per_batch: usize,
) -> Result<U256, errors::MaxFeeEstimateError> {
    let eth_rpc_provider =
        Provider::<Http>::try_from(eth_rpc_url).map_err(|e: url::ParseError| {
            errors::MaxFeeEstimateError::EthereumProviderError(e.to_string())
        })?;
    let gas_price = fetch_gas_price(&eth_rpc_provider).await?;

    // Cost for estimate `num_proofs_per_batch` proofs
    let estimated_gas_per_proof = (CONSTANT_GAS_COST
        + ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * num_proofs_per_batch as u128)
        / num_proofs_per_batch as u128;

    // Price of 1 proof in 32 proof batch
    let fee_per_proof = U256::from(estimated_gas_per_proof) * gas_price;

    Ok(fee_per_proof)
}

async fn fetch_gas_price(
    eth_rpc_provider: &Provider<Http>,
) -> Result<U256, errors::MaxFeeEstimateError> {
    let gas_price = match eth_rpc_provider.get_gas_price().await {
        Ok(price) => price,
        Err(e) => {
            return Err(errors::MaxFeeEstimateError::EthereumGasPriceError(
                e.to_string(),
            ))
        }
    };

    Ok(gas_price)
}

/// Submits multiple proofs to the batcher to be verified in Aligned.
/// # Arguments
/// * `batcher_url` - The url of the batcher to which the proof will be submitted.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - An array of verification data of each proof.
/// * `max_fees` - An array of the maximum fee that the submitter is willing to pay for each proof verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See `get_next_nonce`.
/// # Returns
/// * An array of aligned verification data obtained when submitting the proof.
/// # Errors
/// * `MissingRequiredParameter` if the verification data vector is empty.
/// * `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
/// * `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
/// * `SerializationError` if there is an error deserializing the message sent from the batcher.
/// * `WebSocketConnectionError` if there is an error connecting to the batcher.
/// * `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
/// * `InvalidSignature` if the signature is invalid.
/// * `InvalidNonce` if the nonce is invalid.
/// * `InvalidMaxFee` if the max fee is invalid.
/// * `InvalidProof` if the proof is invalid.
/// * `ProofTooLarge` if the proof is too large.
/// * `InsufficientBalance` if the sender balance is insufficient or unlocked.
/// * `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
/// * `GenericError` if the error doesn't match any of the previous ones.
pub async fn submit_multiple(
    batcher_url: &str,
    network: Network,
    verification_data: &[VerificationData],
    max_fees: &[U256],
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Vec<AlignedVerificationData>, errors::SubmitError> {
    let (ws_stream, _) = connect_async(batcher_url)
        .await
        .map_err(errors::SubmitError::WebSocketConnectionError)?;

    debug!("WebSocket handshake has been successfully completed");
    let (ws_write, ws_read) = ws_stream.split();

    let ws_write = Arc::new(Mutex::new(ws_write));

    let submit_result = _submit_multiple(
        ws_write.clone(),
        ws_read,
        network,
        verification_data,
        max_fees,
        wallet,
        nonce,
    )
    .await;

    if submit_result.is_err() {
        let mut ws_write = ws_write.lock().await;
        ws_write.close().await?;
    }
    submit_result
}

pub fn get_payment_service_address(network: Network) -> ethers::types::H160 {
    match network {
        Network::Devnet => H160::from_str("0x7969c5eD335650692Bc04293B07F5BF2e7A673C0").unwrap(),
        Network::Holesky => H160::from_str("0x815aeCA64a974297942D2Bbf034ABEe22a38A003").unwrap(),
        Network::HoleskyStage => {
            H160::from_str("0x7577Ec4ccC1E6C529162ec8019A49C13F6DAd98b").unwrap()
        }
    }
}

pub fn get_aligned_service_manager_address(network: Network) -> ethers::types::H160 {
    match network {
        Network::Devnet => H160::from_str("0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8").unwrap(),
        Network::Holesky => H160::from_str("0x58F280BeBE9B34c9939C3C39e0890C81f163B623").unwrap(),
        Network::HoleskyStage => {
            H160::from_str("0x9C5231FC88059C086Ea95712d105A2026048c39B").unwrap()
        }
    }
}

async fn _submit_multiple(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    mut ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    network: Network,
    verification_data: &[VerificationData],
    max_fees: &[U256],
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Vec<AlignedVerificationData>, errors::SubmitError> {
    // First message from the batcher is the protocol version
    check_protocol_version(&mut ws_read).await?;

    if verification_data.is_empty() {
        return Err(errors::SubmitError::MissingRequiredParameter(
            "verification_data".to_string(),
        ));
    }
    let ws_write_clone = ws_write.clone();

    let response_stream: ResponseStream =
        ws_read.try_filter(|msg| futures_util::future::ready(msg.is_binary() || msg.is_close()));

    let response_stream = Arc::new(Mutex::new(response_stream));

    let payment_service_addr = get_payment_service_address(network);

    let sent_verification_data = {
        // The sent verification data will be stored here so that we can calculate
        // their commitments later.
        send_messages(
            response_stream.clone(),
            ws_write,
            payment_service_addr,
            verification_data,
            max_fees,
            wallet,
            nonce,
        )
        .await?
    };

    let num_responses = Arc::new(Mutex::new(0));

    // This vector is reversed so that when responses are received, the commitments corresponding
    // to that response can simply be popped of this vector.
    let mut verification_data_commitments_rev: Vec<VerificationDataCommitment> =
        sent_verification_data
            .into_iter()
            .map(|vd| vd.into())
            .rev()
            .collect();

    let aligned_verification_data = receive(
        response_stream,
        ws_write_clone,
        verification_data.len(),
        num_responses,
        &mut verification_data_commitments_rev,
    )
    .await?;

    Ok(aligned_verification_data)
}

/// Submits a proof to the batcher to be verified in Aligned and waits for the verification on-chain.
/// # Arguments
/// * `batcher_url` - The url of the batcher to which the proof will be submitted.
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - The verification data of the proof.
/// * `max_fee` - The maximum fee that the submitter is willing to pay for the verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See `get_next_nonce`.
/// * `payment_service_addr` - The address of the payment service contract.
/// # Returns
/// * The aligned verification data obtained when submitting the proof.
/// # Errors
/// * `MissingRequiredParameter` if the verification data vector is empty.
/// * `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
/// * `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
/// * `SerializationError` if there is an error deserializing the message sent from the batcher.
/// * `WebSocketConnectionError` if there is an error connecting to the batcher.
/// * `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `HexDecodingError` if there is an error decoding the Aligned service manager contract address.
/// * `BatchVerificationTimeout` if there is a timeout waiting for the batch verification.
/// * `InvalidSignature` if the signature is invalid.
/// * `InvalidNonce` if the nonce is invalid.
/// * `InvalidMaxFee` if the max fee is invalid.
/// * `InvalidProof` if the proof is invalid.
/// * `ProofTooLarge` if the proof is too large.
/// * `InsufficientBalance` if the sender balance is insufficient or unlocked
/// * `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
/// * `GenericError` if the error doesn't match any of the previous ones.
#[allow(clippy::too_many_arguments)] // TODO: Refactor this function, use NoncedVerificationData
pub async fn submit_and_wait_verification(
    batcher_url: &str,
    eth_rpc_url: &str,
    network: Network,
    verification_data: &VerificationData,
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<AlignedVerificationData, errors::SubmitError> {
    let verification_data = vec![verification_data.clone()];

    let max_fees = vec![max_fee];

    let aligned_verification_data = submit_multiple_and_wait_verification(
        batcher_url,
        eth_rpc_url,
        network,
        &verification_data,
        &max_fees,
        wallet,
        nonce,
    )
    .await?;

    Ok(aligned_verification_data[0].clone())
}

/// Submits a proof to the batcher to be verified in Aligned.
/// # Arguments
/// * `batcher_url` - The url of the batcher to which the proof will be submitted.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - The verification data of the proof.
/// * `max_fee` - The maximum fee that the submitter is willing to pay for the verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See `get_next_nonce`.
/// # Returns
/// * The aligned verification data obtained when submitting the proof.
/// # Errors
/// * `MissingRequiredParameter` if the verification data vector is empty.
/// * `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
/// * `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
/// * `SerializationError` if there is an error deserializing the message sent from the batcher.
/// * `WebSocketConnectionError` if there is an error connecting to the batcher.
/// * `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
/// * `InvalidSignature` if the signature is invalid.
/// * `InvalidNonce` if the nonce is invalid.
/// * `InvalidMaxFee` if the max fee is invalid.
/// * `InvalidProof` if the proof is invalid.
/// * `ProofTooLarge` if the proof is too large.
/// * `InsufficientBalance` if the sender balance is insufficient or unlocked
/// * `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
/// * `GenericError` if the error doesn't match any of the previous ones.
pub async fn submit(
    batcher_url: &str,
    network: Network,
    verification_data: &VerificationData,
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<AlignedVerificationData, errors::SubmitError> {
    let verification_data = vec![verification_data.clone()];
    let max_fees = vec![max_fee];

    let aligned_verification_data = submit_multiple(
        batcher_url,
        network,
        &verification_data,
        &max_fees,
        wallet,
        nonce,
    )
    .await?;

    Ok(aligned_verification_data[0].clone())
}

/// Checks if the proof has been verified with Aligned and is included in the batch.
/// # Arguments
/// * `aligned_verification_data` - The aligned verification data obtained when submitting the proofs.
/// * `chain` - The chain on which the verification will be done.
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `payment_service_addr` - The address of the payment service.
/// # Returns
/// * A boolean indicating whether the proof was verified on-chain and is included in the batch.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumCallError` if there is an error in the Ethereum call.
/// * `HexDecodingError` if there is an error decoding the Aligned service manager contract address.
pub async fn is_proof_verified(
    aligned_verification_data: &AlignedVerificationData,
    network: Network,
    eth_rpc_url: &str,
) -> Result<bool, errors::VerificationError> {
    let eth_rpc_provider =
        Provider::<Http>::try_from(eth_rpc_url).map_err(|e: url::ParseError| {
            errors::VerificationError::EthereumProviderError(e.to_string())
        })?;

    _is_proof_verified(aligned_verification_data, network, eth_rpc_provider).await
}

async fn _is_proof_verified(
    aligned_verification_data: &AlignedVerificationData,
    network: Network,
    eth_rpc_provider: Provider<Http>,
) -> Result<bool, errors::VerificationError> {
    let contract_address = get_aligned_service_manager_address(network);
    let payment_service_addr = get_payment_service_address(network);

    // All the elements from the merkle proof have to be concatenated
    let merkle_proof: Vec<u8> = aligned_verification_data
        .batch_inclusion_proof
        .merkle_path
        .clone()
        .into_iter()
        .flatten()
        .collect();

    let verification_data_comm = aligned_verification_data
        .verification_data_commitment
        .clone();

    let service_manager = aligned_service_manager(eth_rpc_provider, contract_address).await?;

    let call = service_manager.verify_batch_inclusion(
        verification_data_comm.proof_commitment,
        verification_data_comm.pub_input_commitment,
        verification_data_comm.proving_system_aux_data_commitment,
        verification_data_comm.proof_generator_addr,
        aligned_verification_data.batch_merkle_root,
        merkle_proof.into(),
        aligned_verification_data.index_in_batch.into(),
        payment_service_addr,
    );

    let result = call
        .await
        .map_err(|e| errors::VerificationError::EthereumCallError(e.to_string()))?;

    Ok(result)
}

/// Returns the commitment for the verification key, taking into account the corresponding proving system.
/// # Arguments
/// * `verification_key_bytes` - The serialized contents of the verification key.
/// * `proving_system` - The corresponding proving system ID.
/// # Returns
/// * The commitment.
/// # Errors
/// * None.
pub fn get_vk_commitment(
    verification_key_bytes: &[u8],
    proving_system: ProvingSystemId,
) -> [u8; 32] {
    let proving_system_id_byte = proving_system as u8;
    let mut hasher = Keccak256::new();
    hasher.update(verification_key_bytes);
    hasher.update([proving_system_id_byte]);
    hasher.finalize().into()
}

/// Returns the next nonce for a given address.
/// # Arguments
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `submitter_addr` - The address of the proof submitter for which the nonce will be retrieved.
/// * `payment_service_addr` - The address of the batcher payment service contract.
/// # Returns
/// * The next nonce of the proof submitter account.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumCallError` if there is an error in the Ethereum call.
pub async fn get_next_nonce(
    eth_rpc_url: &str,
    submitter_addr: Address,
    network: Network,
) -> Result<U256, errors::NonceError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| errors::NonceError::EthereumProviderError(e.to_string()))?;

    let payment_service_address = get_payment_service_address(network);

    match batcher_payment_service(eth_rpc_provider, payment_service_address).await {
        Ok(contract) => {
            let call = contract.user_nonces(submitter_addr);

            let result = call
                .call()
                .await
                .map_err(|e| errors::NonceError::EthereumCallError(e.to_string()))?;

            Ok(result)
        }
        Err(e) => Err(errors::NonceError::EthereumCallError(e.to_string())),
    }
}

/// Returns the chain ID of the Ethereum network.
/// # Arguments
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// # Returns
/// * The chain ID of the Ethereum network.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumCallError` if there is an error in the Ethereum call.
pub async fn get_chain_id(eth_rpc_url: &str) -> Result<u64, errors::ChainIdError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| errors::ChainIdError::EthereumProviderError(e.to_string()))?;

    let chain_id = eth_rpc_provider
        .get_chainid()
        .await
        .map_err(|e| errors::ChainIdError::EthereumCallError(e.to_string()))?;

    Ok(chain_id.as_u64())
}

/// Funds the batcher payment service in name of the signer
/// # Arguments
/// * `amount` - The amount to be paid.
/// * `signer` - The signer middleware of the payer.
/// * `network` - The network on which the payment will be done.
/// # Returns
/// * The receipt of the payment transaction.
/// # Errors
/// * `SendError` if there is an error sending the transaction.
/// * `SubmitError` if there is an error submitting the transaction.
/// * `PaymentFailed` if the payment failed.
pub async fn deposit_to_aligned(
    amount: U256,
    signer: SignerMiddleware<Provider<Http>, LocalWallet>,
    network: Network,
) -> Result<ethers::types::TransactionReceipt, errors::PaymentError> {
    let payment_service_address = get_payment_service_address(network);
    let from = signer.address();

    let tx = TransactionRequest::new()
        .from(from)
        .to(payment_service_address)
        .value(amount);

    match signer
        .send_transaction(tx, None)
        .await
        .map_err(|e| errors::PaymentError::SendError(e.to_string()))?
        .await
        .map_err(|e| errors::PaymentError::SubmitError(e.to_string()))?
    {
        Some(receipt) => Ok(receipt),
        None => Err(errors::PaymentError::PaymentFailed),
    }
}

/// Returns the balance of a user in the payment service.
/// # Arguments
/// * `user` - The address of the user.
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `network` - The network on which the balance will be checked.
/// # Returns
/// * The balance of the user in the payment service.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumCallError` if there is an error in the Ethereum call.
pub async fn get_balance_in_aligned(
    user: Address,
    eth_rpc_url: &str,
    network: Network,
) -> Result<U256, errors::BalanceError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| errors::BalanceError::EthereumProviderError(e.to_string()))?;

    let payment_service_address = get_payment_service_address(network);

    match batcher_payment_service(eth_rpc_provider, payment_service_address).await {
        Ok(batcher_payment_service) => {
            let call = batcher_payment_service.user_balances(user);

            let result = call
                .call()
                .await
                .map_err(|e| errors::BalanceError::EthereumCallError(e.to_string()))?;

            Ok(result)
        }
        Err(e) => Err(errors::BalanceError::EthereumCallError(e.to_string())),
    }
}

/// Saves AlignedVerificationData in a file.
/// # Arguments
/// * `batch_inclusion_data_directory_path` - The path of the directory where the data will be saved.
/// * `aligned_verification_data` - The aligned verification data to be saved.
/// # Returns
/// * Ok if the data is saved successfully.
/// # Errors
/// * `FileError` if there is an error writing the data to the file.
pub fn save_response(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
) -> Result<(), errors::FileError> {
    save_response_cbor(
        batch_inclusion_data_directory_path.clone(),
        &aligned_verification_data.clone(),
    )?;
    save_response_json(
        batch_inclusion_data_directory_path,
        aligned_verification_data,
    )
}
fn save_response_cbor(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
) -> Result<(), errors::FileError> {
    let batch_merkle_root = &hex::encode(aligned_verification_data.batch_merkle_root)[..8];
    let batch_inclusion_data_file_name = batch_merkle_root.to_owned()
        + "_"
        + &aligned_verification_data.index_in_batch.to_string()
        + ".cbor";

    let batch_inclusion_data_path =
        batch_inclusion_data_directory_path.join(batch_inclusion_data_file_name);

    let data = cbor_serialize(&aligned_verification_data)?;

    let mut file = File::create(&batch_inclusion_data_path)?;
    file.write_all(data.as_slice())?;
    info!(
        "Batch inclusion data written into {}",
        batch_inclusion_data_path.display()
    );

    Ok(())
}
fn save_response_json(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
) -> Result<(), errors::FileError> {
    let batch_merkle_root = &hex::encode(aligned_verification_data.batch_merkle_root)[..8];
    let batch_inclusion_data_file_name = batch_merkle_root.to_owned()
        + "_"
        + &aligned_verification_data.index_in_batch.to_string()
        + ".json";

    let batch_inclusion_data_path =
        batch_inclusion_data_directory_path.join(batch_inclusion_data_file_name);

    let merkle_proof = aligned_verification_data
        .batch_inclusion_proof
        .merkle_path
        .iter()
        .map(hex::encode)
        .collect::<Vec<String>>()
        .join("");
    let data = json!({
            "proof_commitment": hex::encode(aligned_verification_data.verification_data_commitment.proof_commitment),
            "pub_input_commitment": hex::encode(aligned_verification_data.verification_data_commitment.pub_input_commitment),
            "program_id_commitment": hex::encode(aligned_verification_data.verification_data_commitment.proving_system_aux_data_commitment),
            "proof_generator_addr": hex::encode(aligned_verification_data.verification_data_commitment.proof_generator_addr),
            "batch_merkle_root": hex::encode(aligned_verification_data.batch_merkle_root),
            "verification_data_batch_index": aligned_verification_data.index_in_batch,
            "merkle_proof": merkle_proof,
    });
    let mut file = File::create(&batch_inclusion_data_path)?;
    file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())?;

    info!(
        "Batch inclusion data written into {}",
        batch_inclusion_data_path.display()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    //Public constants for convenience
    pub const HOLESKY_PUBLIC_RPC_URL: &str = "https://ethereum-holesky-rpc.publicnode.com";
    use super::*;

    #[tokio::test]
    async fn computed_max_fee_for_larger_batch_is_smaller() {
        let small_fee = compute_max_fee(HOLESKY_PUBLIC_RPC_URL, 2, 10)
            .await
            .unwrap();
        let large_fee = compute_max_fee(HOLESKY_PUBLIC_RPC_URL, 5, 10)
            .await
            .unwrap();

        assert!(small_fee < large_fee);
    }

    #[tokio::test]
    async fn computed_max_fee_for_more_proofs_larger_than_for_less_proofs() {
        let small_fee = compute_max_fee(HOLESKY_PUBLIC_RPC_URL, 5, 20)
            .await
            .unwrap();
        let large_fee = compute_max_fee(HOLESKY_PUBLIC_RPC_URL, 5, 10)
            .await
            .unwrap();

        assert!(small_fee < large_fee);
    }

    #[tokio::test]
    async fn estimate_fee_are_larger_than_one_another() {
        let min_fee = estimate_fee(HOLESKY_PUBLIC_RPC_URL, PriceEstimate::Min)
            .await
            .unwrap();
        let default_fee = estimate_fee(HOLESKY_PUBLIC_RPC_URL, PriceEstimate::Default)
            .await
            .unwrap();
        let instant_fee = estimate_fee(HOLESKY_PUBLIC_RPC_URL, PriceEstimate::Instant)
            .await
            .unwrap();

        assert!(min_fee < default_fee);
        assert!(default_fee < instant_fee);
    }
}
