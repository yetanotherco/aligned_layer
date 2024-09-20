use crate::{
    communication::{
        batch::await_batch_verification,
        messaging::{receive, send_messages, ResponseStream},
        protocol::check_protocol_version,
    },
    core::{
        errors,
        types::{
            AlignedVerificationData, Chain, ProvingSystemId, VerificationData,
            VerificationDataCommitment,
        },
    },
    eth::{
        aligned_service_manager::aligned_service_manager,
        batcher_payment_service::batcher_payment_service,
    },
};

use ethers::{
    prelude::k256::ecdsa::SigningKey,
    providers::{Http, Middleware, Provider},
    signers::Wallet,
    types::{Address, H160, U256},
};
use sha3::{Digest, Keccak256};
use std::{str::FromStr, sync::Arc};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use log::{debug, info};

use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt, TryStreamExt,
};

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
    chain: Chain,
    verification_data: &[VerificationData],
    max_fees: &[U256],
    wallet: Wallet<SigningKey>,
    nonce: U256,
    payment_service_addr: &str,
) -> Result<Vec<AlignedVerificationData>, errors::SubmitError> {
    let aligned_verification_data = submit_multiple(
        batcher_url,
        chain.clone(),
        verification_data,
        max_fees,
        wallet,
        nonce,
    )
    .await?;

    for aligned_verification_data_item in aligned_verification_data.iter() {
        await_batch_verification(
            aligned_verification_data_item,
            eth_rpc_url,
            chain.clone(),
            payment_service_addr,
        )
        .await?;
    }

    Ok(aligned_verification_data)
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
    chain: Chain,
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

    _submit_multiple(
        ws_write,
        ws_read,
        chain.clone(),
        verification_data,
        max_fees,
        wallet,
        nonce,
    )
    .await
}

