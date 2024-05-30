extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_s3::client::Client as S3Client;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use futures_util::{future, pin_mut, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::{Mutex, Notify};
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::Message;
use types::VerificationCommitmentBatch;

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};
use crate::eth::AlignedLayerServiceManager;
use crate::types::VerificationData;

mod config;
mod eth;
pub mod gnark;
pub mod s3;
pub mod types;

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

pub struct Batcher {
    s3_client: S3Client,
    eth_ws_provider: Provider<Ws>,
    service_manager: AlignedLayerServiceManager,
    current_batch: Mutex<Vec<VerificationData>>,
    max_block_interval: u64,
    min_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
    broadcast_tx: Mutex<Sender<Message>>,
}

struct ConnectionState {
    inner: Mutex<Option<Receiver<Message>>>,
    received_msgs: Mutex<usize>,
    responded_msgs: Mutex<usize>,
}

impl ConnectionState {
    pub fn new() -> Self {
        ConnectionState {
            inner: Mutex::new(None),
            received_msgs: Mutex::new(0),
            responded_msgs: Mutex::new(0),
        }
    }
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

        let (broadcast_tx, _) = broadcast::channel::<Message>(10);
        let broadcast_tx = Mutex::new(broadcast_tx);

        Self {
            s3_client,
            eth_ws_provider,
            service_manager,
            current_batch: Mutex::new(Vec::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_size: config.batcher.batch_size_interval,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
            broadcast_tx,
        }
    }

    pub async fn listen_connections(self: Arc<Self>, address: &str) {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address).await.expect("Failed to build");
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let batcher = self.clone();
            // let rx = tx.subscribe();

