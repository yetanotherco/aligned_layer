use crate::errors;
use crate::models::{
    AlignedVerificationData, BatchInclusionData, SubmitArgs, VerificationCommitmentBatch,
    VerificationData, VerificationDataCommitment,
};

use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use log::{error, info};

use ethers::utils::hex;
use futures_util::{
    future,
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt, TryStreamExt,
};

pub async fn submit(submit_args: SubmitArgs) -> Result<(), errors::SubmitError> {
    let (mut ws_write, ws_read) = submit_args.ws_stream.split();

    // The sent verification data will be stored here so that we can calculate
    // their commitments later.
    let mut sent_verification_data: Vec<VerificationData> = Vec::new();

    let json_data = serde_json::to_string(&submit_args.verification_data)?;

    ws_write.send(Message::Text(json_data.to_string())).await?;
    sent_verification_data.push(submit_args.verification_data.clone());
    info!("Message sent...");

    // This vector is reversed so that when responses are received, the commitments corresponding
    // to that response can simply be popped of this vector.
    let mut verification_data_commitments_rev: Vec<VerificationDataCommitment> =
        sent_verification_data
            .into_iter()
            .map(|vd| vd.into())
            .rev()
            .collect();

    let ws_write = Arc::new(Mutex::new(ws_write));
    receive(ws_read, ws_write, &mut verification_data_commitments_rev)
        .await
        .map_err(|e| anyhow::anyhow!("Submit error {}", e))?;

    Ok(())
}

async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
) -> Result<Option<AlignedVerificationData>, anyhow::Error> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    let msg = response_stream.next().await;

    if let Some(Ok(Message::Close(close_frame))) = msg {
        if let Some(close_msg) = close_frame {
            error!("Connection was closed before receiving all messages. Reason: {}. Try submitting your proof again", close_msg.to_owned());
            ws_write.lock().await.close().await?;
            return Ok(None);
        }
        error!(
            "Connection was closed before receiving all messages. Try submitting your proof again"
        );
        ws_write.lock().await.close().await?;
        return Ok(None);
    } else if let Some(Ok(Message::Binary(data))) = msg {
        // let data = msg.into_data();
        match serde_json::from_slice::<BatchInclusionData>(&data) {
            Ok(batch_inclusion_data) => {
                info!("Received response from batcher");
                info!(
                    "Batch merkle root: {}",
                    hex::encode(batch_inclusion_data.batch_merkle_root)
                );
                info!("Index in batch: {}", batch_inclusion_data.index_in_batch);
                info!("Proof submitted to aligned. See the batch in the explorer:\nhttps://explorer.alignedlayer.com/batches/0x{}", hex::encode(batch_inclusion_data.batch_merkle_root));

                let verification_data_commitment =
                    verification_data_commitments_rev.pop().unwrap_or_default();

                if verify_response(&verification_data_commitment, &batch_inclusion_data) {
                    return Ok(Some(AlignedVerificationData::new(
                        &verification_data_commitment,
                        &batch_inclusion_data,
                    )));
                }
            }
            Err(e) => {
                error!("Error while deserializing batcher response: {}", e);
            }
        }
    }

    Ok(None)
}

fn verify_response(
    verification_data_commitment: &VerificationDataCommitment,
    batch_inclusion_data: &BatchInclusionData,
) -> bool {
    info!("Verifying response data matches sent proof data ...");
    let batch_inclusion_proof = batch_inclusion_data.batch_inclusion_proof.clone();

    if batch_inclusion_proof.verify::<VerificationCommitmentBatch>(
        &batch_inclusion_data.batch_merkle_root,
        batch_inclusion_data.index_in_batch,
        &verification_data_commitment,
    ) {
        info!("Done. Data sent matches batcher answer");
        return true;
    }

    error!("Verification data commitments and batcher response with merkle root {} and index in batch {} don't match", hex::encode(batch_inclusion_data.batch_merkle_root), batch_inclusion_data.index_in_batch);
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::errors::SubmitError;
    use crate::models::ProvingSystemId;
    use ethers::types::Address;

    use std::path::PathBuf;
    use std::str::FromStr;

    use tokio_tungstenite::connect_async;
    #[tokio::test]
    async fn submit_proof_is_correct() -> Result<(), SubmitError> {
        let proof = read_file(PathBuf::from(
            "/Users/nicolasrampoldi/.aligned/test_files/sp1_fibonacci.proof",
        ))?;
        let elf = Some(read_file(PathBuf::from(
            "/Users/nicolasrampoldi/.aligned/test_files/sp1_fibonacci-elf",
        ))?);
        let (ws_stream, _) = connect_async("ws://localhost:8080")
            .await
            .map_err(|e| SubmitError::ConnectionError(e))?;

        let proof_generator_addr =
            Address::from_str("0x66f9664f97F2b50F62D13eA064982f936dE76657").unwrap();

        let verification_data = VerificationData {
            proving_system: ProvingSystemId::SP1,
            proof: proof,
            pub_input: None,
            verification_key: None,
            vm_program_code: elf,
            proof_generator_addr: proof_generator_addr,
        };

        let submit_args = SubmitArgs {
            ws_stream,
            verification_data,
        };

        submit(submit_args).await?;
        assert!(true);
        Ok(())
    }

    fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
        std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
    }
}
