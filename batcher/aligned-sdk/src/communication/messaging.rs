use ethers::providers::{Http, Provider};
use futures_util::{future, stream::SplitStream, SinkExt, StreamExt, TryStreamExt};
use log::{debug, error};
use std::{collections::HashSet, sync::Arc};
use tokio::{net::TcpStream, sync::Mutex};

use ethers::{core::k256::ecdsa::SigningKey, signers::Wallet};
use futures_util::stream::SplitSink;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    communication::batch::{
        handle_batch_inclusion_data, handle_batch_inclusion_data_without_await,
    },
    core::{
        errors::SubmitError,
        types::{
            AlignedVerificationData, ClientMessage, ResponseMessage, VerificationData,
            VerificationDataCommitment,
        },
    },
    eth::{aligned_service_manager, BatchVerifiedEventStream, BatchVerifiedFilter},
};

pub async fn send_messages(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
) -> Result<Vec<VerificationData>, SubmitError> {
    let mut sent_verification_data = Vec::new();

    let mut ws_write = ws_write.lock().await;

    for verification_data in verification_data.iter() {
        let msg = ClientMessage::new(verification_data.clone(), wallet.clone()).await;
        let msg_str = serde_json::to_string(&msg).map_err(SubmitError::SerdeError)?;
        ws_write
            .send(Message::Text(msg_str.clone()))
            .await
            .map_err(SubmitError::ConnectionError)?;
        sent_verification_data.push(verification_data.clone());
        debug!("Message sent...");
    }

    Ok(sent_verification_data)
}

pub async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    total_messages: usize,
    num_responses: Arc<Mutex<usize>>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
) -> Result<Option<Vec<AlignedVerificationData>>, SubmitError> {
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
            process_batch_inclusion_data_without_await(
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
    }

    Ok(None)
}

pub async fn receive_and_wait(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    eth_rpc_provider: Provider<Http>,
    contract_address: &str,
    total_messages: usize,
    num_responses: Arc<Mutex<usize>>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
) -> Result<Option<Vec<AlignedVerificationData>>, SubmitError> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    let mut aligned_verification_data: Vec<AlignedVerificationData> = Vec::new();

    let service_manager = aligned_service_manager(eth_rpc_provider.clone(), contract_address)
        .await
        .map_err(|e| SubmitError::EthError(e.to_string()))?;

    let events = service_manager.event::<BatchVerifiedFilter>();

    let mut stream = events
        .stream()
        .await
        .map_err(|e| SubmitError::BatchVerifiedEventStreamError(e.to_string()))?;

    // Two different proofs can return the same batch_merkle_root and we don't want wait for the same event twice.
    let mut verified_batch_merkle_roots = HashSet::new();

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
            process_batch_inclusion_data(
                msg,
                &mut aligned_verification_data,
                verification_data_commitments_rev,
                &mut stream,
                &mut verified_batch_merkle_roots,
                num_responses.clone(),
            )
            .await?;

            if *num_responses.lock().await == total_messages {
                debug!("All messages responded. Closing connection...");
                ws_write.lock().await.close().await?;
                return Ok(Some(aligned_verification_data));
            }
        }
    }

    Ok(None)
}

async fn process_batch_inclusion_data<'s>(
    msg: Message,
    aligned_verification_data: &mut Vec<AlignedVerificationData>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
    stream: &mut BatchVerifiedEventStream<'s>,
    verified_batch_merkle_roots: &mut HashSet<Vec<u8>>,
    num_responses: Arc<Mutex<usize>>,
) -> Result<(), SubmitError> {
    let mut num_responses_lock = num_responses.lock().await;
    *num_responses_lock += 1;

    let data = msg.into_data();
    match serde_json::from_slice::<ResponseMessage>(&data) {
        Ok(ResponseMessage::BatchInclusionData(batch_inclusion_data)) => {
            handle_batch_inclusion_data(
                batch_inclusion_data,
                aligned_verification_data,
                verification_data_commitments_rev,
                stream,
                verified_batch_merkle_roots,
            )
            .await?;
        }
        Ok(ResponseMessage::ProtocolVersion(_)) => {
            error!("Batcher responded with protocol version instead of batch inclusion data");
        }
        Err(e) => {
            error!("Error while deserializing batcher response: {}", e);
        }
    }

    Ok(())
}

async fn process_batch_inclusion_data_without_await(
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
            handle_batch_inclusion_data_without_await(
                batch_inclusion_data,
                aligned_verification_data,
                verification_data_commitments_rev,
            )
            .await?;
        }
        Ok(ResponseMessage::ProtocolVersion(_)) => {
            error!("Batcher responded with protocol version instead of batch inclusion data");
        }
        Err(e) => {
            error!("Error while deserializing batcher response: {}", e);
        }
    }

    Ok(())
}
