use std::str::FromStr;

use agg_mode_sdk::types::Network;

pub fn parse_network(value: &str) -> Result<Network, String> {
    Network::from_str(value).map_err(|_| format!("unsupported network supplied: {value}"))
}
