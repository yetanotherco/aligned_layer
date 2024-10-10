use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TaskSenderArgs {
    #[clap(subcommand)]
    pub command: TaskSenderCommands,
}

#[derive(Subcommand, Debug)]
pub enum TaskSenderCommands {
    #[clap(about = "Genere proofs")]
    GenerateProofs(GenerateProofsArgs),
    #[clap(about = "Clean proofs args")]
    CleanProofs(CleanProofsArgs),
    #[clap(about = "Open socket connections with batcher")]
    TestConnections(TestConnectionsArgs),
    #[clap(about = "Send infinite proofs from a private-keys file")]
    InfiniteProofs(InfiniteProofsArgs),
    #[clap(about = "Generates wallets and funds it in aligned from one wallet")]
    GenerateAndFundWallets(GenerateAndFundWalletsArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GenerateProofsArgs {
    #[arg(name = "The number of proofs to generate", long = "number-of-proofs")]
    number_of_proofs: usize,
    #[arg(name = "The type of proof to generate", long = "proof-type")]
    proof_type: ProofType,
}

#[derive(Parser, Clone, Debug, ValueEnum)]
enum ProofType {
    Groth16,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CleanProofsArgs {}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GenerateAndFundWalletsArgs {
    #[arg(
        name = "Ethereum RPC provider connection address",
        long = "rpc_url",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(name = "The number of wallets to generate", long = "number-wallets")]
    number_of_wallets: String,
    #[arg(
        name = "The amount to deposit to the wallets in ether",
        long = "amount-to-deposit"
    )]
    amount_to_deposit: String,
    #[arg(
        name = "The amount to deposit to aligned in ether",
        long = "amount-to-deposit-to-aligned"
    )]
    amount_to_deposit_to_aligned: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TestConnectionsArgs {
    #[arg(
        name = "Batcher connection address",
        long = "batcher-url",
        default_value = "ws://localhost:8080"
    )]
    batcher_url: String,
    #[arg(
        name = "Number of spawned sockets",
        long = "num-senders",
        default_value = "1"
    )]
    num_senders: usize,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InfiniteProofsArgs {
    #[arg(
        name = "Ethereum RPC provider connection address",
        long = "rpc_url",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "Batcher connection address",
        long = "batcher-url",
        default_value = "ws://localhost:8080"
    )]
    batcher_url: String,
    #[arg(
        name = "Number of proofs per burst",
        long = "burst-size",
        default_value = "10"
    )]
    burst_size: usize,
    #[arg(
        name = "Time to wait between bursts",
        long = "burst-time",
        default_value = "3"
    )]
    burst_time: usize,
    #[arg(name = "Max Fee", long = "max-fee", default_value = "1300000000000000")]
    max_fee: String,
    #[arg(
        name = "The Ethereum network's name",
        long = "network",
        default_value = "devnet"
    )]
    network: Network,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Network {
    Devnet,
    Holesky,
    HoleskyStage,
}
