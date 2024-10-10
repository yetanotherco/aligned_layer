use aligned_sdk::core::types::Network;
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
    #[clap(about = "Open socket connections with batcher")]
    TestConnections(TestConnectionsArgs),
    #[clap(about = "Send infinite proofs from a private-keys file")]
    SendInfiniteProofs(SendInfiniteProofsArgs),
    #[clap(about = "Generates wallets and funds it in aligned from one wallet")]
    GenerateAndFundWallets(GenerateAndFundWalletsArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GenerateProofsArgs {
    #[arg(name = "The number of proofs to generate", long = "number-of-proofs")]
    pub number_of_proofs: usize,
    #[arg(name = "The type of proof to generate", long = "proof-type")]
    pub proof_type: ProofType,
    #[arg(
        name = "The directory to which save the proofs. You'd then provide this path when sending proofs",
        long = "dir-to-save-proofs"
    )]
    pub dir_to_save_proofs: String,
}

#[derive(Parser, Clone, Debug, ValueEnum)]
pub enum ProofType {
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
    pub eth_rpc_url: String,
    #[arg(
        name = "The funding wallet private key",
        long = "funding-wallet-private-key"
    )]
    pub funding_wallet_private_key: String,
    #[arg(name = "The number of wallets to generate", long = "number-wallets")]
    pub number_of_wallets: usize,
    #[arg(
        name = "The amount to deposit to the wallets in ether",
        long = "amount-to-deposit"
    )]
    pub amount_to_deposit: String,
    #[arg(
        name = "The amount to deposit to aligned in ether",
        long = "amount-to-deposit-to-aligned"
    )]
    pub amount_to_deposit_to_aligned: String,
    #[arg(
        name = "The filepath to which to save the generated wallets's private key",
        long = "private-keys-filepath"
    )]
    pub private_keys_filepath: String,
    #[arg(
        name = "The Ethereum network's name",
        long = "network",
        default_value = "devnet"
    )]
    pub network: NetworkArg,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TestConnectionsArgs {
    #[arg(
        name = "Batcher connection address",
        long = "batcher-url",
        default_value = "ws://localhost:8080"
    )]
    pub batcher_url: String,
    #[arg(
        name = "Number of spawned sockets",
        long = "num-senders",
        default_value = "1"
    )]
    pub num_senders: usize,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SendInfiniteProofsArgs {
    #[arg(
        name = "Ethereum RPC provider connection address",
        long = "eth-rpc-url",
        default_value = "http://localhost:8545"
    )]
    pub eth_rpc_url: String,
    #[arg(
        name = "Batcher connection address",
        long = "batcher-url",
        default_value = "ws://localhost:8080"
    )]
    pub batcher_url: String,
    #[arg(
        name = "Number of proofs per burst",
        long = "burst-size",
        default_value = "10"
    )]
    pub burst_size: usize,
    #[arg(
        name = "Time to wait between bursts in seconds",
        long = "burst-time-secs",
        default_value = "3"
    )]
    pub burst_time_secs: u64,
    #[arg(name = "Max Fee", long = "max-fee", default_value = "1300000000000000")]
    pub max_fee: String,
    #[arg(
        name = "The Ethereum network's name",
        long = "network",
        default_value = "devnet"
    )]
    pub network: NetworkArg,
    #[arg(
        name = "Private keys filepath for the senders",
        long = "private-keys-file"
    )]
    pub private_keys_filepath: String,
    #[arg(
        name = "The generated proofs directory",
        long = "proof-dir-path",
        default_value = "devnet"
    )]
    pub proofs_dir: String,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum NetworkArg {
    Devnet,
    Holesky,
    HoleskyStage,
}

impl From<NetworkArg> for Network {
    fn from(chain_arg: NetworkArg) -> Self {
        match chain_arg {
            NetworkArg::Devnet => Network::Devnet,
            NetworkArg::Holesky => Network::Holesky,
            NetworkArg::HoleskyStage => Network::HoleskyStage,
        }
    }
}
