extern crate dotenv;

use dotenv::dotenv;

use std::io::Error as IoError;
use std::sync::Arc;

use clap::Parser;
use env_logger::Env;

use batcher::{App, Listener};

#[derive(Parser)]
#[command(name = "Aligned Layer Batcher")]
#[command(about = "An application with server and client subcommands", long_about = None)]
struct Cli {
    #[arg(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let app = App::new().await;
    let app = Arc::new(app);

    let cli = Cli::parse();
    let port = cli.port.unwrap_or(8080);

    let addr = format!("localhost:{}", port);

    app.listen(&addr).await;

    Ok(())
}
