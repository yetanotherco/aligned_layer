use clap::{self, ValueEnum};
use std::str::FromStr;

use agg_mode_sdk::types::Network;

pub fn parse_network(value: &str) -> Result<Network, String> {
    Network::from_str(value).map_err(|_| format!("unsupported network supplied: {value}"))
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ProvingSystemArg {
    #[clap(name = "SP1")]
    SP1,
    #[clap(name = "Risc0")]
    Risc0,
}
