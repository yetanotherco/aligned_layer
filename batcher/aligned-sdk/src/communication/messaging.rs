use ethers::signers::Signer;
use ethers::types::Address;
use futures_util::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, error, info};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

use ethers::{core::k256::ecdsa::SigningKey, signers::Wallet, types::U256};
use futures_util::future::Ready;
use futures_util::stream::{SplitSink, TryFilter};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::communication::serialization::{cbor_deserialize, cbor_serialize};
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
    payment_service_addr: Address,
    verification_data: &[VerificationData],
    max_fees: &[U256],
    wallet: Wallet<SigningKey>,
    mut nonce: U256,
) -> Result<Vec<NoncedVerificationData>, SubmitError> {
    let mut sent_verification_data = Vec::new();

    let mut ws_write = ws_write.lock().await;

    let mut response_stream = response_stream.lock().await;

    let chain_id = U256::from(wallet.chain_id());

    for (idx, verification_data) in verification_data.iter().enumerate() {
        let verification_data = NoncedVerificationData::new(
            verification_data.clone(),
            nonce,
            max_fees[idx],
            chain_id,
            payment_service_addr,
        );

        nonce += U256::one();

        let msg = ClientMessage::new(verification_data.clone(), wallet.clone()).await;
        let msg_bin = cbor_serialize(&msg).map_err(SubmitError::SerializationError)?;
        ws_write
            .send(Message::Binary(msg_bin.clone()))
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

        let response_msg: ValidityResponseMessage = cbor_deserialize(msg.into_data().as_slice())
            .map_err(SubmitError::SerializationError)?;

        match response_msg {
            ValidityResponseMessage::Valid => {
                debug!("Message was valid");
            }
            ValidityResponseMessage::InvalidNonce => {
                info!("Invalid Nonce!");
                return Err(SubmitError::InvalidNonce);
            }
            ValidityResponseMessage::InvalidSignature => {
                error!("Invalid Signature!");
                return Err(SubmitError::InvalidSignature);
            }
            ValidityResponseMessage::ProofTooLarge => {
                error!("Proof too large!");
                return Err(SubmitError::ProofTooLarge);
            }
            ValidityResponseMessage::InvalidProof(reason) => {
                error!("Invalid Proof!: {}", reason);
                return Err(SubmitError::InvalidProof(reason));
            }
            ValidityResponseMessage::InvalidMaxFee => {
                error!("Invalid Max Fee!");
                return Err(SubmitError::InvalidMaxFee);
            }
            ValidityResponseMessage::InsufficientBalance(addr) => {
                error!("Insufficient balance for address: {}", addr);
                return Err(SubmitError::InsufficientBalance);
            }
            ValidityResponseMessage::InvalidChainId => {
                error!("Invalid chain id!");
                return Err(SubmitError::InvalidChainId);
            }
            ValidityResponseMessage::InvalidReplacementMessage => {
                error!("Invalid replacement message!");
                return Err(SubmitError::InvalidReplacementMessage);
            }
            ValidityResponseMessage::AddToBatchError => {
                error!("Error while pushing the entry to queue");
                return Err(SubmitError::AddToBatchError);
            }
            ValidityResponseMessage::EthRpcError => {
                return Err(SubmitError::EthereumProviderError(
                    "Batcher experienced Eth RPC connection error".to_string(),
                ));
            }
            ValidityResponseMessage::InvalidPaymentServiceAddress(received_addr, expected_addr) => {
                error!(
                    "Invalid payment service address, received: {}, expected: {}",
                    received_addr, expected_addr
                );
                return Err(SubmitError::InvalidPaymentServiceAddress(
                    received_addr,
                    expected_addr,
                ));
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
) -> Result<Vec<AlignedVerificationData>, SubmitError> {
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
            return Ok(aligned_verification_data);
        }
    }

    Err(SubmitError::GenericError(
        "Connection was closed without close message before receiving all messages".to_string(),
    ))
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
    match cbor_deserialize(data.as_slice()) {
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
        Ok(ResponseMessage::BatchReset) => {
            return Err(SubmitError::ProofQueueFlushed);
        }
        Ok(ResponseMessage::Error(e)) => {
            error!("Batcher responded with error: {}", e);
        }
        Ok(ResponseMessage::CreateNewTaskError(merkle_root)) => {
            return Err(SubmitError::BatchSubmissionFailed(
                "Could not create task with merkle root ".to_owned() + &merkle_root,
            ));
        }
        Ok(ResponseMessage::InvalidProof(reason)) => {
            return Err(SubmitError::InvalidProof(reason));
        }
        Err(e) => {
            return Err(SubmitError::SerializationError(e));
        }
    }

    Ok(())
}
