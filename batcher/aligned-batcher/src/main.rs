extern crate dotenvy;

use std::sync::Arc;

use clap::Parser;
use env_logger::Env;

use aligned_batcher::{types::errors::BatcherError, Batcher};

/// Batcher main flow:
/// There are two main tasks spawned: `listen_connections` and `listen_new_blocks`
/// * `listen_connections` waits for websocket connections and adds verification data sent by clients
///    to the batch.
/// * `listen_new_blocks` waits for new blocks and when one is received, checks if the conditions are met
///    the current batch to be submitted. In other words, this task is the one that controls when a batch
///    is to be posted.
#[derive(Parser)]
#[command(name = "Aligned Batcher")]
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
async fn main() -> Result<(), BatcherError> {
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(8080);

    match cli.env_file {
        Some(env_file) => dotenvy::from_filename(env_file).ok(),
        None => dotenvy::dotenv().ok(),
    };

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let batcher = Batcher::new(cli.config).await;
    let batcher = Arc::new(batcher);

    let addr = format!("localhost:{}", port);

    // spawn task to listening for incoming blocks
    tokio::spawn({
        let app = batcher.clone();
        async move {
            app.listen_new_blocks()
                .await
                .expect("Error listening for new blocks exiting")
        }
    });

    batcher.metrics.inc_batcher_restart();

    batcher.listen_connections(&addr).await?;

    Ok(())
}
