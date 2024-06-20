extern crate core;

use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use crate::eth::BatchVerifiedEventStream;
use aligned_batcher_lib::types::{
    BatchInclusionData, VerificationCommitmentBatch, VerificationData, VerificationDataCommitment,
};
use aws_sdk_s3::client::Client as S3Client;
use eth::BatchVerifiedFilter;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use futures_util::stream::{self, SplitSink};
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::WebSocketStream;
use types::batch_queue::BatchQueue;
use types::errors::BatcherError;

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};
use crate::eth::AlignedLayerServiceManager;

mod config;
mod eth;
pub mod gnark;
pub mod halo2;
pub mod s3;
pub mod sp1;
pub mod jolt;
pub mod types;
mod zk_utils;

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

pub struct Batcher {
    s3_client: S3Client,
    eth_ws_provider: Provider<Ws>,
    service_manager: AlignedLayerServiceManager,
    batch_queue: Mutex<BatchQueue>,
    max_block_interval: u64,
    min_batch_len: usize,
    max_proof_size: usize,
    max_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
    pre_verification_is_enabled: bool,
}

impl Batcher {
    pub async fn new(config_file: String) -> Self {
        let s3_client = s3::create_client().await;

        let config = ConfigFromYaml::new(config_file);
        let deployment_output =
            ContractDeploymentOutput::new(config.aligned_layer_deployment_config_file_path);

        let eth_ws_provider =
            Provider::connect_with_reconnects(&config.eth_ws_url, config.batcher.eth_ws_reconnects)
                .await
                .expect("Failed to get ethereum websocket provider");

        let eth_rpc_provider =
            eth::get_provider(config.eth_rpc_url.clone()).expect("Failed to get provider");

        // FIXME(marian): We are getting just the last block number right now, but we should really
        // have the last submitted batch block registered and query it when the batcher is initialized.
        let last_uploaded_batch_block = eth_rpc_provider
            .get_block_number()
            .await
            .expect("Failed to get block number")
            .try_into()
            .unwrap();

        let service_manager = eth::get_contract(
            eth_rpc_provider,
            config.ecdsa,
            deployment_output.addresses.aligned_layer_service_manager,
        )
        .await
        .expect("Failed to get Aligned service manager contract");

        Self {
            s3_client,
            eth_ws_provider,
            service_manager,
            batch_queue: Mutex::new(BatchQueue::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_len: config.batcher.batch_size_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
            pre_verification_is_enabled: config.batcher.pre_verification_is_enabled,
        }
    }

    pub async fn listen_connections(self: Arc<Self>, address: &str) -> Result<(), BatcherError> {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address).await.expect("Failed to build");
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let batcher = self.clone();
            tokio::spawn(batcher.handle_connection(stream, addr));
        }
        Ok(())
    }

    pub async fn listen_new_blocks(self: Arc<Self>) -> Result<(), BatcherError> {
        let mut stream = self
            .eth_ws_provider
            .subscribe_blocks()
            .await
            .map_err(|e| BatcherError::EthereumSubscriptionError(e.to_string()))?;

        while let Some(block) = stream.next().await {
            let batcher = self.clone();
            let block_number = block.number.unwrap();
            let block_number = u64::try_from(block_number).unwrap();
            info!("Received new block: {}", block_number);
            tokio::spawn(async move {
                if let Err(e) = batcher.handle_new_block(block_number).await {
                    error!("Error when handling new block: {:?}", e);
                };
            });
        }

        Ok(())
    }

    async fn handle_connection(self: Arc<Self>, raw_stream: TcpStream, addr: SocketAddr) {
        info!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");

        debug!("WebSocket connection established: {}", addr);
        let (outgoing, incoming) = ws_stream.split();

        let outgoing = Arc::new(RwLock::new(outgoing));
        match incoming
            .try_filter(|msg| future::ready(msg.is_text()))
            .try_for_each(|msg| self.clone().handle_message(msg, outgoing.clone()))
            .await
        {
            Err(Error::Protocol(ProtocolError::ResetWithoutClosingHandshake)) => {
                info!("Client {} reset connection", &addr)
            }
            Err(e) => error!("Unexpected error: {}", e),
            Ok(_) => info!("{} disconnected", &addr),
        }
    }

    /// Handle an individual message from the client.
    async fn handle_message(
        self: Arc<Self>,
        message: Message,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        // Deserialize verification data from message
        let verification_data: VerificationData =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        if verification_data.proof.len() <= self.max_proof_size {
            // When pre-verification is enabled, batcher will verify proofs for faster feedback with clients
            if self.pre_verification_is_enabled && !zk_utils::verify(&verification_data) {
                return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                    ProtocolError::HandshakeIncomplete,
                ));
            }
            self.add_to_batch(verification_data, ws_conn_sink.clone())
                .await;
        } else {
            // FIXME(marian): Handle this error correctly
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                ProtocolError::HandshakeIncomplete,
            ));
        };

        info!("Verification data message handled");

        Ok(())
    }

    /// Adds verification data to the current batch queue.
    async fn add_to_batch(
        self: Arc<Self>,
        verification_data: VerificationData,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) {
        let mut batch_queue_lock = self.batch_queue.lock().await;
        info!("Calculating verification data commitments...");
        let verification_data_comm = verification_data.clone().into();
        info!("Adding verification data to batch...");
        batch_queue_lock.push((verification_data, verification_data_comm, ws_conn_sink));
        info!("Current batch queue length: {}", batch_queue_lock.len());
    }

    /// Given a new block number listened from the blockchain, checks if the current batch is ready to be posted.
    /// There are essentially two conditions to be checked:
    ///     * Has the current batch reached the minimum size to be posted?
    ///     * Has the received block number surpassed the maximum interval with respect to the last posted batch block?
    /// An extra sanity check is made to check if the batch size is 0, since it does not make sense to post
    /// an empty batch, even if the block interval has been reached.
    /// Once the batch meets the conditions for submission, it check if it needs to be splitted into smaller batches,
    /// depending on the configured maximum batch size. The batch is splitted at the index where the max size is surpassed,
    /// and all the elements up to that index are copied and cleared from the batch queue. The copy is then passed to the
    /// `finalize_batch` function.
    async fn is_batch_ready(&self, block_number: u64) -> Option<BatchQueue> {
        let mut batch_queue_lock = self.batch_queue.lock().await;
        let current_batch_len = batch_queue_lock.len();

        let last_uploaded_batch_block_lock = self.last_uploaded_batch_block.lock().await;

        // FIXME(marian): This condition should be changed to current_batch_size == 0
        // once the bug in Lambdaworks merkle tree is fixed.
        if current_batch_len < 2 {
            info!("Current batch is empty or length 1. Waiting for more proofs...");
            return None;
        }

        if current_batch_len < self.min_batch_len
            && block_number < *last_uploaded_batch_block_lock + self.max_block_interval
        {
            info!(
                "Current batch not ready to be posted. Current block: {} - Last uploaded block: {}. Current batch length: {} - Minimum batch length: {}",
                block_number, *last_uploaded_batch_block_lock, current_batch_len, self.min_batch_len
            );
            return None;
        }

        let batch_verification_data: Vec<VerificationData> = batch_queue_lock
            .iter()
            .map(|(vd, _, _)| vd.clone())
            .collect();

        let current_batch_size = serde_json::to_vec(&batch_verification_data).unwrap().len();

        // check if the current batch needs to be splitted into smaller batches
        if current_batch_size > self.max_batch_size {
            info!("Batch max size exceded. Splitting current batch...");
            let mut acc_batch_size = 0;
            let mut finalized_batch_idx = 0;
            for (idx, (verification_data, _, _)) in batch_queue_lock.iter().enumerate() {
                acc_batch_size += serde_json::to_vec(verification_data).unwrap().len();
                if acc_batch_size > self.max_batch_size {
                    finalized_batch_idx = idx;
                    break;
                }
            }
            let finalized_batch = batch_queue_lock.drain(..finalized_batch_idx).collect();
            return Some(finalized_batch);
        }

        // A copy of the batch is made to be returned and the current batch is cleared
        let finalized_batch = batch_queue_lock.clone();
        batch_queue_lock.clear();

        Some(finalized_batch)
    }

    /// Takes the finalized batch as input and builds the merkle tree, posts verification data batch
    /// to s3, creates new task in Aligned contract and sends responses to all clients that added proofs
    /// to the batch. The last uploaded batch block is updated once the task is created in Aligned.
    async fn finalize_batch(
        &self,
        block_number: u64,
        finalized_batch: BatchQueue,
        wait_for_verification: bool,
    ) -> Result<(), BatcherError> {
        let batch_verification_data: Vec<VerificationData> = finalized_batch
            .clone()
            .into_iter()
            .map(|(data, _, _)| data)
            .collect();

        let batch_bytes = serde_json::to_vec(batch_verification_data.as_slice())
            .expect("Failed to serialize batch");

        info!("Finalizing batch. Length: {}", finalized_batch.len());
        let batch_data_comm: Vec<VerificationDataCommitment> = finalized_batch
            .clone()
            .into_iter()
            .map(|(_, data_comm, _)| data_comm)
            .collect();

        let batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
            MerkleTree::build(&batch_data_comm);

        let events = self.service_manager.event::<BatchVerifiedFilter>();
        let mut stream = events
            .stream()
            .await
            .map_err(|e| BatcherError::BatchVerifiedEventStreamError(e.to_string()))?;

        {
            let mut last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;
            self.submit_batch(&batch_bytes, &batch_merkle_tree.root)
                .await;
            // update last uploaded batch block
            *last_uploaded_batch_block = block_number;
        }

        if !wait_for_verification {
            send_batch_inclusion_data_responses(finalized_batch, &batch_merkle_tree).await;
            return Ok(());
        }

        // This future is created to be passed to the timeout function, so that if it is not resolved
        // within the timeout interval an error is raised. If the event is received, responses are sent to
        // connected clients
        let await_batch_verified_fut =
            await_batch_verified_event(&mut stream, &batch_merkle_tree.root);
        if let Err(_) = timeout(Duration::from_secs(60), await_batch_verified_fut).await {
            send_timeout_close(finalized_batch).await?;
        } else {
            send_batch_inclusion_data_responses(finalized_batch, &batch_merkle_tree).await;
        }

        Ok(())
    }

    /// Receives new block numbers, checks if conditions are met for submission and
    /// finalizes the batch.
    async fn handle_new_block(&self, block_number: u64) -> Result<(), BatcherError> {
        while let Some(finalized_batch) = self.is_batch_ready(block_number).await {
            self.finalize_batch(block_number, finalized_batch, false)
                .await?;
        }
        Ok(())
    }

    /// Post batch to s3 and submit new task to Ethereum
    async fn submit_batch(&self, batch_bytes: &[u8], batch_merkle_root: &[u8; 32]) {
        let s3_client = self.s3_client.clone();
        let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        info!("Batch merkle root: {}", batch_merkle_root_hex);
        let file_name = batch_merkle_root_hex.clone() + ".json";

        info!("Uploading batch to S3...");
        s3::upload_object(&s3_client, S3_BUCKET_NAME, batch_bytes.to_vec(), &file_name)
            .await
            .expect("Failed to upload object to S3");

        info!("Batch sent to S3 with name: {}", file_name);

        info!("Uploading batch to contract");
        let service_manager = &self.service_manager;
        let batch_data_pointer = "https://".to_owned() + S3_BUCKET_NAME + "/" + &file_name;
        match eth::create_new_task(service_manager, *batch_merkle_root, batch_data_pointer).await {
            Ok(_) => info!("Batch verification task created on Aligned contract"),
            Err(e) => error!("Failed to create batch verification task: {}", e),
        }
    }
}
/// Await for the `BatchVerified` event emitted by the Aligned contract and then send responses.
async fn await_batch_verified_event<'s>(
    events_stream: &mut BatchVerifiedEventStream<'s>,
    batch_merkle_root: &[u8; 32],
) -> Result<(), BatcherError> {
    while let Some(event_result) = events_stream.next().await {
        if let Ok(event) = event_result {
            if &event.batch_merkle_root == batch_merkle_root {
                info!("Batch operator signatures verified on Ethereum. Sending response to clients...");
                break;
            }
        } else {
            error!("Error awaiting for batch signature verification event");
            return Err(BatcherError::BatchVerifiedEventStreamError(
                event_result.unwrap_err().to_string(),
            ));
        }
    }
    Ok(())
}

