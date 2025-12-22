use agg_mode_sdk::types::Network;
use clap::{self, Args};
use std::path::PathBuf;

use crate::commands::helpers::{parse_network, ProvingSystemArg};

#[derive(Debug, Clone, Args)]
pub struct VerifyOnChainArgs {
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
    #[arg(long = "rpc-url")]
    rpc_url: String,
    #[arg(long = "beacon-url")]
    beacon_url: String,
    #[arg(long = "from-block")]
    from_block: Option<u64>,
    #[arg(long = "proving-system")]
    proving_system: ProvingSystemArg,
    #[arg(name = "Program verification key hash", long = "vk", required = true)]
    program_vk: PathBuf,
    #[arg(long = "vk")]
    verifying_key_path: PathBuf,
    #[arg(name = "Public input file name", long = "public-input")]
    pub_input_file_name: Option<PathBuf>,
}

pub async fn run(args: VerifyOnChainArgs) {}
