extern crate dotenvy;

use std::sync::Arc;

use clap::Parser;

use aligned_batcher::{telemetry, types::errors::BatcherError, Batcher};
use opentelemetry::global::shutdown_tracer_provider;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

    // Intialize tokio::tracing_subscriber with OpenTelemetry.
    let tracer = telemetry::init_tracer("http://localhost:4317")
        .expect("Failed to initialize tracer provider.");
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

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

    batcher.listen_connections(&addr).await?;

    shutdown_tracer_provider();
    Ok(())
}
