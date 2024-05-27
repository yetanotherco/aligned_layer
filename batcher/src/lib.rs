extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_s3::client::Client as S3Client;
use bytes::Bytes;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Http;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, warn, info};
use sha3::{Digest, Sha3_256};
use sp1_sdk::ProverClient;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};
use crate::eth::AlignedLayerServiceManager;
use crate::types::{VerificationBatch, VerificationData};

mod config;
mod eth;
pub mod s3;
pub mod types;

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

pub struct App {
    s3_client: S3Client,
    eth_rpc_provider: Provider<Http>,
    service_manager: AlignedLayerServiceManager,
    sp1_prover_client: ProverClient,
    eth_ws_url: String,
    current_batch: Mutex<Vec<VerificationData>>,
    block_interval: u64,
    batch_size_interval: usize,
    max_proof_size: usize,
    max_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
}

impl App {
    pub async fn new(config_file: String) -> Self {
        let s3_client = s3::create_client().await;

        let config = ConfigFromYaml::new(config_file);
        let deployment_output =
            ContractDeploymentOutput::new(config.aligned_layer_deployment_config_file_path);

        info!("Initializing prover client");
        let sp1_prover_client: ProverClient = ProverClient::new();
        info!("Prover client initialized");

        let eth_rpc_provider =
            eth::get_provider(config.eth_rpc_url.clone()).expect("Failed to get provider");

        let service_manager = eth::get_contract(
            eth_rpc_provider.clone(),
            config.ecdsa,
            deployment_output.addresses.aligned_layer_service_manager,
        )
        .await
        .expect("Failed to get contract");

        Self {
            s3_client,
            eth_rpc_provider,
            service_manager,
            sp1_prover_client,
            eth_ws_url: config.eth_ws_url,
            current_batch: Mutex::new(Vec::new()),
            block_interval: config.batcher.block_interval,
            batch_size_interval: config.batcher.batch_size_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(0),
        }
    }

    pub async fn listen(self: &Arc<Self>, address: &str) {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address).await.expect("Failed to build");
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            let c = self.clone();

