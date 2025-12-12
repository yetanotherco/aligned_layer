use std::env;

use payments_poller::{config::Config, db::Db, payments::PaymentsPoller};
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

    let payment_poller = PaymentsPoller::new(db, config);
    payment_poller.start().await;
}
