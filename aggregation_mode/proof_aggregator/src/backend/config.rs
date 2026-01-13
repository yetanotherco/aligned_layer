use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct ECDSAConfig {
    pub private_key_store_path: String,
    pub private_key_store_password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub eth_rpc_url: String,
    pub eth_ws_url: String,
    pub max_proofs_in_queue: u16,
    pub proof_aggregation_service_address: String,
    pub aligned_service_manager_address: String,
    pub ecdsa: ECDSAConfig,
    pub proofs_per_chunk: u16,
    pub total_proofs_limit: u16,
    pub risc0_chunk_aggregator_image_id: String,
    pub sp1_chunk_aggregator_vk_hash: String,
    pub zisk_chunk_aggregator_vk_hash_bytes: String,
    pub monthly_budget_eth: f64,
    pub db_connection_urls: Vec<String>,
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
