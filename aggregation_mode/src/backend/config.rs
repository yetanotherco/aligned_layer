use serde::{Deserialize, Serialize};
use std::{fs::File, fs::OpenOptions, io::Read, io::Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct ECDSAConfig {
    pub private_key_store_path: String,
    pub private_key_store_password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastAggregatedBlock {
    pub last_aggregated_block: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub eth_rpc_url: String,
    pub eth_ws_url: String,
    pub max_proofs_in_queue: u16,
    pub proof_aggregation_service_address: String,
    pub aligned_service_manager_address: String,
    pub last_aggregated_block_filepath: String,
    pub ecdsa: ECDSAConfig,
    pub proofs_per_chunk: u16,
    pub total_proofs_limit: u16,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn get_last_aggregated_block(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut file = File::open(&self.last_aggregated_block_filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lab: LastAggregatedBlock = serde_json::from_str(&contents)?;
        Ok(lab.last_aggregated_block)
    }

    pub fn update_last_aggregated_block(
        &self,
        last_aggregated_block: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let last_aggregated_block_struct = LastAggregatedBlock {
            last_aggregated_block,
        };

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.last_aggregated_block_filepath)?;

        let content = serde_json::to_string(&last_aggregated_block_struct)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
