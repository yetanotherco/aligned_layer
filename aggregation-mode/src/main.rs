use std::env;

use proof_aggregator::backend::{config::Config, ProofAggregator};
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

    let mut proof_aggregator = ProofAggregator::new(&config);
    proof_aggregator.start().await;
}