async fn _submit_multiple(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    mut ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    chain: Chain,
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

    let payment_service_addr = match chain {
        Chain::Devnet => H160::from_str("0x7969c5eD335650692Bc04293B07F5BF2e7A673C0").ok(),
        Chain::Holesky => H160::from_str("0x815aeCA64a974297942D2Bbf034ABEe22a38A003").ok(),
        Chain::HoleskyStage => H160::from_str("0x7577Ec4ccC1E6C529162ec8019A49C13F6DAd98b").ok(),
    };

    let sent_verification_data = match payment_service_addr {
        // The sent verification data will be stored here so that we can calculate
        // their commitments later.
        Some(payment_service_addr) => {
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
        }
        None => {
            return Err(errors::SubmitError::GenericError(
                "Invalid chain".to_string(),
            ))
        }
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
    chain: Chain,
    verification_data: &VerificationData,
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
    payment_service_addr: &str,
) -> Result<AlignedVerificationData, errors::SubmitError> {
    let verification_data = vec![verification_data.clone()];

    let max_fees = vec![max_fee];

    let aligned_verification_data = submit_multiple_and_wait_verification(
        batcher_url,
        eth_rpc_url,
        chain,
        &verification_data,
        &max_fees,
        wallet,
        nonce,
        payment_service_addr,
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
    chain: Chain,
    verification_data: &VerificationData,
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<AlignedVerificationData, errors::SubmitError> {
    let verification_data = vec![verification_data.clone()];
    let max_fees = vec![max_fee];

    let aligned_verification_data = submit_multiple(
        batcher_url,
        chain.clone(),
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
    chain: Chain,
    eth_rpc_url: &str,
    payment_service_addr: &str,
) -> Result<bool, errors::VerificationError> {
    let eth_rpc_provider =
        Provider::<Http>::try_from(eth_rpc_url).map_err(|e: url::ParseError| {
            errors::VerificationError::EthereumProviderError(e.to_string())
        })?;

    _is_proof_verified(
        aligned_verification_data,
        chain,
        eth_rpc_provider,
        payment_service_addr,
    )
    .await
}

async fn _is_proof_verified(
    aligned_verification_data: &AlignedVerificationData,
    chain: Chain,
    eth_rpc_provider: Provider<Http>,
    payment_service_addr: &str,
) -> Result<bool, errors::VerificationError> {
    let contract_address = match chain {
        Chain::Devnet => "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
        // If we re-deploy the Aligned SM contract we need to change this value to the new contract address
        Chain::Holesky => "0x0584313310bD52B77CF0b81b350Ca447B97Df5DF",
        Chain::HoleskyStage => "0x9C5231FC88059C086Ea95712d105A2026048c39B",
    };

    let payment_service_addr = payment_service_addr
        .parse::<Address>()
        .map_err(|e| errors::VerificationError::HexDecodingError(e.to_string()))?;

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

    let result = call.await.map_err(|e| {
        info!("err: {}", e.to_string());
        errors::VerificationError::EthereumCallError(e.to_string())
    })?;

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
    let proving_system_id_byte = proving_system.clone() as u8;
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
    payment_service_addr: &str,
) -> Result<U256, errors::NonceError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| errors::NonceError::EthereumProviderError(e.to_string()))?;

    match batcher_payment_service(eth_rpc_provider, payment_service_addr).await {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::{errors::SubmitError, types::ProvingSystemId};
    use ethers::types::Address;
    use ethers::types::H160;

    use std::path::PathBuf;
    use std::str::FromStr;
    use tokio::time::sleep;

    use ethers::signers::LocalWallet;

    const BATCHER_PAYMENT_SERVICE_ADDR: &str = "0x7969c5eD335650692Bc04293B07F5BF2e7A673C0";
    const MAX_FEE: U256 = U256::max_value();

    #[tokio::test]
    async fn test_submit_success() {
        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let proof = read_file(base_dir.join("test_files/sp1/sp1_fibonacci.proof")).unwrap();
        let elf = Some(read_file(base_dir.join("test_files/sp1/sp1_fibonacci.elf")).unwrap());

        let proof_generator_addr =
            Address::from_str("0x66f9664f97F2b50F62D13eA064982f936dE76657").unwrap();

        let verification_data = VerificationData {
            proving_system: ProvingSystemId::SP1,
            proof,
            pub_input: None,
            verification_key: None,
            vm_program_code: elf,
            proof_generator_addr,
        };

        let verification_data = vec![verification_data];

        let max_fees = vec![MAX_FEE];

        let wallet = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            .parse::<LocalWallet>()
            .map_err(|e| SubmitError::GenericError(e.to_string()))
            .unwrap();

        let aligned_verification_data = submit_multiple_and_wait_verification(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            &max_fees,
            wallet,
            U256::zero(),
            BATCHER_PAYMENT_SERVICE_ADDR,
        )
        .await
        .unwrap();

        assert_eq!(aligned_verification_data.len(), 1);
    }

    #[tokio::test]
    async fn test_submit_failure() {
        //Create an erroneous verification data vector
        let contract_addr = H160::from_str("0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8").unwrap();

        let verification_data = vec![VerificationData {
            proving_system: ProvingSystemId::SP1,
            proof: vec![],
            pub_input: None,
            verification_key: None,
            vm_program_code: None,
            proof_generator_addr: contract_addr,
        }];

        let wallet = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            .parse::<LocalWallet>()
            .map_err(|e| SubmitError::GenericError(e.to_string()))
            .unwrap();

        let max_fees = vec![MAX_FEE];

        let result = submit_multiple_and_wait_verification(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            &max_fees,
            wallet,
            U256::zero(),
            BATCHER_PAYMENT_SERVICE_ADDR,
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_proof_onchain_success() {
        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let proof = read_file(base_dir.join("test_files/groth16_bn254/plonk.proof")).unwrap();
        let pub_input =
            read_file(base_dir.join("test_files/groth16_bn254/plonk_pub_input.pub")).ok();
        let vk = read_file(base_dir.join("test_files/groth16_bn254/plonk.vk")).ok();

        let proof_generator_addr =
            Address::from_str("0x66f9664f97F2b50F62D13eA064982f936dE76657").unwrap();

        let verification_data = VerificationData {
            proving_system: ProvingSystemId::Groth16Bn254,
            proof,
            pub_input,
            verification_key: vk,
            vm_program_code: None,
            proof_generator_addr,
        };

        let verification_data = vec![verification_data];

        let wallet = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            .parse::<LocalWallet>()
            .map_err(|e| SubmitError::GenericError(e.to_string()))
            .unwrap();

        let max_fees = vec![MAX_FEE];

        let aligned_verification_data = submit_multiple_and_wait_verification(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            &max_fees,
            wallet,
            U256::zero(),
            BATCHER_PAYMENT_SERVICE_ADDR,
        )
        .await
        .unwrap();

        sleep(std::time::Duration::from_secs(20)).await;

        let result = is_proof_verified(
            &aligned_verification_data[0],
            Chain::Devnet,
            "http://localhost:8545",
            BATCHER_PAYMENT_SERVICE_ADDR,
        )
        .await
        .unwrap();

        assert!(result, "Proof was not verified on-chain");
    }

    #[tokio::test]
    async fn test_verify_proof_onchain_failure() {
        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let proof = read_file(base_dir.join("test_files/sp1/sp1_fibonacci.proof")).unwrap();
        let elf = Some(read_file(base_dir.join("test_files/sp1/sp1_fibonacci.elf")).unwrap());

        let proof_generator_addr =
            Address::from_str("0x66f9664f97F2b50F62D13eA064982f936dE76657").unwrap();

        let verification_data = VerificationData {
            proving_system: ProvingSystemId::SP1,
            proof,
            pub_input: None,
            verification_key: None,
            vm_program_code: elf,
            proof_generator_addr,
        };

        let verification_data = vec![verification_data];

        let wallet = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            .parse::<LocalWallet>()
            .map_err(|e| SubmitError::GenericError(e.to_string()))
            .unwrap();

        let aligned_verification_data = submit_multiple_and_wait_verification(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            &[MAX_FEE],
            wallet,
            U256::zero(),
            BATCHER_PAYMENT_SERVICE_ADDR,
        )
        .await
        .unwrap();

        sleep(std::time::Duration::from_secs(20)).await;

        let mut aligned_verification_data_modified = aligned_verification_data[0].clone();

        // Modify the batch merkle root so that the verification fails
        aligned_verification_data_modified.batch_merkle_root[0] = 0;

        let result = is_proof_verified(
            &aligned_verification_data_modified,
            Chain::Devnet,
            "http://localhost:8545",
            BATCHER_PAYMENT_SERVICE_ADDR,
        )
        .await
        .unwrap();

        assert!(!result, "Proof verified on chain");
    }

    fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
        std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
    }
}
