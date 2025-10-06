use std::env;

use aligned_sdk::common::types::Network;

#[derive(Clone)]
pub struct EnvConfig {
    pub eth_rpc_url: String,
    pub private_key: String,
    pub fibonacci_contract_address: String,
    pub network: Network,
}

impl EnvConfig {
    pub fn new(env_file: Option<String>) -> Self {
        if let Some(env_file) = env_file {
            dotenv::from_filename(env_file).expect("env file to exist");
        } else {
            dotenv::dotenv().ok();
        }

        let eth_rpc_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set");
        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
        let fibonacci_contract_address =
            env::var("FIBONACCI_CONTRACT_ADDRESS").expect("FIBONACCI_CONTRACT_ADDRESS not set");

        let network = match env::var("NETWORK")
            .expect("NETWORK not set")
            .to_lowercase()
            .as_str()
        {
            "devnet" => Network::Devnet,
            "holesky-stage" => Network::HoleskyStage,
            "holesky" => Network::Holesky,
            "mainnet" => Network::Mainnet,
            "hoodi" => Network::Hoodi,
            _ => panic!("Unsupported NETWORK value"),
        };

        EnvConfig {
            eth_rpc_url,
            private_key,
            fibonacci_contract_address,
            network,
        }
    }
}
