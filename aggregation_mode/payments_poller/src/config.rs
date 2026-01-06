use std::{fs::File, fs::OpenOptions, io::Read, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub db_connection_urls: Vec<String>,
    pub eth_rpc_url: String,
    pub payment_service_address: String,
    pub last_block_fetched_filepath: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LastBlockFetched {
    pub last_block_fetched: u64,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn get_last_block_fetched(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let mut file = File::open(&self.last_block_fetched_filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lbf_struct: LastBlockFetched = serde_json::from_str(&contents)?;
        Ok(lbf_struct.last_block_fetched)
    }

    pub fn update_last_block_fetched(
        &self,
        last_block_fetched: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let last_block_fetched_struct = LastBlockFetched { last_block_fetched };

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.last_block_fetched_filepath)?;

        let content = serde_json::to_string(&last_block_fetched_struct)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
