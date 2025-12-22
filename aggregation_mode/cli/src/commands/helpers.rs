use clap::{self, ValueEnum};
use std::str::FromStr;

use agg_mode_sdk::types::Network;

pub fn parse_network(value: &str) -> Result<Network, String> {
    Network::from_str(value).map_err(|_| format!("unsupported network supplied: {value}"))
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ProvingSystemArg {
    #[clap(name = "GnarkPlonkBls12_381")]
    GnarkPlonkBls12_381,
    #[clap(name = "GnarkPlonkBn254")]
    GnarkPlonkBn254,
    #[clap(name = "GnarkGroth16Bn254")]
    GnarkGroth16Bn254,
    #[clap(name = "SP1")]
    SP1,
    #[clap(name = "Risc0")]
    Risc0,
    #[clap(name = "CircomGroth16Bn256")]
    CircomGroth16Bn256,
    #[clap(name = "Mina")]
    Mina,
    #[clap(name = "MinaAccount")]
    MinaAccount,
}
