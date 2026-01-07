use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub db_connection_urls: Vec<String>,
    pub network: String,
    pub max_daily_proofs_per_user: i64,
    pub gateway_metrics_port: u16,
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
