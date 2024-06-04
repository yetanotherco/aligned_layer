extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_s3::client::Client as S3Client;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use futures_util::{future, pin_mut, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::Message;
use types::VerificationCommitmentBatch;

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};
use crate::eth::AlignedLayerServiceManager;
use crate::types::VerificationData;

mod config;
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
    current_batch: Mutex<Vec<VerificationData>>,
    max_block_interval: u64,
    min_batch_size: usize,
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

        let eth_ws_provider = Provider::connect_with_reconnects(
            &config.eth_ws_url, config.batcher.eth_ws_reconnects)
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
            current_batch: Mutex::new(Vec::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_size: config.batcher.batch_size_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
        }
    }

    pub async fn listen_connections(self: Arc<Self>, address: &str, tx: Arc<Sender<Message>>) {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address).await.expect("Failed to build");
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let batcher = self.clone();
            let rx = tx.subscribe();

            tokio::spawn(batcher.handle_connection(stream, addr, rx));
        }
    }

    pub async fn listen_new_blocks(
        self: Arc<Self>,
        tx: Arc<Sender<Message>>,
    ) -> Result<(), anyhow::Error> {
        let mut stream = self.eth_ws_provider.subscribe_blocks().await?;

        while let Some(block) = stream.next().await {
            let batcher = self.clone();
            let tx = tx.clone();
            info!("Received new block");
            tokio::spawn(async move {
                let block_number = block.number.unwrap();
                let block_number = u64::try_from(block_number).unwrap();
                batcher.handle_new_block(block_number, tx).await;
            });
        }

        Ok(())
    }

    async fn handle_connection(
        self: Arc<Self>,
        raw_stream: TcpStream,
        addr: SocketAddr,
        rx: Receiver<Message>,
    ) {
        info!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");

        debug!("WebSocket connection established: {}", addr);
        let (mut outgoing, incoming) = ws_stream.split();

        let get_incoming = incoming
            .try_filter(|msg| future::ready(msg.is_text()))
            .try_for_each(|msg| self.clone().handle_message(msg));

        let mut rx = rx;
        let send_outgoing = async {
            let msg = rx.recv().await.unwrap();
            outgoing.send(msg).await.unwrap();
            outgoing.close().await
        };

        pin_mut!(get_incoming, send_outgoing);
        future::select(get_incoming, send_outgoing).await;

        info!("{} disconnected", &addr);
    }

    async fn handle_message(
        self: Arc<Self>,
        message: Message,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        // Deserialize task from message
        let verification_data: VerificationData =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        if verification_data.proof.len() <= self.max_proof_size && verification_data.verify() {
            self.add_to_batch(verification_data).await;
        } else {
            // FIXME(marian): Handle this error correctly
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                ProtocolError::HandshakeIncomplete,
            ));
        };

        info!("Verification data message handled");

        Ok(())
    }

    async fn add_to_batch(self: Arc<Self>, verification_data: VerificationData) {
        info!("Adding verification data to batch...");
        let mut current_batch = self.current_batch.lock().await;
        current_batch.push(verification_data);
        info!("Current batch size: {}", current_batch.len());
    }

    /// Given a new block number listened from the blockchain, checks if the current batch is ready to be posted.
    /// There are essentially two conditions to be checked:
    ///     * Has the current batch reached the minimum size to be posted?
    ///     * Has the received block number surpassed the maximum interval with respect to the last posted batch block?
    /// An extra sanity check is made to check if the batch size is 0, since it does not make sense to post
    /// an empty batch, even if the block interval has been reached.
    async fn batch_ready(&self, block_number: u64) -> bool {
        let current_batch_size = self.current_batch.lock().await.len();
        // FIXME(marian): This condition should be changed to current_batch_size == 0
        // once the bug in Lambdaworks merkle tree is fixed.
        if current_batch_size < 2 {
            info!("Current batch is empty or size 1. Waiting for more proofs...");
            return false;
        }

        let last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;
        if current_batch_size < self.min_batch_size
            && block_number < *last_uploaded_batch_block + self.max_block_interval
        {
            info!(
                "Current batch not ready to be posted. Current block: {} - Last uploaded block: {}. Current batch size: {} - Minimum batch size: {}",
                block_number, *last_uploaded_batch_block, current_batch_size, self.min_batch_size
            );
            return false;
        }

        true
    }

    async fn process_batch_and_update_state(&self, block_number: u64) -> (Vec<u8>, [u8; 32]) {
        let mut current_batch = self.current_batch.lock().await;

        let mut batch_bytes =
            serde_json::to_vec(current_batch.as_slice()).expect("Failed to serialize batch");

        let batch_to_send;
        if batch_bytes.len() > self.max_batch_size {
            let mut current_batch_end = 0; // not inclusive
            let mut current_batch_size = 0;
            for (i, verification_data) in current_batch.iter().enumerate() {
                let verification_data_bytes = serde_json::to_vec(verification_data)
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
            batch_to_send = current_batch.drain(..current_batch_end).collect::<Vec<_>>();

            info!(
                "# of Elements remaining for next batch: {}",
                current_batch.len()
            );
            batch_bytes = serde_json::to_vec(&batch_to_send).expect("Failed to serialize batch");
        } else {
            batch_to_send = current_batch.clone();
            current_batch.clear();
        }

        let batch_commitment = VerificationCommitmentBatch::from(&batch_to_send);
        let batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
            MerkleTree::build(&batch_commitment.0);

        // update batcher state (update last uploaded batch block)
        *self.last_uploaded_batch_block.lock().await = block_number;

        (batch_bytes, batch_merkle_tree.root)
    }

    async fn handle_new_block(&self, block_number: u64, tx: Arc<Sender<Message>>) {
        if !self.batch_ready(block_number).await {
            return;
        }

        let (batch_bytes, batch_merkle_root) =
            self.process_batch_and_update_state(block_number).await;

        let s3_client = self.s3_client.clone();
        let service_manager = self.service_manager.clone();
        let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        info!("Batch merkle root: {}", batch_merkle_root_hex);

        let file_name = batch_merkle_root_hex.clone() + ".json";

        info!("Uploading batch to S3...");

        s3::upload_object(&s3_client, S3_BUCKET_NAME, batch_bytes, &file_name)
            .await
            .expect("Failed to upload object to S3");

        info!("Batch sent to S3 with name: {}", file_name);
        info!("Uploading batch to contract");

        let batch_data_pointer = "https://".to_owned() + S3_BUCKET_NAME + "/" + &file_name;
        match eth::create_new_task(service_manager, batch_merkle_root, batch_data_pointer).await {
            Ok(_) => info!("Batch verification task created on Aligned contract"),
            Err(e) => error!("Failed to create batch verification task: {}", e),
        }
        tx.send(Message::Text(batch_merkle_root_hex))
            .expect("Could not send response");
    }
}
