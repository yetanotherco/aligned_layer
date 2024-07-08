use crate::{
    communication::{
        messaging::{receive, receive_and_wait, send_messages},
        protocol::check_protocol_version,
    },
    core::{
        errors,
        types::{
            AlignedVerificationData, Chain, ClientMessage, VerificationData,
            VerificationDataCommitment,
        },
    },
    eth,
};

use ethers::{
    prelude::k256::ecdsa::SigningKey,
    providers::{Http, Provider},
    signers::Wallet,
};
use sha3::{Digest, Keccak256};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use log::debug;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

/// Submits multiple proofs to the batcher to be verified in Aligned and waits for the verification on-chain.
/// # Arguments
/// * `batcher_addr` - The address of the batcher to which the proof will be submitted.
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - An array of verification data of each proof.
/// * `wallet` - The wallet used to sign the proof.
/// # Returns
/// * An array of aligned verification data obtained when submitting the proof.
/// # Errors
/// * If there is an error connecting to the batcher.
/// * If there is an error serializing the message.
/// * If there is an error deserializing the message.
pub async fn submit_multiple_and_wait(
    batcher_addr: &str,
    eth_rpc_url: &str,
    chain: Chain,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError> {
    let (ws_stream, _) = connect_async(batcher_addr)
        .await
        .map_err(errors::SubmitError::ConnectionError)?;

    debug!("WebSocket handshake has been successfully completed");
    let (ws_write, ws_read) = ws_stream.split();

    let ws_write = Arc::new(Mutex::new(ws_write));

    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e: url::ParseError| errors::SubmitError::EthError(e.to_string()))?;

    let contract_address = match chain {
        Chain::Devnet => "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
        Chain::Holesky => "0x58F280BeBE9B34c9939C3C39e0890C81f163B623",
    };

    _submit_multiple_and_wait(
        ws_write,
        ws_read,
        eth_rpc_provider,
        contract_address,
        verification_data,
        wallet,
    )
    .await
}

