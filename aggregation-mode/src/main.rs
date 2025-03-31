use std::{env, sync::Arc};

use proof_aggregator::backend::{
    config::Config, fetcher::ProofsFetcher, queue::ProofsQueue, ProofAggregator,
};
use tokio::sync::Mutex;
use tracing_subscriber::FmtSubscriber;

fn read_config_filepath_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(
            "You mus provide a config file. Usage: {} <config-file-path>",
            args[0]
        );
    }

    args[1].clone()
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // load config
    let config_file_path = read_config_filepath_from_args();
    tracing::info!("Loading config from {}...", config_file_path);
    let config = Config::from_file(&config_file_path).expect("Config is valid");
    tracing::info!("Config loaded");

    let queue = Arc::new(Mutex::new(ProofsQueue::new(config.max_proofs_in_queue)));
    let mut proof_aggregator = ProofAggregator::new(&config, queue.clone()).await;
    let proofs_fetcher = ProofsFetcher::new(&config, queue).await;

    // start tasks -> Proof aggregator + Proofs fetcher
    let proof_aggregator_handle = tokio::spawn(async move { proof_aggregator.start().await });
    let proofs_fetcher_handle = tokio::spawn(async move { proofs_fetcher.start().await });

    let _ = tokio::join!(proof_aggregator_handle, proofs_fetcher_handle);
}
