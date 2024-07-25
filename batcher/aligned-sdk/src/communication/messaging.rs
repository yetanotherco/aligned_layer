use futures_util::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, error, info};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

use ethers::{core::k256::ecdsa::SigningKey, signers::Wallet, types::U256};
use futures_util::future::Ready;
use futures_util::stream::{SplitSink, TryFilter};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    communication::batch::handle_batch_inclusion_data,
    core::{
        errors::SubmitError,
        types::{
            AlignedVerificationData, ClientMessage, NoncedVerificationData, ResponseMessage,
            ValidityResponseMessage, VerificationData, VerificationDataCommitment,
        },
    },
};

pub type ResponseStream = TryFilter<
    SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    Ready<bool>,
    fn(&Message) -> Ready<bool>,
>;

pub async fn send_messages(
    response_stream: Arc<Mutex<ResponseStream>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Vec<NoncedVerificationData>, SubmitError> {
    let mut sent_verification_data = Vec::new();

    let mut ws_write = ws_write.lock().await;

    let mut nonce = nonce.clone();
    let mut nonce_bytes = [0u8; 32];

    let mut response_stream = response_stream.lock().await;

    for verification_data in verification_data.iter() {
        nonce.to_big_endian(&mut nonce_bytes);

        let verification_data = NoncedVerificationData::new(verification_data.clone(), nonce_bytes);
        nonce += U256::one();

        let msg = ClientMessage::new(verification_data.clone(), wallet.clone());
        let msg_str = serde_json::to_string(&msg).map_err(SubmitError::SerializationError)?;
        ws_write
            .send(Message::Text(msg_str.clone()))
            .await
            .map_err(SubmitError::WebSocketConnectionError)?;

        debug!("Message sent...");

        let msg = match response_stream.next().await {
            Some(Ok(msg)) => msg,
            _ => {
                return Err(SubmitError::GenericError(
                    "Connection was closed without close message before receiving all messages"
                        .to_string(),
                ));
            }
        };

        let response_msg = serde_json::from_slice::<ValidityResponseMessage>(&msg.into_data())
            .map_err(SubmitError::SerializationError)?;

        match response_msg {
            ValidityResponseMessage::Valid => {
                debug!("Message was valid");
            }
            ValidityResponseMessage::InvalidNonce => {
                info!("Invalid Nonce!");
                // TODO: handle (invalidate local cache)
            }
            ValidityResponseMessage::InvalidSignature => {
                error!("Invalid Signature!");
                return Err(SubmitError::InvalidSignature);
            }
            ValidityResponseMessage::ProofTooLarge => {
                error!("Proof too large!");
                return Err(SubmitError::ProofTooLarge);
            }
            ValidityResponseMessage::InvalidProof => {
                error!("Invalid Proof!");
                return Err(SubmitError::InvalidProof);
            }
            ValidityResponseMessage::InsufficientBalance(addr) => {
                error!("Insufficient balance for address: {}", addr);
                return Err(SubmitError::InsufficientBalance);
            }
        };

        sent_verification_data.push(verification_data.clone());
    }

    Ok(sent_verification_data)
}

pub async fn receive(
    response_stream: Arc<Mutex<ResponseStream>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    total_messages: usize,
    num_responses: Arc<Mutex<usize>>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
) -> Result<Option<Vec<AlignedVerificationData>>, SubmitError> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream = response_stream.lock().await;

    let mut aligned_verification_data: Vec<AlignedVerificationData> = Vec::new();

    while let Some(Ok(msg)) = response_stream.next().await {
        if let Message::Close(close_frame) = msg {
            if let Some(close_msg) = close_frame {
                return Err(SubmitError::WebSocketClosedUnexpectedlyError(
                    close_msg.to_owned(),
                ));
            }
            return Err(SubmitError::GenericError(
                "Connection was closed without close message before receiving all messages"
                    .to_string(),
            ));
        }
        process_batch_inclusion_data(
            msg,
            &mut aligned_verification_data,
            verification_data_commitments_rev,
            num_responses.clone(),
        )
        .await?;

        if *num_responses.lock().await == total_messages {
            debug!("All messages responded. Closing connection...");
            ws_write.lock().await.close().await?;
            return Ok(Some(aligned_verification_data));
        }
    }

    Ok(None)
}

async fn process_batch_inclusion_data(
    msg: Message,
    aligned_verification_data: &mut Vec<AlignedVerificationData>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
    num_responses: Arc<Mutex<usize>>,
) -> Result<(), SubmitError> {
    let mut num_responses_lock = num_responses.lock().await;
    *num_responses_lock += 1;

    let data = msg.into_data();
    match serde_json::from_slice::<ResponseMessage>(&data) {
        Ok(ResponseMessage::BatchInclusionData(batch_inclusion_data)) => {
            let _ = handle_batch_inclusion_data(
                batch_inclusion_data,
                aligned_verification_data,
                verification_data_commitments_rev,
            );
        }
        Ok(ResponseMessage::ProtocolVersion(_)) => {
            return Err(SubmitError::UnexpectedBatcherResponse(
                "Batcher responded with protocol version instead of batch inclusion data"
                    .to_string(),
            ));
        }
        Ok(ResponseMessage::Error(e)) => {
            error!("Batcher responded with error: {}", e);
        }
        Ok(ResponseMessage::CreateNewTaskError(merkle_root)) => {
            return Err(SubmitError::CreateNewTaskError(
                "Could not create task with merkle root ".to_owned() + &merkle_root,
            ));
        }
        Err(e) => {
            return Err(SubmitError::SerializationError(e));
        }
    }

    Ok(())
}
