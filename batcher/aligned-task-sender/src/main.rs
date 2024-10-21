use clap::Parser;
use env_logger::Env;
use task_sender::{
    commands,
    structs::{TaskSenderArgs, TaskSenderCommands},
};

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = TaskSenderArgs::parse();

    match args.command {
        TaskSenderCommands::GenerateAndFundWallets(args) => {
            commands::generate_and_fund_wallets(args).await
        }
        TaskSenderCommands::GenerateProofs(args) => commands::generate_proofs(args).await,
        TaskSenderCommands::SendInfiniteProofs(args) => commands::send_infinite_proofs(args).await,
        TaskSenderCommands::TestConnections(args) => commands::test_connection(args).await,
    }
}
