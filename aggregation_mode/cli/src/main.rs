use agg_mode_cli::commands::{self, submit::SubmitCommand, verify::VerifyCommand, Cli, Command};
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
            SubmitCommand::SP1(args) => commands::submit::run_sp1(args).await,
            SubmitCommand::Zisk(args) => commands::submit::run_zisk(args).await,
        },
        Command::VerifyOnChain(subcommand) => match subcommand {
            VerifyCommand::SP1(args) => commands::verify::run_sp1(args).await,
            VerifyCommand::Risc0(args) => commands::verify::run_risc0(args).await,
            VerifyCommand::Zisk(args) => commands::verify::run_zisk(args).await,
        },
        Command::Deposit(args) => commands::deposit::run(args).await,
    };
}
