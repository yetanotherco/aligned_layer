use aligned_sdk::core::types::Network;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use std::str::FromStr;

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
pub struct GenerateAndFundWalletsArgs {
    #[arg(
        name = "Ethereum RPC provider connection address",
        long = "eth-rpc-url",
        default_value = "http://localhost:8545"
    )]
    pub eth_rpc_url: String,
    #[arg(
        name = "The funding wallet private key",
        long = "funding-wallet-private-key",
        default_value = ""
    )]
    pub funding_wallet_private_key: String,
    #[arg(
        name = "The number of wallets to generate",
        long = "number-wallets",
        default_value = "1"
    )]
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
    #[clap(flatten)]
    pub network: NetworkArg,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TestConnectionsArgs {
    #[clap(flatten)]
    pub network: NetworkArg,
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
    #[clap(flatten)]
    pub network: NetworkArg,
    #[arg(
        name = "Private keys filepath for the senders",
        long = "private-keys-filepath"
    )]
    pub private_keys_filepath: String,
    #[arg(
        name = "The generated proofs directory",
        long = "proofs-dirpath",
        default_value = "devnet"
    )]
    pub proofs_dir: String,
}

#[derive(Debug, Clone, Copy)]
enum NetworkNameArg {
    Devnet,
    Holesky,
    HoleskyStage,
    Mainnet,
}

impl FromStr for NetworkNameArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "devnet" => Ok(NetworkNameArg::Devnet),
            "holesky" => Ok(NetworkNameArg::Holesky),
            "holesky-stage" => Ok(NetworkNameArg::HoleskyStage),
            "mainnet" => Ok(NetworkNameArg::Mainnet),
            _ => Err(
                "Unknown network. Possible values: devnet, holesky, holesky-stage, mainnet"
                    .to_string(),
            ),
        }
    }
}

#[derive(Debug, clap::Args, Clone)]
pub struct NetworkArg {
    #[arg(
        name = "The working network's name",
        long = "network",
        default_value = "devnet",
        help = "[possible values: devnet, holesky, holesky-stage, mainnet]"
    )]
    network: Option<NetworkNameArg>,
    #[arg(
        name = "Aligned Service Manager Contract Address",
        long = "aligned_service_manager",
        conflicts_with("The working network's name"),
        requires("Batcher Payment Service Contract Address"),
        requires("Batcher URL")
    )]
    aligned_service_manager_address: Option<String>,
    #[arg(
        name = "Batcher Payment Service Contract Address",
        long = "batcher_payment_service",
        conflicts_with("The working network's name"),
        requires("Aligned Service Manager Contract Address"),
        requires("Batcher URL")
    )]
    batcher_payment_service_address: Option<String>,
    #[arg(
        name = "Batcher URL",
        long = "batcher_url",
        conflicts_with("The working network's name"),
        requires("Aligned Service Manager Contract Address"),
        requires("Batcher Payment Service Contract Address")
    )]
    batcher_url: Option<String>,
}

impl From<NetworkArg> for Network {
    fn from(network_arg: NetworkArg) -> Self {
        let mut processed_network_argument = network_arg.clone();

        if network_arg.batcher_url.is_some()
            || network_arg.aligned_service_manager_address.is_some()
            || network_arg.batcher_payment_service_address.is_some()
        {
            processed_network_argument.network = None; // We need this because network is Devnet as default, which is not true for a Custom network
        }

        match processed_network_argument.network {
            None => Network::Custom(
                network_arg.aligned_service_manager_address.unwrap(),
                network_arg.batcher_payment_service_address.unwrap(),
                network_arg.batcher_url.unwrap(),
            ),
            Some(NetworkNameArg::Devnet) => Network::Devnet,
            Some(NetworkNameArg::Holesky) => Network::Holesky,
            Some(NetworkNameArg::HoleskyStage) => Network::HoleskyStage,
            Some(NetworkNameArg::Mainnet) => Network::Mainnet,
        }
    }
}
