use serde::{Deserialize, Serialize};
use std::{fs::File, fs::OpenOptions, io::Read, io::Write};

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
    pub last_processed_block: u64,
    pub ecdsa: ECDSAConfig,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;
        let content = serde_yaml::to_string(&self)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
