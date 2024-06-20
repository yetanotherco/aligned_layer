mod errors;

//here imports to other parts of code
// refactor them inside the SDK
use aligned_batcher_lib::types::{
    BatchInclusionData, VerificationData, VerificationDataCommitment, VerificationCommitmentBatch
};
// ^^

use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use tokio::{net::TcpStream, sync::Mutex};

use log::{error, info};
use std::sync::Arc;

pub struct SubmitArgs {
    // ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    verification_data: VerificationData,
}
use futures_util::{
    future, TryStreamExt, SinkExt, StreamExt,
    stream::{SplitSink, SplitStream}
};
use ethers::utils::hex;

/// Submits proof to batcher
/// 
/// Example
/// 
pub async fn submit(mut submit_args: SubmitArgs) -> Result<(), errors::SubmitError> {
    
    // The sent verification data will be stored here so that we can calculate
    // their commitments later.
    let mut sent_verification_data: Vec<VerificationData> = Vec::new(); // todo ?
    
    let json_data = serde_json::to_string(&submit_args.verification_data)?; // todo check if clone

    submit_args.ws_write.send(Message::Text(json_data.to_string())).await?;
    sent_verification_data.push(submit_args.verification_data.clone()); // todo check if clone
    info!("Message sent...");

    
    // This vector is reversed so that when responses are received, the commitments corresponding
    // to that response can simply be popped of this vector.
    let mut verification_data_commitments_rev: Vec<VerificationDataCommitment> =
        sent_verification_data
            .into_iter()
            .map(|vd| vd.into())
            .rev()
            .collect();
    
    receive(
        submit_args.ws_read,
        &mut verification_data_commitments_rev,
    )
    .await?;
    Ok(())
}


async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
) -> Result<(), errors::SubmitError> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    let Some(Ok(msg)) = response_stream.next().await else { 
        todo!()
    };
    if let Message::Close(close_frame) = msg {
        if let Some(close_msg) = close_frame {
            error!("Connection was closed before receiving response message. Reason: {}. Try submitting your proof again", close_msg.to_owned());
            return Ok(()); // TODO return error, from batcher error lib
            // create/import batcher error codes:
            // https://github.com/yetanotherco/aligned_layer/blob/main/batcher/aligned/src/errors.rs

        }
        error!("Connection was closed before receiving response message. Try submitting your proof again");
        return Ok(()); // TODO return error, from batcher error lib
    } else {
        let data = msg.into_data();
        match serde_json::from_slice::<BatchInclusionData>(&data) {
            Ok(batch_inclusion_data) => {
                
                let batch_merkle_root = hex::encode(batch_inclusion_data.batch_merkle_root);
                info!("Batcher response receive, merkle root: {}", batch_merkle_root);
                
                // file.write_all(data.as_slice()).unwrap(); //TODO return this
                let verification_data_commitment = 
                    verification_data_commitments_rev.pop().unwrap_or_default();

                verify_response(&verification_data_commitment, &batch_inclusion_data);
            }
            Err(e) => {
                error!("Error while deserializing batcher response: {}", e);
            }
        }
    }

    Ok(())
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

    use aligned_batcher_lib::types::ProvingSystemId;

    use std::path::PathBuf;

    use tokio_tungstenite::connect_async;
    use ethers::types::Address;
    #[tokio::test]
    async fn normal_flow() -> Result<(), errors::SubmitError> {
    
        let proof = read_file("/Users/urix/aligned_layer/batcher/aligned/test_files/plonk_bls12_381/plonk.proof".into())?;
        
        let pub_input = Some(read_file("/Users/urix/aligned_layer/batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub".into())?);
        let verification_key = Some(read_file("/Users/urix/aligned_layer/batcher/aligned/test_files/plonk_bls12_381/plonk.vk".into())?);
        let vm_program_code = None;
        let proof_generator_addr = Address::random();
        let url = "ws://localhost:8080";

        let (ws_stream, _) = connect_async(url).await?;
        info!("WebSocket handshake has been successfully completed");
        let (ws_write, ws_read) = ws_stream.split();

        
        let submit_args_1 = SubmitArgs {
            ws_read: ws_read,
            ws_write: ws_write,
            verification_data: VerificationData {
                proving_system: ProvingSystemId::GnarkPlonkBls12_381,
                proof: proof,
                pub_input: pub_input,
                verification_key: verification_key,
                vm_program_code: vm_program_code,
                proof_generator_addr: proof_generator_addr,
            },
        };

        // let ws_write_m = Arc::new(Mutex::new(ws_write));
        // let ws_read_m = Arc::new(Mutex::new(ws_read));

        // let submit_args_2 = SubmitArgs {
        //     ws_read: ws_read_m,
        //     ws_write: ws_write_m,
        //     verification_data: VerificationData {
        //         proving_system: ProvingSystemId::GnarkPlonkBls12_381,
        //         proof: proof.clone(),
        //         pub_input: pub_input.clone(),
        //         verification_key: verification_key.clone(),
        //         vm_program_code: vm_program_code.clone(),
        //         proof_generator_addr: proof_generator_addr,
        //     },
        // };
        let future1 = submit(submit_args_1);
        // let future2 = submit(submit_args_2);
        // let (_result1, _result2) = tokio::join!(future1, future2);
        let _ = future1.await;
        assert! (true);
        Ok(())
    }

    fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
        std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
    }
}