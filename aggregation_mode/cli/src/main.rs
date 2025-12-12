use agg_mode_cli::commands::{self, submit::SubmitCommand};
use clap::{Parser, Subcommand};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand)]
    Submit(SubmitCommand),
}

#[tokio::main]
async fn main() {
    let filter = EnvFilter::new("info");
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();

    match cli.command {
        Command::Submit(subcommand) => match subcommand {
            SubmitCommand::SP1(args) => commands::submit::run(args).await,
        },
    };
}
