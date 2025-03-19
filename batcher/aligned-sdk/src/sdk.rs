use crate::{
    communication::{
        batch::await_batch_verification,
        messaging::{receive, send_messages, ResponseStream},
        protocol::check_protocol_version,
        serialization::{cbor_deserialize, cbor_serialize},
    },
    core::{
        constants::{
            ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF, DEFAULT_CONSTANT_GAS_COST,
            DEFAULT_MAX_FEE_BATCH_SIZE, GAS_PRICE_PERCENTAGE_MULTIPLIER,
            INSTANT_MAX_FEE_BATCH_SIZE, PERCENTAGE_DIVIDER,
        },
        errors::{self, GetNonceError},
        types::{
            AlignedVerificationData, ClientMessage, FeeEstimationType, GetNonceResponseMessage,
            Network, ProvingSystemId, VerificationData,
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
    types::{Address, U256},
};
use sha3::{Digest, Keccak256};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use log::{debug, info};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryFutureExt, TryStreamExt,
};

use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Submits multiple proofs to the batcher to be verified in Aligned and waits for the verification on-chain.
/// # Arguments
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - An array of verification data of each proof.
/// * `max_fees` - An array of the maximum fee that the submitter is willing to pay for each proof verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See [`get_nonce_from_ethereum`] or [`get_nonce_from_batcher`].
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
    eth_rpc_url: &str,
    network: Network,
    verification_data: &[VerificationData],
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Vec<Result<AlignedVerificationData, errors::SubmitError>> {
    let mut aligned_verification_data =
        submit_multiple(network.clone(), verification_data, max_fee, wallet, nonce).await;

    // TODO: open issue: use a join to .await all at the same time, avoiding the loop
    // And await only once per batch, no need to await multiple proofs if they are in the same batch.
    let mut error_awaiting_batch_verification: Option<errors::SubmitError> = None;
    for aligned_verification_data_item in aligned_verification_data.iter().flatten() {
        if let Err(e) =
            await_batch_verification(aligned_verification_data_item, eth_rpc_url, network.clone())
                .await
        {
            error_awaiting_batch_verification = Some(e);
            break;
        }
    }
    if let Some(error_awaiting_batch_verification) = error_awaiting_batch_verification {
        aligned_verification_data.push(Err(error_awaiting_batch_verification));
    }

    aligned_verification_data
}

/// Returns the estimated `max_fee` depending on the batch inclusion preference of the user, computed based on the current gas price, and the number of proofs in a batch.
/// NOTE: The `max_fee` is computed from an rpc nodes max priority gas price.
/// To estimate the `max_fee` of a batch we compute it based on a batch size of 1 (Instant), 10 (Default), or a user supplied `number_proofs_in_batch` (Custom).
/// The `max_fee` estimates therefore are:
/// * `Default`: Specifies a `max_fee` equivalent to the cost of paying for one proof within a batch of 10 proofs ie. 1 / 10 proofs.
///        This estimates a default `max_fee` the user should specify for including there proof within the batch.
/// * `Instant`: Specifies a `max_fee` equivalent to the cost of paying for an entire batch ensuring the user's proof is included instantly assuming the proof is not competing with others for inclusion.
/// * `Custom (number_proofs_in_batch)`: Specifies a `max_fee` equivalent to the cost of paying 1 proof / `number_proofs_in_batch` allowing the user a user to estimate the `max_fee` precisely based on the `number_proofs_in_batch`.
///
/// # Arguments
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `estimate_type` - Enum specifying the type of price estimate:  Default, Instant. Custom(usize)
/// # Returns
/// The estimated `max_fee` in gas for a proof based on the users `FeeEstimateType` as a `U256`.
/// # Errors
/// * `EthereumProviderError` if there is an error in the connection with the RPC provider.
/// * `EthereumGasPriceError` if there is an error retrieving the Ethereum gas price.
pub async fn estimate_fee(
    eth_rpc_url: &str,
    fee_estimation_type: FeeEstimationType,
) -> Result<U256, errors::FeeEstimateError> {
    match fee_estimation_type {
        FeeEstimationType::Default => {
            calculate_fee_per_proof_for_batch_of_size(eth_rpc_url, DEFAULT_MAX_FEE_BATCH_SIZE).await
        }
        FeeEstimationType::Instant => {
            calculate_fee_per_proof_for_batch_of_size(eth_rpc_url, INSTANT_MAX_FEE_BATCH_SIZE).await
        }
        FeeEstimationType::Custom(n) => {
            calculate_fee_per_proof_for_batch_of_size(eth_rpc_url, n).await
        }
    }
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
pub async fn calculate_fee_per_proof_for_batch_of_size(
    eth_rpc_url: &str,
    num_proofs_in_batch: usize,
) -> Result<U256, errors::FeeEstimateError> {
    let eth_rpc_provider =
        Provider::<Http>::try_from(eth_rpc_url).map_err(|e: url::ParseError| {
            errors::FeeEstimateError::EthereumProviderError(e.to_string())
        })?;
    let gas_price = fetch_gas_price(&eth_rpc_provider).await?;

    // Cost for estimate `num_proofs_per_batch` proofs
    let estimated_gas_per_proof = (DEFAULT_CONSTANT_GAS_COST
        + ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * num_proofs_in_batch as u128)
        / num_proofs_in_batch as u128;

    // Price of 1 proof in a batch of size `num_proofs_in_batch` i.e. (1 / `num_proofs_in_batch`).
    // The computed price is adjusted with respect to the percentage multiplier from:
    // https://github.com/yetanotherco/aligned_layer/blob/staging/batcher/aligned-batcher/src/lib.rs#L1401
    let fee_per_proof = (U256::from(estimated_gas_per_proof)
        * gas_price
        * U256::from(GAS_PRICE_PERCENTAGE_MULTIPLIER))
        / U256::from(PERCENTAGE_DIVIDER);

    Ok(fee_per_proof)
}

async fn fetch_gas_price(
    eth_rpc_provider: &Provider<Http>,
) -> Result<U256, errors::FeeEstimateError> {
    let gas_price = match eth_rpc_provider.get_gas_price().await {
        Ok(price) => price,
        Err(e) => {
            return Err(errors::FeeEstimateError::EthereumGasPriceError(
                e.to_string(),
            ))
        }
    };

    Ok(gas_price)
}

/// Submits multiple proofs to the batcher to be verified in Aligned.
/// # Arguments
/// * `network` - The netork on which the verification will be done.
/// * `verification_data` - An array of verification data of each proof.
/// * `max_fees` - An array of the maximum fee that the submitter is willing to pay for each proof verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See [`get_nonce_from_ethereum`] or [`get_nonce_from_batcher`].
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
    network: Network,
    verification_data: &[VerificationData],
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Vec<Result<AlignedVerificationData, errors::SubmitError>> {
    let (ws_stream, _) = match connect_async(network.get_batcher_url()).await {
        Ok((ws_stream, response)) => (ws_stream, response),
        Err(e) => return vec![Err(errors::SubmitError::WebSocketConnectionError(e))],
    };

    debug!("WebSocket handshake has been successfully completed");
    let (ws_write, ws_read) = ws_stream.split();

    let ws_write = Arc::new(Mutex::new(ws_write));

    _submit_multiple(
        ws_write,
        ws_read,
        network,
        verification_data,
        max_fee,
        wallet,
        nonce,
    )
    .await
}

// Will submit the proofs to the batcher and wait for their responses
// Will return once all proofs are responded, or up to a proof that is responded with an error
async fn _submit_multiple(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    mut ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    network: Network,
    verification_data: &[VerificationData],
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Vec<Result<AlignedVerificationData, errors::SubmitError>> {
    // First message from the batcher is the protocol version
    if let Err(e) = check_protocol_version(&mut ws_read).await {
        return vec![Err(e)];
    }

    if verification_data.is_empty() {
        return vec![Err(errors::SubmitError::MissingRequiredParameter(
            "verification_data".to_string(),
        ))];
    }
    if verification_data.len() > 10000 {
        //TODO Magic number
        return vec![Err(errors::SubmitError::GenericError(
            "Trying to submit too many proofs at once".to_string(),
        ))];
    }

    let ws_write_clone = ws_write.clone();

    let response_stream: ResponseStream =
        ws_read.try_filter(|msg| futures_util::future::ready(msg.is_binary() || msg.is_close()));

    let response_stream = Arc::new(Mutex::new(response_stream));

    let payment_service_addr = network.get_batcher_payment_service_address();

    let result = async {
        let sent_verification_data_rev = send_messages(
            ws_write,
            payment_service_addr,
            verification_data,
            max_fee,
            wallet,
            nonce,
        )
        .await;
        receive(response_stream, sent_verification_data_rev).await
    }
    .await;

    // Close connection
    info!("Closing WS connection");
    if let Err(e) = ws_write_clone.lock().await.close().await {
        return vec![Err(errors::SubmitError::GenericError(e.to_string()))];
    }
    result
}

/// Submits a proof to the batcher to be verified in Aligned and waits for the verification on-chain.
/// # Arguments
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - The verification data of the proof.
/// * `max_fee` - The maximum fee that the submitter is willing to pay for the verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See [`get_nonce_from_ethereum`] or [`get_nonce_from_batcher`].
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
    eth_rpc_url: &str,
    network: Network,
    verification_data: &VerificationData,
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<AlignedVerificationData, errors::SubmitError> {
    let verification_data = vec![verification_data.clone()];

    let aligned_verification_data = submit_multiple_and_wait_verification(
        eth_rpc_url,
        network,
        &verification_data,
        max_fee,
        wallet,
        nonce,
    )
    .await;

    match aligned_verification_data.first() {
        Some(Ok(aligned_verification_data)) => Ok(aligned_verification_data.clone()),
        Some(Err(e)) => Err(errors::SubmitError::GenericError(e.to_string())),
        None => Err(errors::SubmitError::GenericError(
            "No response from the batcher".to_string(),
        )),
    }
}

/// Submits a proof to the batcher to be verified in Aligned.
/// # Arguments
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - The verification data of the proof.
/// * `max_fee` - The maximum fee that the submitter is willing to pay for the verification.
/// * `wallet` - The wallet used to sign the proof.
/// * `nonce` - The nonce of the submitter address. See [`get_nonce_from_ethereum`] or [`get_nonce_from_batcher`].
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
    network: Network,
    verification_data: &VerificationData,
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<AlignedVerificationData, errors::SubmitError> {
    let verification_data = vec![verification_data.clone()];

    let aligned_verification_data =
        submit_multiple(network, &verification_data, max_fee, wallet, nonce).await;

    match aligned_verification_data.first() {
        Some(Ok(aligned_verification_data)) => Ok(aligned_verification_data.clone()),
        Some(Err(e)) => Err(errors::SubmitError::GenericError(e.to_string())),
        None => Err(errors::SubmitError::GenericError(
            "No response from the batcher".to_string(),
        )),
    }
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
    let contract_address = network.clone().get_aligned_service_manager_address();
    let payment_service_addr = network.get_batcher_payment_service_address();

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

/// Returns the next nonce for a given address from the batcher.
/// You should prefer this method instead of [`get_nonce_from_ethereum`] if you have recently sent proofs,
/// as the batcher proofs might not yet be on ethereum,
/// producing an out-of-sync nonce with the payment service contract on ethereum
/// # Arguments
/// * `address` - The user address for which the nonce will be retrieved.
/// * `network` - The network from which the nonce will be retrieved.
/// # Returns
/// * The next nonce of the proof submitter account.
/// # Errors
/// * `EthRpcError` if the batcher has an error in the Ethereum call when retrieving the nonce if not already cached.
pub async fn get_nonce_from_batcher(
    network: Network,
    address: Address,
) -> Result<U256, GetNonceError> {
    let (ws_stream, _) = connect_async(network.get_batcher_url())
        .await
        .map_err(|_| {
            GetNonceError::ConnectionFailed("Ws connection to batcher failed".to_string())
        })?;

    debug!("WebSocket handshake has been successfully completed");
    let (mut ws_write, mut ws_read) = ws_stream.split();
    check_protocol_version(&mut ws_read)
        .map_err(|e| match e {
            errors::SubmitError::ProtocolVersionMismatch { current, expected } => {
                GetNonceError::ProtocolMismatch { current, expected }
            }
            _ => GetNonceError::UnexpectedResponse(
                "Unexpected response, expected protocol version".to_string(),
            ),
        })
        .await?;

    let msg = ClientMessage::GetNonceForAddress(address);

    let msg_bin = cbor_serialize(&msg)
        .map_err(|_| GetNonceError::SerializationError("Failed to serialize msg".to_string()))?;
    ws_write
        .send(Message::Binary(msg_bin.clone()))
        .await
        .map_err(|_| {
            GetNonceError::ConnectionFailed(
                "Ws connection failed to send message to batcher".to_string(),
            )
        })?;

    let mut response_stream: ResponseStream =
        ws_read.try_filter(|msg| futures_util::future::ready(msg.is_binary()));

    let msg = match response_stream.next().await {
        Some(Ok(msg)) => msg,
        _ => {
            return Err(GetNonceError::ConnectionFailed(
                "Connection was closed without close message before receiving all messages"
                    .to_string(),
            ));
        }
    };

    let _ = ws_write.close().await;

    match cbor_deserialize(msg.into_data().as_slice()) {
        Ok(GetNonceResponseMessage::Nonce(nonce)) => Ok(nonce),
        Ok(GetNonceResponseMessage::EthRpcError(e)) => Err(GetNonceError::EthRpcError(e)),
        Ok(GetNonceResponseMessage::InvalidRequest(e)) => Err(GetNonceError::InvalidRequest(e)),
        Err(_) => Err(GetNonceError::SerializationError(
            "Failed to deserialize batcher message".to_string(),
        )),
    }
}

/// Returns the next nonce for a given address in Ethereum from aligned payment service contract.
/// Note that it might be out of sync if you recently sent proofs. For that see [`get_nonce_from_batcher`]
/// # Arguments
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `address` - The user address for which the nonce will be retrieved.
/// # Returns
/// * The next nonce of the proof submitter account from ethereum.
/// # Errors
/// * `EthRpcError` if the batcher has an error in the Ethereum call when retrieving the nonce if not already cached.
pub async fn get_nonce_from_ethereum(
    eth_rpc_url: &str,
    submitter_addr: Address,
    network: Network,
) -> Result<U256, GetNonceError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| GetNonceError::EthRpcError(e.to_string()))?;

    let payment_service_address = network.get_batcher_payment_service_address();

    match batcher_payment_service(eth_rpc_provider, payment_service_address).await {
        Ok(contract) => {
            let call = contract.user_nonces(submitter_addr);
            let result = call
                .call()
                .await
                .map_err(|e| GetNonceError::EthRpcError(e.to_string()))?;
            Ok(result)
        }
        Err(e) => Err(GetNonceError::EthRpcError(e.to_string())),
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
    let payment_service_address = network.get_batcher_payment_service_address();
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

    let payment_service_address = network.get_batcher_payment_service_address();

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
    info!(
        "Saving batch inclusion data files in folder {}",
        batch_inclusion_data_directory_path.display()
    );
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

    let mut file = File::create(batch_inclusion_data_path)?;
    file.write_all(data.as_slice())?;

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
    let mut file = File::create(batch_inclusion_data_path)?;
    file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod test {
    //Public constants for convenience
    pub const HOLESKY_PUBLIC_RPC_URL: &str = "https://ethereum-holesky-rpc.publicnode.com";
    use super::*;

    #[tokio::test]
    async fn computed_max_fee_for_larger_batch_is_smaller() {
        let small_fee = calculate_fee_per_proof_for_batch_of_size(HOLESKY_PUBLIC_RPC_URL, 5)
            .await
            .unwrap();
        let large_fee = calculate_fee_per_proof_for_batch_of_size(HOLESKY_PUBLIC_RPC_URL, 2)
            .await
            .unwrap();

        assert!(small_fee < large_fee);
    }

    #[tokio::test]
    async fn computed_max_fee_for_more_proofs_larger_than_for_less_proofs() {
        let small_fee = calculate_fee_per_proof_for_batch_of_size(HOLESKY_PUBLIC_RPC_URL, 20)
            .await
            .unwrap();
        let large_fee = calculate_fee_per_proof_for_batch_of_size(HOLESKY_PUBLIC_RPC_URL, 10)
            .await
            .unwrap();

        assert!(small_fee < large_fee);
    }

    #[tokio::test]
    async fn estimate_fee_are_larger_than_one_another() {
        let min_fee = estimate_fee(HOLESKY_PUBLIC_RPC_URL, FeeEstimationType::Custom(100))
            .await
            .unwrap();
        let default_fee = estimate_fee(HOLESKY_PUBLIC_RPC_URL, FeeEstimationType::Default)
            .await
            .unwrap();
        let instant_fee = estimate_fee(HOLESKY_PUBLIC_RPC_URL, FeeEstimationType::Instant)
            .await
            .unwrap();

        assert!(min_fee < default_fee);
        assert!(default_fee < instant_fee);
    }
}
