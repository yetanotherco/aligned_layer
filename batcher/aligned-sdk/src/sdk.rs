use crate::errors;
use crate::eth;
use crate::models::{
    AlignedVerificationData, BatchInclusionData, Chain, VerificationCommitmentBatch,
    VerificationData, VerificationDataCommitment,
};

use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use log::{debug, error};

use ethers::providers::{Http, Provider};
use ethers::utils::hex;
use futures_util::{
    future,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};

/// Submits the proofs to the batcher to be verified and returns a vector of Aligned verification data.
/// # Arguments
/// * `ws_write` - A mutex-protected split sink to write messages to the websocket.
/// * `ws_read` - A split stream to read messages from the websocket.
/// * `verification_data` - A vector of verification data to be submitted to the batcher.
/// # Returns
/// * A vector of Aligned verification data.
/// # Errors
/// * If the verification data vector is empty.
/// * If there is an error serializing the verification data.
/// * If there is an error sending the message to the websocket.
/// * If there is an error receiving the response from the websocket.
/// * If there is an error closing the websocket.
/// * If there is an error deserializing the response from the websocket.
/// * If the connection was closed before receiving all messages.
pub async fn submit(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    verification_data: Vec<VerificationData>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError> {
    if verification_data.is_empty() {
        return Err(errors::SubmitError::MissingParameter(
            "verification_data".to_string(),
        ));
    }
    let ws_write_clone = ws_write.clone();
    let mut ws_write = ws_write.lock().await;

    // The sent verification data will be stored here so that we can calculate
    // their commitments later.
    let mut sent_verification_data: Vec<VerificationData> = Vec::new();

    for verification_data in verification_data.iter() {
        let json_data =
            serde_json::to_string(&verification_data).map_err(errors::SubmitError::SerdeError)?;
        ws_write
            .send(Message::Text(json_data.to_string()))
            .await
            .map_err(errors::SubmitError::ConnectionError)?;
        sent_verification_data.push(verification_data.clone());
        debug!("Message sent...");
    }

    drop(ws_write);

    // This vector is reversed so that when responses are received, the commitments corresponding
    // to that response can simply be popped of this vector.
    let mut verification_data_commitments_rev: Vec<VerificationDataCommitment> =
        sent_verification_data
            .into_iter()
            .map(|vd| vd.into())
            .rev()
            .collect();

    let num_responses = Arc::new(Mutex::new(0));

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

async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    total_messages: usize,
    num_responses: Arc<Mutex<usize>>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    let mut aligned_verification_data: Vec<AlignedVerificationData> = Vec::new();

    while let Some(Ok(msg)) = response_stream.next().await {
        if let Message::Close(close_frame) = msg {
            if let Some(close_msg) = close_frame {
                error!("Connection was closed before receiving all messages. Reason: {}. Try submitting your proof again", close_msg.to_owned());
                ws_write.lock().await.close().await?;
                return Ok(None);
            }
            error!("Connection was closed before receiving all messages. Try submitting your proof again");
            ws_write.lock().await.close().await?;
            return Ok(None);
        } else {
            let mut num_responses_lock = num_responses.lock().await;
            *num_responses_lock += 1;

            let data = msg.into_data();
            match serde_json::from_slice::<BatchInclusionData>(&data) {
                Ok(batch_inclusion_data) => {
                    debug!("Received response from batcher");
                    debug!(
                        "Batch merkle root: {}",
                        hex::encode(batch_inclusion_data.batch_merkle_root)
                    );
                    debug!("Index in batch: {}", batch_inclusion_data.index_in_batch);

                    let verification_data_commitment =
                        verification_data_commitments_rev.pop().unwrap_or_default();

                    if verify_response(&verification_data_commitment, &batch_inclusion_data) {
                        aligned_verification_data.push(AlignedVerificationData::new(
                            &verification_data_commitment,
                            &batch_inclusion_data,
                        ));
                    }
                }
                Err(e) => {
                    error!("Error while deserializing batcher response: {}", e);
                }
            }
            if *num_responses_lock == total_messages {
                debug!("All messages responded. Closing connection...");
                ws_write.lock().await.close().await?;
                return Ok(Some(aligned_verification_data));
            }
        }
    }

    Ok(None)
}

fn verify_response(
    verification_data_commitment: &VerificationDataCommitment,
    batch_inclusion_data: &BatchInclusionData,
) -> bool {
    debug!("Verifying response data matches sent proof data ...");
    let batch_inclusion_proof = batch_inclusion_data.batch_inclusion_proof.clone();

    if batch_inclusion_proof.verify::<VerificationCommitmentBatch>(
        &batch_inclusion_data.batch_merkle_root,
        batch_inclusion_data.index_in_batch,
        verification_data_commitment,
    ) {
        debug!("Done. Data sent matches batcher answer");
        return true;
    }

    error!("Verification data commitments and batcher response with merkle root {} and index in batch {} don't match", hex::encode(batch_inclusion_data.batch_merkle_root), batch_inclusion_data.index_in_batch);
    false
}

/// Checks if the proof has been verified with Aligned and is included in the batch.
/// # Arguments
/// * `aligned_verification_data` - The aligned verification data obtained when submitting the proofs.
/// * `chain` - The chain on which the verification will be done.
/// * `eth_rpc_provider` - The Ethereum RPC provider.
/// # Returns
/// * A boolean indicating whether the proof was verified on-chain and is included in the batch.
/// # Errors
/// * If there is an error creating the service manager.
/// * If there is an error calling the service manager.
/// * If there is an error verifying the proof on-chain.
pub async fn verify_proof_onchain(
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::errors::SubmitError;
    use crate::models::ProvingSystemId;
    use ethers::types::Address;
    use ethers::types::H160;

    use std::path::PathBuf;
    use std::str::FromStr;

    use tokio_tungstenite::connect_async;

    #[tokio::test]
    async fn test_submit_success() {
        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let proof = read_file(base_dir.join("test_files/sp1/sp1_fibonacci.proof")).unwrap();
        let elf = Some(read_file(base_dir.join("test_files/sp1/sp1_fibonacci-elf")).unwrap());

        let (ws_stream, _) = connect_async("ws://localhost:8080")
            .await
            .map_err(|e| SubmitError::ConnectionError(e))
            .unwrap();

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

        let (ws_write, ws_read) = ws_stream.split();

        let ws_write_mutex = Arc::new(Mutex::new(ws_write));

        let aligned_verification_data = submit(ws_write_mutex.clone(), ws_read, verification_data)
            .await
            .unwrap()
            .unwrap();

        ws_write_mutex.lock().await.close().await.unwrap();

        assert_eq!(aligned_verification_data.len(), 1);
    }

    #[tokio::test]
    async fn test_submit_failure() {
        let (ws_stream, _) = connect_async("ws://localhost:8080")
            .await
            .map_err(|e| SubmitError::ConnectionError(e))
            .unwrap();

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

        let (ws_write, ws_read) = ws_stream.split();

        let ws_write_mutex = Arc::new(Mutex::new(ws_write));

        let result = submit(ws_write_mutex.clone(), ws_read, verification_data).await;

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

        let (ws_stream, _) = connect_async("ws://localhost:8080")
            .await
            .map_err(|e| SubmitError::ConnectionError(e))
            .unwrap();

        let (ws_write, ws_read) = ws_stream.split();

        let ws_write_mutex = Arc::new(Mutex::new(ws_write));

        let aligned_verification_data = submit(ws_write_mutex.clone(), ws_read, verification_data)
            .await
            .unwrap()
            .unwrap();

        let eth_rpc_provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();

        let result = verify_proof_onchain(
            aligned_verification_data[0].clone(),
            Chain::Devnet,
            eth_rpc_provider.clone(),
        )
        .await
        .unwrap();

        assert!(result);
    }

    #[tokio::test]
    async fn test_verify_proof_onchain_failure() {
        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let proof = read_file(base_dir.join("test_files/sp1/sp1_fibonacci.proof")).unwrap();
        let elf = Some(read_file(base_dir.join("test_files/sp1/sp1_fibonacci-elf")).unwrap());

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

        let (ws_stream, _) = connect_async("ws://localhost:8080")
            .await
            .map_err(|e| SubmitError::ConnectionError(e))
            .unwrap();

        let (ws_write, ws_read) = ws_stream.split();

        let ws_write_mutex = Arc::new(Mutex::new(ws_write));

        let aligned_verification_data = submit(ws_write_mutex.clone(), ws_read, verification_data)
            .await
            .unwrap()
            .unwrap();

        let eth_rpc_provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();

        let mut aligned_verification_data_modified = aligned_verification_data[0].clone();

        // Modify the index in batch to make the verification fail
        aligned_verification_data_modified.index_in_batch = 99;

        let result = verify_proof_onchain(
            aligned_verification_data_modified,
            Chain::Devnet,
            eth_rpc_provider.clone(),
        )
        .await
        .unwrap();

        assert!(!result);
    }

    fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
        std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
    }
}
