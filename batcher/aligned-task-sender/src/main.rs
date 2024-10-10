use clap::Parser;
use task_sender::{
    commands,
    structs::{TaskSenderArgs, TaskSenderCommands},
};

#[tokio::main]
async fn main() {
    let args = TaskSenderArgs::parse();

    match args.command {
        TaskSenderCommands::GenerateAndFundWallets(args) => {
            commands::generate_and_fund_wallets(args).await
        }
        TaskSenderCommands::GenerateProofs(args) => commands::generate_proofs(args).await,
        TaskSenderCommands::SendInfiniteProofs(args) => commands::infinite_proofs(args).await,
        TaskSenderCommands::TestConnections(args) => commands::test_connection(args).await,
    }
}
