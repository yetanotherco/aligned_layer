use std::env;

use agg_mode_batcher::config::Config;
use agg_mode_batcher::payments::PaymentsPoller;
use agg_mode_batcher::{db::Db, server::http::BatcherServer};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn read_config_filepath_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(
            "You must provide a config file. Usage: {} <config-file-path>",
            args[0]
        );
    }

    args[1].clone()
}

#[tokio::main]
async fn main() {
    let filter = EnvFilter::new("info,sp1_cuda=warn");
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config_file_path = read_config_filepath_from_args();
    tracing::info!("Loading config from {}...", config_file_path);
    let config = Config::from_file(&config_file_path).expect("Config is valid");
    tracing::info!("Config loaded");

    let db = Db::try_new(&config.db_connection_url)
        .await
        .expect("db to start");

    let payment_poller = PaymentsPoller::new(db.clone(), config.clone());
    let http_server = BatcherServer::new(db, config.clone());

    let payment_poller_handle = tokio::spawn(async move { payment_poller.start().await });
    let http_server_handle = tokio::spawn(async move { http_server.start().await });

    // TODO: maybe this could two different processes (started with different commands) instead of being in the same one
    // TODO: abort the process if one stops instead of waiting for them both
    // TODO: ctrl + c handler for aborting the process should work
    let _ = tokio::join!(payment_poller_handle, http_server_handle);
}
