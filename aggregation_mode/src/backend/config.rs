use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct ECDSAConfig {
    pub private_key_store_path: String,
    pub private_key_store_password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub eth_rpc_url: String,
    pub eth_ws_url: String,
    pub max_proofs_in_queue: u16,
    pub proof_aggregation_service_address: String,
    pub aligned_service_manager_address: String,
    pub ecdsa: ECDSAConfig,
    pub fetch_logs_from_secs_ago: u64,
    pub block_time_secs: u64,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
