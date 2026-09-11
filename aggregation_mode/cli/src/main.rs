use agg_mode_cli::commands::{self, submit::SubmitCommand, Cli, Command};
use clap::Parser;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

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
        Command::VerifyOnChain(args) => commands::verify::run(args).await,
        Command::Deposit(args) => commands::deposit::run(args).await,
    };
}
