extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_s3::client::Client as S3Client;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use futures_util::stream::{self, SplitSink};
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use types::VerificationCommitmentBatch;

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};
use crate::eth::AlignedLayerServiceManager;
use crate::types::VerificationData;

mod config;
mod connection;
mod eth;
pub mod gnark;
pub mod halo2;
pub mod s3;
pub mod types;

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

pub struct Batcher {
    s3_client: S3Client,
    eth_ws_provider: Provider<Ws>,
    service_manager: AlignedLayerServiceManager,
    // message_queue / proof_queue / batch_queue
    // VerificationData, subscriber/splitWebSocket
    batch_queue: Mutex<
        Vec<(
            VerificationData,
            Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        )>,
    >,
    max_block_interval: u64,
    min_batch_len: usize,
    max_proof_size: usize,
    max_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
}

impl Batcher {
    pub async fn new(config_file: String) -> Self {
        let s3_client = s3::create_client().await;

        let config = ConfigFromYaml::new(config_file);
        let deployment_output =
            ContractDeploymentOutput::new(config.aligned_layer_deployment_config_file_path);

        let eth_ws_provider = Provider::connect(&config.eth_ws_url)
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
            batch_queue: Mutex::new(Vec::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_len: config.batcher.batch_size_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
        }
    }

    pub async fn listen_connections(self: Arc<Self>, address: &str) {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address).await.expect("Failed to build");
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let batcher = self.clone();
            tokio::spawn(batcher.handle_connection(stream, addr));
        }
    }

    pub async fn listen_new_blocks(self: Arc<Self>) -> Result<(), anyhow::Error> {
        let mut stream = self.eth_ws_provider.subscribe_blocks().await?;
        while let Some(block) = stream.next().await {
            let batcher = self.clone();
            let block_number = block.number.unwrap();
            let block_number = u64::try_from(block_number).unwrap();
            info!("Received new block: {}", block_number);
            tokio::spawn(async move {
                batcher.handle_new_block(block_number).await;
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

        incoming
            .try_filter(|msg| future::ready(msg.is_text()))
            .try_for_each(|msg| self.clone().handle_message(msg, outgoing.clone()))
            .await
            .unwrap();

        info!("{} disconnected", &addr);
    }

    /// Handle an individual message from the client.
    async fn handle_message(
        self: Arc<Self>,
        message: Message,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        // Deserialize task from message
        let verification_data: VerificationData =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        if verification_data.proof.len() <= self.max_proof_size && verification_data.verify() {
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

    /// Adds verification data to the current batch.
    async fn add_to_batch(
        self: Arc<Self>,
        verification_data: VerificationData,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) {
        info!("Adding verification data to batch...");
        let mut current_batch_lock = self.batch_queue.lock().await;

        current_batch_lock.push((verification_data, ws_conn_sink));

        // The data has been added to the batch, so the responded messages counter is updated
        // conn_state.update_responded_msg_count().await;

        // The connection subscribes to the processed batch channel if if was not already subscribed
        // conn_state.maybe_subscribe(self.clone()).await;
        info!("Current batch size: {}", current_batch_lock.len());
    }

    /// Given a new block number listened from the blockchain, checks if the current batch is ready to be posted.
    /// There are essentially two conditions to be checked:
    ///     * Has the current batch reached the minimum size to be posted?
    ///     * Has the received block number surpassed the maximum interval with respect to the last posted batch block?
    /// An extra sanity check is made to check if the batch size is 0, since it does not make sense to post
    /// an empty batch, even if the block interval has been reached.
    /// If the batch is ready to be submitted, a MutexGuard of it is returned so it can be processed in a thread-safe
    /// manner.
    async fn is_batch_ready(&self, block_number: u64) -> bool {
        let current_batch_lock = self.batch_queue.lock().await;
        let current_batch_size = current_batch_lock.len();
        info!("Batch size in batch_ready function: {}", current_batch_size);

        let last_uploaded_batch_block_lock = self.last_uploaded_batch_block.lock().await;

        // FIXME(marian): This condition should be changed to current_batch_size == 0
        // once the bug in Lambdaworks merkle tree is fixed.
        if current_batch_size < 2 {
            info!("Current batch is empty or size 1. Waiting for more proofs...");
            return false;
        }

        if current_batch_size < self.min_batch_len
            && block_number < *last_uploaded_batch_block_lock + self.max_block_interval
        {
            info!(
                "Current batch not ready to be posted. Current block: {} - Last uploaded block: {}. Current batch size: {} - Minimum batch size: {}",
                block_number, *last_uploaded_batch_block_lock, current_batch_size, self.min_batch_len
            );
            return false;
        }
        return true;
    }

    async fn finalize_batch(&self, block_number: u64) {
        let mut batch_queue_lock = self.batch_queue.lock().await;
        let finalized_batch = batch_queue_lock.clone();
        batch_queue_lock.clear();

        // We release the so the process listening for new proofs
        // can start queuein again
        drop(batch_queue_lock);

        let mut last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;
        let mut batch_verification_data: Vec<VerificationData> = finalized_batch
            .clone()
            .into_iter()
            .map(|(data, _)| data)
            .collect();

        let mut batch_bytes = serde_json::to_vec(batch_verification_data.as_slice())
            .expect("Failed to serialize batch");

        let batch_to_send: Vec<VerificationData>;
        if batch_bytes.len() > self.max_batch_size {
            let mut current_batch_end = 0; // not inclusive
            let mut current_batch_size = 0;
            for (i, verification_data) in batch_verification_data.iter().enumerate() {
                let verification_data_bytes = serde_json::to_vec(&verification_data)
                    .expect("Failed to serialize verification data");

                current_batch_size += verification_data_bytes.len();
                if current_batch_size > self.max_batch_size {
                    current_batch_end = i;
                    break;
                }
            }
            debug!(
                "Batch size exceeds max batch size, splitting batch at index: {}",
                current_batch_end
            );
            batch_to_send = batch_verification_data.drain(..current_batch_end).collect();

            info!(
                "Number of elements remaining for next batch: {}",
                finalized_batch.len()
            );
            batch_bytes = serde_json::to_vec(&batch_to_send).expect("Failed to serialize batch");
        }

        info!("Finalizing batch. Size: {}", finalized_batch.len());
        let batch_commitment = VerificationCommitmentBatch::from(&batch_verification_data);
        let batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
            MerkleTree::build(&batch_commitment.0);

        self.submit_batch(&batch_bytes, &batch_merkle_tree.root)
            .await;
        *last_uploaded_batch_block = block_number;

        stream::iter(finalized_batch.iter())
            .for_each(|(_, ws_sink)| async move {
                ws_sink
                    .write()
                    .await
                    .send(Message::binary(batch_merkle_tree.root.to_vec()))
                    .await
                    .unwrap();
                info!("Message sent");
            })
            .await;
    }

    /*
        async fn send_to_all<'a>(
            &self,
            current_batch: &mut MutexGuard<
                'a,
                Vec<(
                    VerificationData,
                    Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
                )>,
            >,
            batch_merkle_root: &[u8; 32],
        ) {
            let batch = current_batch.clone();
            batch.clear();

            let stream = stream::iter(current_batch.drain(..));
            info!("Sending response to all connected clients!!!!");

            // TODO(marian): We should also include de VerificationDataCommitment into the response
            stream
                .for_each(|(_, ws_sink)| async move {
                    ws_sink
                        .write()
                        .await
                        .send(Message::binary(batch_merkle_root.to_vec()))
                        .await
                        .unwrap();
                    info!("Message sent");
                })
                .await;
            info!("Won't print. Deadlock");

            // let mut ws_sinks_lock = self.peer_sink_map.lock().await;
            // let stream = stream::iter(ws_sinks_lock.drain());
            // stream
            //     .for_each(|(_, ws_sink)| async move {
            //         ws_sink
            //             .write()
            //             .await
            //             .send(Message::Binary(batch_merkle_root.to_vec()))
            //             .await
            //             .unwrap()
            //     })
            //     .await;
            // assert!(ws_sinks_lock.is_empty());
        }
    */
    /// Receives new block numbers, checks if conditions are met for submission and
    /// finalizes the batch.
    async fn handle_new_block(&self, block_number: u64) {
        if self.is_batch_ready(block_number).await {
            self.finalize_batch(block_number).await;
        }
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