async fn send_batch_inclusion_data_responses(
    finalized_batch: BatchQueue,
    batch_merkle_tree: &MerkleTree<VerificationCommitmentBatch>,
) {
    stream::iter(finalized_batch.iter())
        .enumerate()
        .for_each(|(vd_batch_idx, (_, vdc, ws_sink))| async move {
            let response = BatchInclusionData::new(vdc, vd_batch_idx, batch_merkle_tree);
            let serialized_response =
                serde_json::to_vec(&response).expect("Could not serialize response");

            let sending_result = ws_sink
                .write()
                .await
                .send(Message::binary(serialized_response))
                .await;

            match sending_result {
                Err(Error::AlreadyClosed) => (),
                Err(e) => error!("Error while sending batch inclusion data response: {}", e),
                Ok(_) => (),
            }

            info!("Response sent");
        })
        .await;
}

/// Send a close response to all clients that included data in the batch indicated that a
/// timeout was exceeded awaiting for the batch verification events
async fn send_timeout_close(finalized_batch: BatchQueue) -> Result<(), BatcherError> {
    let timeout_msg = Message::Close(Some(CloseFrame {
        code: CloseCode::Protocol,
        reason: Cow::from("Timeout: BatchVerified event not received"),
    }));

    for (_, _, ws_sink) in finalized_batch.iter() {
        let send_result = ws_sink.write().await.send(timeout_msg.clone()).await;
        match send_result {
            // When two or more proofs from the same client are included into a batch,
            // there will be more than one `ws_sink` corresponding to that client. When one is
            // closed, the other ones will raise this error. We can just ignore it.
            Err(Error::Protocol(ProtocolError::SendAfterClosing)) => (),
            Err(e) => {
                error!("Error sending timeout response to clients: {}", e);
                return Err(e.into());
            }
            Ok(_) => (),
        }

        info!("Timeout close response sent");
    }
    Ok(())
}
