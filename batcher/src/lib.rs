extern crate core;

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_s3::client::Client as S3Client;
use bytes::Bytes;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, info};
use sha3::{Digest, Sha3_256};
use sp1_sdk::ProverClient;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

use crate::config::{BatcherConfigFromYaml, ContractDeploymentOutput};
use crate::eth::AlignedLayerServiceManager;
use crate::types::{VerificationBatch, VerificationData};

mod config;
mod eth;
pub mod s3;
pub mod types;

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

pub trait Listener {
    fn listen(&self, address: &str) -> impl Future;
}

pub struct App {
    s3_client: S3Client,
    service_manager: AlignedLayerServiceManager,
    sp1_prover_client: ProverClient,
    current_batch: Mutex<Vec<VerificationData>>,
}

// Implement the Listener trait for the App struct
impl Listener for Arc<App> {
    fn listen(&self, address: &str) -> impl Future {
        async move {
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
    }
}

impl App {
    pub async fn new(config_file: String) -> Self {
        let s3_client = s3::create_client().await;

        let config = BatcherConfigFromYaml::new(config_file);
        let deployment_output =
            ContractDeploymentOutput::new(config.aligned_layer_deployment_config_file_path);

        info!("Initializing prover client");
        let sp1_prover_client: ProverClient = ProverClient::new();
        info!("Prover client initialized");

        let service_manager = eth::get_contract(
            config.eth_rpc_url,
            config.ecdsa,
            deployment_output.addresses.aligned_layer_service_manager,
        )
        .await
        .expect("Failed to get contract");

        Self {
            s3_client,
            service_manager,
            sp1_prover_client,
            current_batch: Mutex::new(Vec::new()),
        }
    }

    pub async fn handle_connection(&self, raw_stream: TcpStream, addr: SocketAddr) {
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

    pub async fn handle_message(
        &self,
        tx: UnboundedSender<Message>,
        message: Message,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        // TODO: Handle errors

        // Deserialize task from message
        let verification_data: VerificationData =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        let proof = verification_data.proof.as_slice();
        let vm_program_code = verification_data.vm_program_code.as_ref();

        // switch on proving system
        let response = match verification_data.proving_system {
            types::ProvingSystemId::SP1 => {
                let elf = vm_program_code.expect("VM program code is required");

                let elf = elf.as_slice();

                self.verify_sp1_proof(proof, elf).await
            }
            _ => {
                error!("Unsupported proving system");
                Err(anyhow::anyhow!("Unsupported proving system"))
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

        // Close connection
        tx.close_channel();

        Ok(())
    }

    pub async fn verify_sp1_proof(&self, proof: &[u8], elf: &[u8]) -> Result<(), anyhow::Error> {
        let (_pk, vk) = self.sp1_prover_client.setup(elf);
        let proof = bincode::deserialize(proof).map_err(|_| anyhow::anyhow!("Invalid proof"))?;

        self.sp1_prover_client
            .verify(&proof, &vk)
            .map_err(|_| anyhow::anyhow!("Failed to verify proof"))?;

        Ok(())
    }

    pub async fn add_task(&self, verification_data: VerificationData) {
        info!("Adding verification data to batch...");

        let mut current_batch = self.current_batch.lock().await;
        current_batch.push(verification_data);

        debug!("Batch size: {}", current_batch.len());
        if current_batch.len() < 2 {
            return;
        }

        let batch_merkle_tree: MerkleTree<VerificationBatch> = MerkleTree::build(&current_batch);
        let batch_merkle_root_hex = hex::encode(batch_merkle_tree.root);
        info!("Batch merkle root: {}", batch_merkle_root_hex);

        let file_name = batch_merkle_root_hex + ".json";

        let batch_bytes =
            serde_json::to_vec(current_batch.as_slice()).expect("Failed to serialize batch");

        current_batch.clear();

        let s3_client = self.s3_client.clone();
        let service_manager = self.service_manager.clone();
        tokio::spawn(async move {
            info!("Uploading batch to S3...");

            s3::upload_object(&s3_client, S3_BUCKET_NAME, batch_bytes, &file_name)
                .await
                .expect("Failed to upload object to S3");

            info!("Batch sent to S3 with name: {}", file_name);
            info!("Uploading batch to contract");

            let batch_data_pointer = "https://".to_owned() + S3_BUCKET_NAME + "/" + &file_name;

            match eth::create_new_task(service_manager, batch_merkle_tree.root, batch_data_pointer)
                .await
            {
                Ok(_) => info!("Batch verification task created on Aligned contract"),
                Err(e) => error!("Failed to create batch verification task: {}", e),
            }
        });
    }
}
