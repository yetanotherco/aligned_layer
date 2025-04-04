use ethers::signers::Signer;
use ethers::types::Address;
use futures_util::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

use ethers::{core::k256::ecdsa::SigningKey, signers::Wallet, types::U256};
use futures_util::future::Ready;
use futures_util::stream::{SplitSink, TryFilter};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::communication::serialization::{cbor_deserialize, cbor_serialize};
use crate::core::types::{BatchInclusionData, SubmitProofMessage};
use crate::{
    communication::batch::process_batcher_response,
    core::{
        errors::SubmitError,
        types::{
            AlignedVerificationData, ClientMessage, NoncedVerificationData,
            SubmitProofResponseMessage, VerificationData, VerificationDataCommitment,
        },
    },
};

pub type ResponseStream = TryFilter<
    SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    Ready<bool>,
    fn(&Message) -> Ready<bool>,
>;

// Sends the proofs to the batcher via WS
// Stores the proofs sent in an array
// Returns the array
pub async fn send_messages(
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    payment_service_addr: Address,
    verification_data: &[VerificationData],
    max_fee: U256,
    wallet: Wallet<SigningKey>,
    mut nonce: U256,
) -> Vec<Result<NoncedVerificationData, SubmitError>> {
    let chain_id = U256::from(wallet.chain_id());
    let mut ws_write = ws_write.lock().await;
    let mut sent_verification_data: Vec<Result<NoncedVerificationData, SubmitError>> = Vec::new();

    for (idx, verification_data_i) in verification_data.iter().enumerate() {
        // Build each message to send
        let verification_data = NoncedVerificationData::new(
            verification_data_i.clone(),
            nonce,
            max_fee,
            chain_id,
            payment_service_addr,
        );

        nonce += U256::one();
        let data = SubmitProofMessage::new(verification_data.clone(), wallet.clone()).await;
        let msg = ClientMessage::SubmitProof(Box::new(data));

        let msg_bin = match cbor_serialize(&msg) {
            Ok(bin) => bin,
            Err(e) => {
                error!("Error while serializing message: {:?}", e);
                sent_verification_data.push(Err(SubmitError::SerializationError(e)));
                return sent_verification_data;
            }
        };

        // Send the message
        if let Err(e) = ws_write.send(Message::Binary(msg_bin.clone())).await {
            error!("Error while sending message: {:?}", e);
            sent_verification_data.push(Err(SubmitError::WebSocketConnectionError(e)));
            return sent_verification_data;
        }

        debug!("{:?} Message sent", idx);

        // Save the verification data commitment to read its response later
        sent_verification_data.push(Ok(verification_data));
    }

    info!("All proofs sent");
    // This vector is reversed so that while responses are received, removing from the end is cheaper.
    let sent_verification_data_rev: Vec<Result<NoncedVerificationData, SubmitError>> =
        sent_verification_data.into_iter().rev().collect();
    sent_verification_data_rev
}

// Receives the array of proofs sent
// Reads the WS responses
// Matches each response with the corresponding proof sent
// finishes when the last proof sent receives its response
// finishes early if the batcher replies with a SubmitError
pub async fn receive(
    response_stream: Arc<Mutex<ResponseStream>>,
    mut sent_verification_data_rev: Vec<Result<NoncedVerificationData, SubmitError>>,
) -> Vec<Result<AlignedVerificationData, SubmitError>> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream = response_stream.lock().await;
    let mut aligned_submitted_data: Vec<Result<AlignedVerificationData, SubmitError>> = Vec::new();
    let last_proof_nonce = get_biggest_nonce(&sent_verification_data_rev);

    // read from WS
    while let Some(Ok(msg)) = response_stream.next().await {
        // unexpected WS close:
        if let Message::Close(close_frame) = msg {
            warn!("Unexpected WS close");
            if let Some(close_msg) = close_frame {
                aligned_submitted_data.push(Err(SubmitError::WebSocketClosedUnexpectedlyError(
                    close_msg.to_owned(),
                )));
                break;
            }
            aligned_submitted_data.push(Err(SubmitError::GenericError(
                "Connection was closed before receive() processed all sent messages ".to_string(),
            )));
            break;
        }

        // first error msg from batcher will drop the rest of the messages in the burst

        let batch_inclusion_data_message = match handle_batcher_response(msg).await {
            Ok(data) => data,
            Err(e) => {
                warn!("Error while handling batcher response: {:?}", e);
                aligned_submitted_data.push(Err(e));
                break;
            }
        };

        let related_verification_data = match match_batcher_response_with_stored_verification_data(
            &batch_inclusion_data_message,
            &mut sent_verification_data_rev,
        ) {
            Ok(data) => data,
            Err(e) => {
                warn!(
                    "Error while matching batcher response with sent data: {:?}",
                    e
                );
                aligned_submitted_data.push(Err(e));
                break;
            }
        };

        let aligned_verification_data = match process_batcher_response(
            &batch_inclusion_data_message,
            &related_verification_data,
        ) {
            Ok(data) => data,
            Err(e) => {
                warn!("Error while processing batcher response: {:?}", e);
                aligned_submitted_data.push(Err(e));
                break;
            }
        };

        aligned_submitted_data.push(Ok(aligned_verification_data));
        debug!("Message response handled successfully");

        if batch_inclusion_data_message.user_nonce == last_proof_nonce {
            break;
        }
    }

    aligned_submitted_data
}

