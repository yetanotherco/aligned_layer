use std::env;

use agg_mode_batcher::config::Config;
use agg_mode_batcher::{db::Db, server::BatcherServer};

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

#[actix_web::main]
async fn main() {
    let config_file_path = read_config_filepath_from_args();
    tracing::info!("Loading config from {}...", config_file_path);
    let config = Config::from_file(&config_file_path).expect("Config is valid");
    tracing::info!("Config loaded");

    let db = Db::try_new(&config.db_connection_url)
        .await
        .expect("db to start");
    let http_server = BatcherServer::new(db.clone(), config);

    http_server.start().await.expect("Server to keep running");
}