            tokio::spawn(async move {
                c.handle_connection(stream, addr).await;
            });
        }
    }

    pub async fn poll_new_blocks(&self) {
        eth::poll_new_blocks(self.eth_ws_url.clone(), |block_number| async move {
            self.handle_new_block(block_number).await
        })
        .await
        .expect("Failed to poll new blocks");
    }

    async fn handle_connection(self: &Arc<Self>, raw_stream: TcpStream, addr: SocketAddr) {
        info!("Incoming TCP connection from: {}", addr);

        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");
        debug!("WebSocket connection established: {}", addr);

        let (tx, rx) = unbounded();
        let (outgoing, incoming) = ws_stream.split();

        let get_incoming = incoming
            .try_filter(|msg| future::ready(msg.is_text()))
            .try_for_each(|msg| {
                let tx = tx.clone();

                async move { self.handle_message(tx, msg).await }
            });

        let receive_from_others = rx.map(Ok).forward(outgoing);
        pin_mut!(get_incoming, receive_from_others);
        future::select(get_incoming, receive_from_others).await;

        info!("{} disconnected", &addr);
    }

    async fn handle_message(
        self: &Arc<Self>,
        tx: UnboundedSender<Message>,
        message: Message,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        // TODO: Handle errors
        /* TODO: response could be handling better by returning Ok / Error from in here
        and then sending the response from the caller */
        
        // Deserialize task from message
        let verification_data: VerificationData =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        let proof = verification_data.proof.as_slice();
        if proof.len() > self.max_proof_size {
            let response: Result<(), _> = Err("Proof size is too large".to_string());
            let response = serde_json::to_string(&response).expect("Failed to serialize response");

            tx.unbounded_send(Message::Text(response))
                .expect("Failed to send message");

            return Ok(());
        }
        
        let vm_program_code = verification_data.vm_program_code.as_ref();

        let response = match verification_data.proving_system {
            types::ProvingSystemId::SP1 => {
                let elf = vm_program_code.expect("VM program code is required");

                let elf = elf.as_slice();

                self.verify_sp1_proof(proof, elf)
            }
            _ => {
                warn!("Unsupported proving system, proof not verified");
                Ok(())
            }
        };

        let response = match response {
            Ok(_) => {
                let task_bytes = bincode::serialize(&verification_data)
                    .expect("Failed to bincode serialize task");

                self.add_task(verification_data).await;

                let task_bytes = Bytes::from(task_bytes);
                let mut hasher = Sha3_256::new();
                hasher.update(&task_bytes);
                let hash = hasher.finalize().to_vec();

                Ok(hash)
            }
            Err(e) => Err(e.to_string()),
        };

        let response = serde_json::to_string(&response).expect("Failed to serialize response");

        tx.unbounded_send(Message::Text(response))
            .expect("Failed to send message");

        Ok(())
    }

    async fn add_task(self: &Arc<Self>, verification_data: VerificationData) {
        info!("Adding verification data to batch...");

        let len = {
            let mut current_batch = self.current_batch.lock().await;
            current_batch.push(verification_data);

            debug!("Batch length: {}", current_batch.len());
            current_batch.len()
        };

        if len >= self.batch_size_interval {
            let c = self.clone();
            tokio::spawn(async move {
                let block_number = c
                    .eth_rpc_provider
                    .get_block_number()
                    .await
                    .expect("Failed to get block number");
                let block_number =
                    u64::try_from(block_number).expect("Failed to convert block number");
                c.handle_new_block(block_number).await;
            });
        }
    }

    async fn handle_new_block(&self, block_number: u64) {
        let (batch_bytes, batch_merkle_root) = {
            let mut last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;
            let mut current_batch = self.current_batch.lock().await;
            let current_batch_len = current_batch.len();
            if current_batch_len <= 1 {
                // Needed because merkle tree freezes on only one leaf
                debug!("New block reached but current batch is empty or has only one proof. Waiting for more proofs...");
                return;
            }

            // check if neither interval is reached
            if current_batch.len() < self.batch_size_interval
                && block_number < *last_uploaded_batch_block + self.block_interval
            {
                info!(
                    "Block interval not reached, current block: {}, last uploaded block: {}",
                    block_number, *last_uploaded_batch_block
                );
                return;
            }

            let mut batch_bytes =
                serde_json::to_vec(current_batch.as_slice()).expect("Failed to serialize batch");

            let batch_to_send;
            if batch_bytes.len() > self.max_batch_size {
                let mut current_batch_end = 0;
                let mut current_batch_size = 0;
                for (i, verification_data) in current_batch.iter().enumerate() {
                    let verification_data_bytes = serde_json::to_vec(verification_data)
                        .expect("Failed to serialize verification data");

                    debug!("Current batch size: {}, Verification data size: {}", current_batch_size,
                        verification_data_bytes.len());
                    current_batch_size += verification_data_bytes.len();
                    if current_batch_size > self.max_batch_size {
                        current_batch_end = i;
                        break;
                    }
                }

                debug!("Batch size exceeds max batch size, splitting batch at index: {}", current_batch_end);
                batch_to_send = current_batch.drain(..current_batch_end)
                    .collect::<Vec<_>>();

                debug!("Batch size after splitting: {}", batch_to_send.len());
                debug!("# of Elements remaining: {}", current_batch.len());
                
                batch_bytes = serde_json::to_vec(&batch_to_send)
                    .expect("Failed to serialize batch");
            } else {
                batch_to_send = current_batch.clone();
                current_batch.clear();
            }

            let batch_merkle_tree: MerkleTree<VerificationBatch> =
                MerkleTree::build(&batch_to_send);

            *last_uploaded_batch_block = block_number;
            (batch_bytes, batch_merkle_tree.root)
        }; // lock is released here so new proofs can be added

        let s3_client = self.s3_client.clone();
        let service_manager = self.service_manager.clone();
        tokio::spawn(async move {
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

            match eth::create_new_task(service_manager, batch_merkle_root, batch_data_pointer).await
            {
                Ok(_) => info!("Batch verification task created on Aligned contract"),
                Err(e) => error!("Failed to create batch verification task: {}", e),
            }
        });
    }

    fn verify_sp1_proof(&self, proof: &[u8], elf: &[u8]) -> Result<(), anyhow::Error> {
        let (_pk, vk) = self.sp1_prover_client.setup(elf);
        let proof = bincode::deserialize(proof).map_err(|_| anyhow::anyhow!("Invalid proof"))?;

        self.sp1_prover_client
            .verify(&proof, &vk)
            .map_err(|_| anyhow::anyhow!("Failed to verify proof"))?;

        Ok(())
    }
}
