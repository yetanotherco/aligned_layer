use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, StreamExt, TryStreamExt};
use log::{debug, info};
use sp1_sdk::ProverClient;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

use crate::types::{Task, VerificationResult};

pub mod types;

pub trait Listener {
    fn listen(&self, address: &str) -> impl Future;
}

pub struct App {
    sp1_prover_client: ProverClient,
}

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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        info!("Initializing prover client");
        let prover_client: ProverClient = ProverClient::new();
        info!("Prover client initialized");

        Self {
            sp1_prover_client: prover_client,
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
            let hash = task.hash();

            serde_json::to_string(&VerificationResult::Success {
                hash,
            })
            .expect("Failed to serialize response")
        } else {
            serde_json::to_string(&VerificationResult::Failure)
                .expect("Failed to serialize response")
        };

        tx.unbounded_send(Message::Text(response))
            .expect("Failed to send message");

        // Close channel after response
        tx.close_channel();

        Ok(())
    }
}
