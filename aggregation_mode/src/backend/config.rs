use serde::{Deserialize, Serialize};
use std::{fs::File, fs::OpenOptions, io::Read, io::Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct ECDSAConfig {
    pub private_key_store_path: String,
    pub private_key_store_password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastProcessedBlock {
    pub last_processed_block: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub eth_rpc_url: String,
    pub eth_ws_url: String,
    pub max_proofs_in_queue: u16,
    pub proof_aggregation_service_address: String,
    pub aligned_service_manager_address: String,
    pub last_processed_block_filepath: String,
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

    pub fn get_last_processed_block(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match File::open(&self.last_processed_block_filepath) {
            Err(_) =>{
                // if file doesn't exist, default 0
                Ok(0)
            }
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let lpb: LastProcessedBlock = serde_json::from_str(&contents)?;
                Ok(lpb.last_processed_block)
            }
        } 
    }

    pub fn update_last_processed_block(
        &self,
        last_processed_block: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let last_processed_block_struct = LastProcessedBlock {
            last_processed_block: last_processed_block,
        };

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.last_processed_block_filepath)?;

        let content = serde_json::to_string(&last_processed_block_struct)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