async fn handle_batcher_response(msg: Message) -> Result<BatchInclusionData, SubmitError> {
    let data = msg.into_data();
    match cbor_deserialize(data.as_slice()) {
        Ok(SubmitProofResponseMessage::BatchInclusionData(batch_inclusion_data)) => {
            //OK case. Proofs was valid and it was included in this batch.
            Ok(batch_inclusion_data)
        }
        Ok(SubmitProofResponseMessage::InvalidNonce) => {
            error!("Batcher responded with invalid nonce. Funds have not been spent.");
            Err(SubmitError::InvalidNonce)
        }
        Ok(SubmitProofResponseMessage::InvalidSignature) => {
            error!("Batcher responded with invalid signature. Funds have not been spent.");
            Err(SubmitError::InvalidSignature)
        }
        Ok(SubmitProofResponseMessage::ProofTooLarge) => {
            error!("Batcher responded with proof too large. Funds have not been spent.");
            Err(SubmitError::ProofTooLarge)
        }
        Ok(SubmitProofResponseMessage::InvalidMaxFee) => {
            error!("Batcher responded with invalid max fee. Funds have not been spent.");
            Err(SubmitError::InvalidMaxFee)
        }
        Ok(SubmitProofResponseMessage::InsufficientBalance(addr)) => {
            error!("Batcher responded with insufficient balance. Funds have not been spent for submittions which had insufficient balance.");
            Err(SubmitError::InsufficientBalance(addr))
        }
        Ok(SubmitProofResponseMessage::InvalidChainId) => {
            error!("Batcher responded with invalid chain id. Funds have not been spent.");
            Err(SubmitError::InvalidChainId)
        }
        Ok(SubmitProofResponseMessage::InvalidReplacementMessage) => {
            error!(
                "Batcher responded with invalid replacement message. Funds have not been spent."
            );
            Err(SubmitError::InvalidReplacementMessage)
        }
        Ok(SubmitProofResponseMessage::AddToBatchError) => {
            error!("Batcher responded with add to batch error. Funds have not been spent.");
            Err(SubmitError::AddToBatchError)
        }
        Ok(SubmitProofResponseMessage::EthRpcError) => {
            error!("Batcher experienced Eth RPC connection error. Funds have not been spent.");
            Err(SubmitError::EthereumProviderError(
                "Batcher experienced Eth RPC connection error. Funds have not been spent."
                    .to_string(),
            ))
        }
        Ok(SubmitProofResponseMessage::InvalidPaymentServiceAddress(
            received_addr,
            expected_addr,
        )) => {
            error!(
                "Batcher responded with invalid payment service address: {:?}, expected: {:?}. Funds have not been spent.",
                received_addr, expected_addr
            );
            Err(SubmitError::InvalidPaymentServiceAddress(
                received_addr,
                expected_addr,
            ))
        }
        Ok(SubmitProofResponseMessage::InvalidProof(reason)) => {
            error!(
                "Batcher responded with invalid proof: {}. Funds have not been spent.",
                reason
            );
            Err(SubmitError::InvalidProof(reason))
        }
        Ok(SubmitProofResponseMessage::CreateNewTaskError(merkle_root, error)) => {
            error!(
                "Batcher responded with create new task error: {}. Funds have not been spent.",
                error
            );
            Err(SubmitError::BatchSubmissionFailed(
                "Could not create task with merkle root ".to_owned()
                    + &merkle_root
                    + ", failed with error: "
                    + &error,
            ))
        }
        Ok(SubmitProofResponseMessage::ProtocolVersion(_)) => {
            error!("Batcher responded with protocol version instead of batch inclusion data. Funds have not been spent.");
            Err(SubmitError::UnexpectedBatcherResponse(
                "Batcher responded with protocol version instead of batch inclusion data. Funds have not been spent."
                    .to_string(),
            ))
        }
        Ok(SubmitProofResponseMessage::BatchReset) => {
            error!("Batcher responded with batch reset. Funds have not been spent.");
            Err(SubmitError::ProofQueueFlushed)
        }
        Ok(SubmitProofResponseMessage::Error(e)) => {
            error!(
                "Batcher responded with error: {}. Funds have not been spent.",
                e
            );
            Err(SubmitError::GenericError(e))
        }
        Err(e) => {
            error!(
                "Error while deserializing batch inclusion data: {}. Funds have not been spent.",
                e
            );
            Err(SubmitError::SerializationError(e))
        }
    }
}

// Used to match the message received from the batcher,
// with the NoncedVerificationData you sent
// This is used to verify the proof you sent was indeed included in the batch
fn match_batcher_response_with_stored_verification_data(
    batch_inclusion_data: &BatchInclusionData,
    sent_verification_data_rev: &mut Vec<Result<NoncedVerificationData, SubmitError>>,
) -> Result<VerificationDataCommitment, SubmitError> {
    debug!("Matching verification data with batcher response ...");
    let mut index = None;
    for (i, sent_nonced_verification_data) in
        sent_verification_data_rev.iter_mut().enumerate().rev()
    {
        // iterate in reverse since the last element is the most probable to match
        if let Ok(sent_nonced_verification_data) = sent_nonced_verification_data {
            if sent_nonced_verification_data.nonce == batch_inclusion_data.user_nonce {
                debug!("local nonced verification data matched with batcher response");
                index = Some(i);
                break;
            }
        }
    }

    // cant remove an element while iterating, so we remove it here
    if let Some(i) = index {
        let verification_data = sent_verification_data_rev.remove(i).unwrap();
        return Ok(verification_data.verification_data.clone().into());
    }

    Err(SubmitError::InvalidProofInclusionData)
}

// Returns the biggest nonce from the sent verification data
// Used to know which is the last proof sent to the Batcher,
// to know when to stop reading the WS for responses
fn get_biggest_nonce(
    sent_verification_data: &[Result<NoncedVerificationData, SubmitError>],
) -> U256 {
    let mut biggest_nonce = U256::zero();
    for verification_data in sent_verification_data.iter().flatten() {
        if verification_data.nonce > biggest_nonce {
            biggest_nonce = verification_data.nonce;
        }
    }
    biggest_nonce
}
