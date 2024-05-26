extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{self, Duration};

use aws_sdk_s3::client::Client as S3Client;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, SinkExt, StreamExt, TryFutureExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::Message;
use types::{ProvingSystemId, VerificationCommitmentBatch};

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};
use crate::eth::AlignedLayerServiceManager;
use crate::types::VerificationData;

mod config;
mod eth;
pub mod s3;
pub mod types;

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

pub struct App {
    s3_client: S3Client,
    eth_ws_provider: Provider<Ws>,
    service_manager: AlignedLayerServiceManager,
    // eth_ws_url: String,
    current_batch: Mutex<Vec<VerificationData>>,
    max_block_interval: u64,
    min_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
}

impl App {
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
            // eth_ws_url: config.eth_ws_url,
            current_batch: Mutex::new(Vec::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_size: config.batcher.batch_size_interval,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
        }
    }

    pub async fn listen_connections(self: &Arc<Self>, address: &str, tx: Arc<Sender<Message>>) {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address).await.expect("Failed to build");
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let c = self.clone();
            let mut rx = tx.subscribe();

            tokio::spawn(async move {
                c.handle_connection(stream, addr, &mut rx).await;
            });
        }
    }

    pub async fn listen_new_blocks(
        self: &Arc<Self>,
        tx: Arc<Sender<Message>>,
    ) -> Result<(), anyhow::Error> {
        // eth::poll_new_blocks(self.eth_ws_url.clone(), |block_number| async move {
        //     self.handle_new_block(block_number).await
        // })
        // .await
        // .expect("Failed to poll new blocks");
        // let provider = Provider::<Ws>::connect(&self.eth_ws_url).await?;
        let mut stream = self.eth_ws_provider.subscribe_blocks().await?;
        while let Some(block) = stream.next().await {
            let batcher = self.clone();
            let tx = tx.clone();
            info!("New block received");
            tokio::spawn(async move {
                let block_number = block.number.unwrap();
                let block_number = u64::try_from(block_number)
                    .map_err(|err: &str| anyhow::anyhow!(err))
                    .unwrap();
                batcher.handle_new_block(block_number, tx).await;
            });
        }

        Ok(())
    }

    async fn handle_connection(
        self: &Arc<Self>,
        raw_stream: TcpStream,
        addr: SocketAddr,
        rx: &mut Receiver<Message>,
    ) {
        info!("Incoming TCP connection from: {}", addr);

        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");
        debug!("WebSocket connection established: {}", addr);

        // let (tx, rx) = unbounded();
        let (mut outgoing, incoming) = ws_stream.split();

        let get_incoming = incoming
            .try_filter(|msg| future::ready(msg.is_text()))
            .try_for_each(|msg| async move { self.handle_message(msg).await });

        // let send_outgoing = rx.map(Ok).forward(outgoing);
        // let send_outgoing = rx.recv().map(f);
        info!("hola");
        // let send_outgoing = rx.recv().and_then(|msg| outgoing.send(msg));
        let send_outgoing = rx.recv().map_ok(|msg| outgoing.send(msg));
        info!("chau");

        pin_mut!(get_incoming, send_outgoing);
        future::select(get_incoming, send_outgoing).await;

        info!("{} disconnected", &addr);
    }

    async fn handle_message(
        self: &Arc<Self>,
        // tx: UnboundedSender<Message>,
        message: Message,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        // TODO: Handle errors

        // Deserialize task from message
        let verification_data: VerificationData =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        verification_data.verify();

        if verification_data.verify() {
            self.add_task(verification_data).await;
        } else if verification_data.proving_system == ProvingSystemId::Groth16Bn254 {
            self.add_task(verification_data).await;
        } else {
            // FIXME(marian): Handle this error correctly
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                ProtocolError::HandshakeIncomplete,
            ));
        };

        info!("Verification data message handled");

        // let response = serde_json::to_string(&response).expect("Failed to serialize response");
        // tx.unbounded_send(Message::Text(response))
        //     .expect("Failed to send message");

        Ok(())
    }

    async fn add_task(self: &Arc<Self>, verification_data: VerificationData) {
        info!("Adding verification data to batch...");
        let mut current_batch = self.current_batch.lock().await;
        current_batch.push(verification_data);
        info!("Current batch size: {}", current_batch.len());
        // if len >= self.batch_size_interval {
        //     let c = self.clone();
        //     tokio::spawn(async move {
        //         let block_number = c
        //             .eth_rpc_provider
        //             .get_block_number()
        //             .await
        //             .expect("Failed to get block number");
        //         let block_number =
        //             u64::try_from(block_number).expect("Failed to convert block number");
        //         c.handle_new_block(block_number).await;
        //     });
        // }
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
        let batch_commitment = VerificationCommitmentBatch::from(&(*current_batch));
        let batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
            MerkleTree::build(&batch_commitment.0);
        let batch_bytes =
            serde_json::to_vec(current_batch.as_slice()).expect("Failed to serialize batch");

        // update batcher state (clear current batch buffer and update last uploaded batch block)
        current_batch.clear();
        *self.last_uploaded_batch_block.lock().await = block_number;

        (batch_bytes, batch_merkle_tree.root)
    }

    async fn handle_new_block(&self, block_number: u64, tx: Arc<Sender<Message>>) {
        // let (batch_bytes, batch_merkle_root) = {
        //     let mut last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;
        //     let mut current_batch = self.current_batch.lock().await;
        //     let current_batch_len = current_batch.len();
        //     if current_batch_len <= 1 {
        //         // Needed because merkle tree freezes on only one leaf
        //         debug!("New block reached but current batch is empty or has only one proof. Waiting for more proofs...");
        //         return;
        //     }

        // check if neither interval is reached
        // if current_batch.len() < self.min_batch_size
        //     && block_number < *last_uploaded_batch_block + self.max_block_interval
        // {
        //     info!(
        //         "Block interval not reached, current block: {}, last uploaded block: {}",
        //         block_number, *last_uploaded_batch_block
        //     );
        //     return;
        // }

        if !self.batch_ready(block_number).await {
            return;
        }
        let (batch_bytes, batch_merkle_root) =
            self.process_batch_and_update_state(block_number).await;

        // If this condition is met then the batch is ready
        // let current_batch = self.current_batch.lock().await;
        // let batch_commitment = VerificationCommitmentBatch::from(&(*current_batch));
        // let batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
        //     MerkleTree::build(&batch_commitment.0);
        // let batch_bytes =
        //     serde_json::to_vec(current_batch.as_slice()).expect("Failed to serialize batch");
        // batch_merkle_tree.root;

        // // update batcher state (clear current batch buffer and update last uploaded batch block)
        // current_batch.clear();
        // *self.last_uploaded_batch_block.lock().await = block_number;

        //     (batch_bytes, batch_merkle_tree.root)
        // }; // lock is released here so new proofs can be added

        let s3_client = self.s3_client.clone();
        let service_manager = self.service_manager.clone();
        // tokio::spawn(async move {
        let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        info!("Batch merkle root: {}", batch_merkle_root_hex);

        let file_name = batch_merkle_root_hex + ".json";

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
        tx.send(Message::Binary(batch_merkle_root.to_vec()))
            .expect("Could not send response");
        // }
    }
}
