use std::{io::Error as IoError, net::SocketAddr};
use std::sync::Arc;

use clap::Parser;
use env_logger::Env;
use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use log::info;
use sp1_sdk::ProverClient;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;

use batcher::{Task, VerificationResult};

#[derive(Parser)]
#[command(name = "Aligned Layer Batcher")]
#[command(about = "An application with server and client subcommands", long_about = None)]
struct Cli {
    #[arg(short, long)]
    port: Option<u16>,
}

async fn handle_connection(
    prover_client: Arc<ProverClient>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    info!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {}", addr);

    let (tx, rx) = unbounded();
    let (outgoing, incoming) = ws_stream.split();

    let get_incoming = incoming
        .try_filter(|msg| future::ready(msg.is_text()))
        .try_for_each(|msg| {
            info!("Received a message from {}", addr,);
            // tx.unbounded_send(msg.clone())
            //     .expect("Failed to send message");

            // verify proof
            let task: Task = serde_json::from_str(msg.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

            let proof = task.proof;
            let elf = task.public_input.as_slice();

            let proof =
                bincode::deserialize(proof.as_slice()).expect("Failed to deserialize proof");

            let (_pk, vk) = prover_client.setup(elf);
            let is_valid = prover_client.verify(&proof, &vk).is_ok();
            info!("Proof verification result: {}", is_valid);

            let response = if is_valid {
                serde_json::to_string(&VerificationResult::Success {
                    hash: vec![1, 2, 3, 4],
                })
                .expect("Failed to serialize response")
            } else {
                serde_json::to_string(&VerificationResult::Failure)
                    .expect("Failed to serialize response")
            };

            tx.unbounded_send(tungstenite::Message::Text(response))
                .expect("Failed to send message");

            // Close channel after response
            tx.close_channel();

            future::ok(())
        });

    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(get_incoming, receive_from_others);
    future::select(get_incoming, receive_from_others).await;

    info!("{} disconnected", &addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Initializing prover client");
    let prover_client: ProverClient = ProverClient::new();

    let prover_client = Arc::new(prover_client);

    info!("Prover client initialized");

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(8080);

    let addr = format!("localhost:{}", port);

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(prover_client.clone(), stream, addr));
    }

    Ok(())
}