async fn _submit_multiple_and_wait(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    mut ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    eth_rpc_provider: Provider<Http>,
    contract_address: &str,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError> {
    // First message from the batcher is the protocol version
    check_protocol_version(&mut ws_read).await?;

    if verification_data.is_empty() {
        return Err(errors::SubmitError::MissingParameter(
            "verification_data".to_string(),
        ));
    }
    let ws_write_clone = ws_write.clone();
    // The sent verification data will be stored here so that we can calculate
    // their commitments later.
    let mut sent_verification_data: Vec<VerificationData> = Vec::new();

    {
        let mut ws_write = ws_write.lock().await;

        for verification_data in verification_data.iter() {
            let msg = ClientMessage::new(verification_data.clone(), wallet.clone()).await;
            let msg_str = serde_json::to_string(&msg).map_err(errors::SubmitError::SerdeError)?;
            ws_write
                .send(Message::Text(msg_str.clone()))
                .await
                .map_err(errors::SubmitError::ConnectionError)?;
            sent_verification_data.push(verification_data.clone());
            debug!("Message sent...");
        }
    }

    let num_responses = Arc::new(Mutex::new(0));

    // This vector is reversed so that when responses are received, the commitments corresponding
    // to that response can simply be popped of this vector.
    let mut verification_data_commitments_rev: Vec<VerificationDataCommitment> =
        sent_verification_data
            .into_iter()
            .map(|vd| vd.into())
            .rev()
            .collect();

    let aligned_verification_data = receive_and_wait(
        ws_read,
        ws_write_clone,
        eth_rpc_provider,
        contract_address,
        verification_data.len(),
        num_responses,
        &mut verification_data_commitments_rev,
    )
    .await?;

    Ok(aligned_verification_data)
}

/// Submits multiple proofs to the batcher to be verified in Aligned.
/// # Arguments
/// * `batcher_addr` - The address of the batcher to which the proof will be submitted.
/// * `verification_data` - An array of verification data of each proof.
/// * `wallet` - The wallet used to sign the proof.
/// # Returns
/// * An array of aligned verification data obtained when submitting the proof.
/// # Errors
/// * If there is an error connecting to the batcher.
/// * If there is an error serializing the message.
/// * If there is an error deserializing the message.
pub async fn submit_multiple(
    batcher_addr: &str,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError> {
    let (ws_stream, _) = connect_async(batcher_addr)
        .await
        .map_err(errors::SubmitError::ConnectionError)?;

    debug!("WebSocket handshake has been successfully completed");
    let (ws_write, ws_read) = ws_stream.split();

    let ws_write = Arc::new(Mutex::new(ws_write));

    _submit_multiple(ws_write, ws_read, verification_data, wallet).await
}

async fn _submit_multiple(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    mut ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError> {
    // First message from the batcher is the protocol version
    check_protocol_version(&mut ws_read).await?;

    if verification_data.is_empty() {
        return Err(errors::SubmitError::MissingParameter(
            "verification_data".to_string(),
        ));
    }
    let ws_write_clone = ws_write.clone();
    // The sent verification data will be stored here so that we can calculate
    // their commitments later.
    let sent_verification_data = send_messages(ws_write, verification_data, wallet).await?;

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
        ws_read,
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
/// * `batcher_addr` - The address of the batcher to which the proof will be submitted.
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// * `chain` - The chain on which the verification will be done.
/// * `verification_data` - The verification data of the proof.
/// * `wallet` - The wallet used to sign the proof.
/// # Returns
/// * The aligned verification data obtained when submitting the proof.
/// # Errors
/// * If there is an error connecting to the batcher.
/// * If there is an error serializing the message.
/// * If there is an error deserializing the message.
pub async fn submit_and_wait(
    batcher_addr: &str,
    eth_rpc_url: &str,
    chain: Chain,
    verification_data: &VerificationData,
    wallet: Wallet<SigningKey>,
) -> Result<Option<AlignedVerificationData>, errors::SubmitError> {
    let verification_data = vec![verification_data.clone()];

    let aligned_verification_data =
        submit_multiple_and_wait(batcher_addr, eth_rpc_url, chain, &verification_data, wallet)
            .await?;

    if let Some(mut aligned_verification_data) = aligned_verification_data {
        Ok(aligned_verification_data.pop())
    } else {
        Ok(None)
    }
}

/// Submits a proof to the batcher to be verified in Aligned.
/// # Arguments
/// * `batcher_addr` - The address of the batcher to which the proof will be submitted.
/// * `verification_data` - The verification data of the proof.
/// * `wallet` - The wallet used to sign the proof.
/// # Returns
/// * The aligned verification data obtained when submitting the proof.
/// # Errors
/// * If there is an error connecting to the batcher.
/// * If there is an error serializing the message.
/// * If there is an error deserializing the message.
pub async fn submit(
    batcher_addr: &str,
    verification_data: &VerificationData,
    wallet: Wallet<SigningKey>,
) -> Result<Option<AlignedVerificationData>, errors::SubmitError> {
    let (ws_stream, _) = connect_async(batcher_addr)
        .await
        .map_err(errors::SubmitError::ConnectionError)?;

    debug!("WebSocket handshake has been successfully completed");
    let (ws_write, ws_read) = ws_stream.split();

    let ws_write = Arc::new(Mutex::new(ws_write));

    let verification_data = vec![verification_data.clone()];

    let aligned_verification_data =
        _submit_multiple(ws_write, ws_read, &verification_data, wallet).await?;

    if let Some(mut aligned_verification_data) = aligned_verification_data {
        Ok(aligned_verification_data.pop())
    } else {
        Ok(None)
    }
}

/// Checks if the proof has been verified with Aligned and is included in the batch.
/// # Arguments
/// * `aligned_verification_data` - The aligned verification data obtained when submitting the proofs.
/// * `chain` - The chain on which the verification will be done.
/// * `eth_rpc_url` - The URL of the Ethereum RPC node.
/// # Returns
/// * A boolean indicating whether the proof was verified on-chain and is included in the batch.
/// # Errors
/// * If there is an error creating the service manager.
/// * If there is an error calling the service manager.
/// * If there is an error verifying the proof on-chain.
pub async fn verify_proof_onchain(
    aligned_verification_data: AlignedVerificationData,
    chain: Chain,
    eth_rpc_url: &str,
) -> Result<bool, errors::VerificationError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e: url::ParseError| errors::VerificationError::EthError(e.to_string()))?;
    _verify_proof_onchain(aligned_verification_data, chain, eth_rpc_provider).await
}

async fn _verify_proof_onchain(
    aligned_verification_data: AlignedVerificationData,
    chain: Chain,
    eth_rpc_provider: Provider<Http>,
) -> Result<bool, errors::VerificationError> {
    let contract_address = match chain {
        Chain::Devnet => "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
        Chain::Holesky => "0x58F280BeBE9B34c9939C3C39e0890C81f163B623",
    };

    // All the elements from the merkle proof have to be concatenated
    let merkle_proof: Vec<u8> = aligned_verification_data
        .batch_inclusion_proof
        .merkle_path
        .into_iter()
        .flatten()
        .collect();

    let verification_data_comm = aligned_verification_data.verification_data_commitment;

    let service_manager = eth::aligned_service_manager(eth_rpc_provider, contract_address).await?;

    let call = service_manager.verify_batch_inclusion(
        verification_data_comm.proof_commitment,
        verification_data_comm.pub_input_commitment,
        verification_data_comm.proving_system_aux_data_commitment,
        verification_data_comm.proof_generator_addr,
        aligned_verification_data.batch_merkle_root,
        merkle_proof.into(),
        aligned_verification_data.index_in_batch.into(),
    );

    let result = call
        .await
        .map_err(|e| errors::VerificationError::EthError(e.to_string()))?;

    Ok(result)
}

/// Returns the commitment for a given verification key.
/// # Arguments
/// * `content` - The verification key for which the commitment will be calculated.
/// # Returns
/// * The verification key commitment.
/// # Errors
/// * None.
pub fn get_verification_key_commitment(content: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(content);
    hasher.finalize().into()
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

        let wallet = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            .parse::<LocalWallet>()
            .map_err(|e| SubmitError::GenericError(e.to_string()))
            .unwrap();

        let aligned_verification_data = submit_multiple_and_wait(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            wallet,
        )
        .await
        .unwrap()
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

        let result = submit_multiple_and_wait(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            wallet,
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
            pub_input: pub_input,
            verification_key: vk,
            vm_program_code: None,
            proof_generator_addr,
        };

        let verification_data = vec![verification_data];

        let wallet = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            .parse::<LocalWallet>()
            .map_err(|e| SubmitError::GenericError(e.to_string()))
            .unwrap();

        let aligned_verification_data = submit_multiple_and_wait(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            wallet,
        )
        .await
        .unwrap()
        .unwrap();

        sleep(std::time::Duration::from_secs(20)).await;

        let result = verify_proof_onchain(
            aligned_verification_data[0].clone(),
            Chain::Devnet,
            "http://localhost:8545",
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

        let aligned_verification_data = submit_multiple_and_wait(
            "ws://localhost:8080",
            "http://localhost:8545",
            Chain::Devnet,
            &verification_data,
            wallet,
        )
        .await
        .unwrap()
        .unwrap();

        sleep(std::time::Duration::from_secs(10)).await;

        let mut aligned_verification_data_modified = aligned_verification_data[0].clone();

        // Modify the index in batch to make the verification fail
        aligned_verification_data_modified.index_in_batch = 99;

        let result = verify_proof_onchain(
            aligned_verification_data_modified,
            Chain::Devnet,
            "http://localhost:8545",
        )
        .await
        .unwrap();

        assert!(!result, "Proof verified on chain");
    }

    fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
        std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
    }
}