            tokio::spawn(batcher.handle_connection(stream, addr));
        }
    }

    pub async fn listen_new_blocks(self: Arc<Self>) -> Result<(), anyhow::Error> {
        let mut stream = self.eth_ws_provider.subscribe_blocks().await?;
        while let Some(block) = stream.next().await {
            info!("Received new block");
            let batcher = self.clone();
            // let tx = tx.clone();
            let block_number = block.number.unwrap();
            let block_number = u64::try_from(block_number).unwrap();
            tokio::spawn(async move {
                batcher.handle_new_block(block_number).await;
            });
        }

        Ok(())
    }

    async fn handle_connection(
        self: Arc<Self>,
        raw_stream: TcpStream,
        addr: SocketAddr,
        // _rx: Receiver<Message>,
    ) {
        info!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");

        debug!("WebSocket connection established: {}", addr);
        let (mut outgoing, incoming) = ws_stream.split();

        let conn_state = Arc::new(ConnectionState::new());
        let notify = Arc::new(Notify::new());

        let get_incoming = incoming
            .try_filter(|msg| future::ready(msg.is_text()))
            .try_for_each(|msg| {
                self.clone()
                    .handle_message(msg, conn_state.clone(), notify.clone())
            });

        let send_outgoing = async {
            loop {
                let mut rx_lock = conn_state.inner.lock().await;
                if let Some(rx) = &mut (*rx_lock) {
                    // if let Some(rx) = &mut rx_ {
                    let msg = rx.recv().await.unwrap();
                    outgoing.send(msg).await.unwrap();

                    // reset the receiver
                    *rx_lock = None;

                    if *conn_state.responded_msgs.lock().await
                        == *conn_state.received_msgs.lock().await
                    {
                        outgoing.close().await.unwrap();
                    }
                }
            }
        };

        pin_mut!(get_incoming, send_outgoing);
        future::select(get_incoming, send_outgoing).await;

        info!("{} disconnected", &addr);
    }

    async fn handle_message(
        self: Arc<Self>,
        message: Message,
        conn_state: Arc<ConnectionState>,
        notifier: Arc<Notify>,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        {
            *conn_state.received_msgs.lock().await += 1
        }

        // Deserialize task from message
        let verification_data: VerificationData =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        if verification_data.verify() {
            self.add_to_batch(verification_data, conn_state, notifier)
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

    async fn add_to_batch(
        self: Arc<Self>,
        verification_data: VerificationData,
        conn_state: Arc<ConnectionState>,
        notifier: Arc<Notify>,
    ) {
        info!("Adding verification data to batch...");
        let mut current_batch = self.current_batch.lock().await;
        current_batch.push(verification_data);

        let mut responded_msgs_lock = conn_state.responded_msgs.lock().await;
        *responded_msgs_lock += 1;

        if let Ok(mut rx) = conn_state.inner.try_lock() {
            if rx.is_none() {
                *rx = Some(self.broadcast_tx.lock().await.subscribe());
                notifier.notify_one();
            }
        }
        info!("Current batch size: {}", current_batch.len());
    }

    /// Given a new block number listened from the blockchain, checks if the current batch is ready to be posted.
    /// There are essentially two conditions to be checked:
    ///     * Has the current batch reached the minimum size to be posted?
    ///     * Has the received block number surpassed the maximum interval with respect to the last posted batch block?
    /// An extra sanity check is made to check if the batch size is 0, since it does not make sense to post
    /// an empty batch, even if the block interval has been reached.
    async fn batch_ready(&self, block_number: u64) -> bool {
        let current_batch_lock = self.current_batch.lock().await;
        let current_batch_size = current_batch_lock.len();

        let last_uploaded_batch_block_lock = self.last_uploaded_batch_block.lock().await;

        // FIXME(marian): This condition should be changed to current_batch_size == 0
        // once the bug in Lambdaworks merkle tree is fixed.
        if current_batch_size < 2 {
            info!("Current batch is empty or size 1. Waiting for more proofs...");
            return false;
        }

        if current_batch_size < self.min_batch_size
            && block_number < *last_uploaded_batch_block_lock + self.max_block_interval
        {
            info!(
                "Current batch not ready to be posted. Current block: {} - Last uploaded block: {}. Current batch size: {} - Minimum batch size: {}",
                block_number, *last_uploaded_batch_block_lock, current_batch_size, self.min_batch_size
            );
            return false;
        }

        true
    }

    async fn process_batch_and_update_state(&self, block_number: u64) -> (Vec<u8>, [u8; 32]) {
        let mut current_batch = self.current_batch.lock().await;
        let mut broadcast_tx = self.broadcast_tx.lock().await;
        let mut last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;

        info!("Finalizing batch. Size: {}", current_batch.len());

        let batch_commitment = VerificationCommitmentBatch::from(&(*current_batch));
        let batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
            MerkleTree::build(&batch_commitment.0);
        let batch_bytes =
            serde_json::to_vec(current_batch.as_slice()).expect("Failed to serialize batch");

        self.submit_batch(&batch_bytes, &batch_merkle_tree.root)
            .await;

        // The only possible way this can fail is when there are no subscribed receivers left,
        // so we just log a warning and continue
        if broadcast_tx
            .send(Message::Binary(batch_merkle_tree.root.to_vec()))
            .is_err()
        {
            warn!("No connections awaiting for anwsers. Reseting batch state and continuing...");
        }

        // update batcher state (clear current batch and update last uploaded batch block)
        current_batch.clear();
        *last_uploaded_batch_block = block_number;
        let (new_broadcast_tx, _) = broadcast::channel(10);
        *broadcast_tx = new_broadcast_tx;

        (batch_bytes, batch_merkle_tree.root)
    }

    async fn handle_new_block(&self, block_number: u64) {
        if !self.batch_ready(block_number).await {
            return;
        }

        let (batch_bytes, batch_merkle_root) =
            self.process_batch_and_update_state(block_number).await;

        // let s3_client = self.s3_client.clone();
        // let service_manager = self.service_manager.clone();
        // let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        // info!("Batch merkle root: {}", batch_merkle_root_hex);

        // let file_name = batch_merkle_root_hex.clone() + ".json";

        // info!("Uploading batch to S3...");

        // s3::upload_object(&s3_client, S3_BUCKET_NAME, batch_bytes, &file_name)
        //     .await
        //     .expect("Failed to upload object to S3");

        // info!("Batch sent to S3 with name: {}", file_name);
        // info!("Uploading batch to contract");

        // let batch_data_pointer = "https://".to_owned() + S3_BUCKET_NAME + "/" + &file_name;
        // match eth::create_new_task(service_manager, batch_merkle_root, batch_data_pointer).await {
        //     Ok(_) => info!("Batch verification task created on Aligned contract"),
        //     Err(e) => error!("Failed to create batch verification task: {}", e),
        // }
        // tx.send(Message::Text(batch_merkle_root_hex))
        //     .expect("Could not send response");
    }

    /// Post to batch to s3 and submit new task to Ethereum
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
