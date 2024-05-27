extern crate dotenv;

use std::io::Error as IoError;
use std::sync::Arc;

use clap::Parser;
use env_logger::Env;

use batcher::Batcher;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

/// Batcher main flow:
/// There are two main tasks spawed: `listen_connections` and `listen_new_blocks`
/// * `listen_connections` waits for websocket connections and adds verification data sent by clients
///    to the batch.
/// * `listen_new_blocks` waits for new blocks and when one is received, checks if the conditions are met
///    the current batch to be submitted. In other words, this task is the one that controls when a batch
///    is to be posted.
#[derive(Parser)]
#[command(name = "Aligned Layer Batcher")]
#[command(about = "An application with server and client subcommands", long_about = None)]
struct Cli {
    #[arg(short, long)]
    config: String,
    #[arg(short, long)]
    env_file: Option<String>,
    #[arg(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(8080);

    match cli.env_file {
        Some(env_file) => dotenv::from_filename(env_file).ok(),
        None => dotenv::dotenv().ok(),
    };

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let batcher = Batcher::new(cli.config).await;
    let batcher = Arc::new(batcher);

    let addr = format!("localhost:{}", port);

    // A broadcast channel transmitter is created so that when the block listener checks that a batch
    // is ready to be submitted, the information about the merkle root of the batch is transmitted to
    // the the websocket connections to respond to clients.
    let (tx, _) = broadcast::channel::<Message>(10);
    let tx = Arc::new(tx);

    // `connections_tx` is passed to the  connections listener task so that each new
    // connections subscribes to this transmitter and waits to receive the batch merkle root
    // once it is processed
    let connections_tx = tx.clone();

    // `blocks_tx` is passed to the blocks listener so that when a batch is processed, its
    // merkle root is transmitted to the websocket connections
    let blocks_tx = tx.clone();

    // spawn thread listening to blocks
    tokio::spawn({
        let app = batcher.clone();
        async move {
            app.listen_new_blocks(blocks_tx).await.unwrap();
        }
    });

    batcher.listen_connections(&addr, connections_tx).await;

    Ok(())
}
