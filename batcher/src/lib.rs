extern crate core;

use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use log::{debug, info};
use sp1_sdk::ProverClient;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use aws_sdk_s3::client::Client as S3Client;
use bytes::Bytes;
use sha3::{Digest, Sha3_256};
use tokio::sync::Mutex;

use crate::types::{Task, VerificationResult};

pub mod types;
pub mod s3;

pub trait Listener {
    fn listen(&self, address: &str) -> impl Future;
}

pub struct App {
    s3_client: S3Client,
    sp1_prover_client: ProverClient,
    current_batch: Mutex<Vec<Task>>,
}

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

// Implement the Listener trait for the App struct
impl Listener for Arc<App> {
    fn listen(&self, address: &str) -> impl Future {
        async move {
            // Create the event loop and TCP listener we'll accept connections on.
            let try_socket = TcpListener::bind(address).await;
            let listener = try_socket.expect("Failed to bind");
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
    pub async fn new() -> Self {
        let s3_client = s3::create_client().await;

        info!("Initializing prover client");
        let sp1_prover_client: ProverClient = ProverClient::new();
        info!("Prover client initialized");

        Self {
            s3_client,
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
        let task: Task = serde_json::from_str(message.to_text().expect("Message is not text"))
            .expect("Failed to deserialize task");

        let proof = task.proof.as_slice();
        let elf = task.public_input.as_slice();

        // Deserialize proof from task
        let proof = bincode::deserialize(proof).expect("Failed to deserialize proof");

        info!("Verifying proof");
        let (_pk, vk) = self.sp1_prover_client.setup(elf);
        let is_valid = self.sp1_prover_client.verify(&proof, &vk).is_ok();
        info!("Proof verification result: {}", is_valid);

        let response = if is_valid {
            let task_bytes = bincode::serialize(&task)
                .expect("Failed to bincode serialize task");

            let task_bytes = Bytes::from(task_bytes);

            let mut hasher = Sha3_256::new();
            hasher.update(&task_bytes);
            let hash = hasher.finalize().to_vec();

            self.add_task(task).await;

            serde_json::to_string(&VerificationResult::Success {
                hash,
            })
            .expect("Failed to serialize response")


        } else {
            serde_json::to_string(&VerificationResult::Failure).expect("Failed to serialize response")
        };

        tx.unbounded_send(Message::Text(response))
            .expect("Failed to send message");

        // Close connection
        tx.close_channel();

        Ok(())
    }

    pub async fn add_task(&self, task: Task) {
        debug!("Adding task to batch");

        let mut current_batch = self.current_batch.lock().await;
        current_batch.push(task);

        debug!("Batch size: {}", current_batch.len());
        if current_batch.len() < 2 {
            return;
        }

        let batch_bytes = bincode::serialize(current_batch.as_slice())
            .expect("Failed to bincode serialize batch");

        current_batch.clear();

        let s3_client = self.s3_client.clone();
        tokio::spawn(async move {
            info!("Sending batch to s3");
            let mut hasher = Sha3_256::new();
            hasher.update(&batch_bytes);
            let hash = hasher.finalize().to_vec();

            let hex_hash = hex::encode(hash.as_slice());

            let batch_bytes = Bytes::from(batch_bytes);

            s3::upload_object(&s3_client, S3_BUCKET_NAME, batch_bytes, &hex_hash)
                .await.expect("Failed to upload object to S3");

            info!("Batch sent to S3 with name: {}", hex_hash);
        });
    }
}
