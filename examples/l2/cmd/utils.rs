use std::env;

use aligned_sdk::common::types::Network;
use dotenv::dotenv;
use l2::config::Config;

pub fn load_config() -> Config {
    dotenv().ok();

    let network = match env::var("NETWORK").expect("NETWORK not set").as_str() {
        "holesky" => Network::Holesky,
        "holesky-stage" => Network::HoleskyStage,
        "devnet" => Network::Devnet,
        _ => panic!("Invalid network, possible values are: holesky, holesky-stage, devnet"),
    };

    let config = Config {
        network,
        eth_rpc_url: env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set"),
        ws_eth_rpc_url: env::var("WS_ETH_RPC_URL").expect("WS_ETH_RPC_URL not set"),
        beacon_client_url: env::var("BEACON_CLIENT_URL").expect("BEACON_CLIENT_URL not set"),
        private_key_store_path: env::var("PRIVATE_KEY_STORE_PATH")
            .expect("PRIVATE_KEY_STORE_PATH not set"),
        private_key_store_password: env::var("PRIVATE_KEY_STORE_PASSWORD")
            .expect("PRIVATE_KEY_STORE_PASSWORD not set"),
        state_transition_contract_address: env::var("STATE_TRANSITION_CONTRACT_ADDRESS")
            .expect("STATE_TRANSITION_CONTRACT_ADDRESS not set"),
        db_path: Some("./db".to_string()),
    };

    config
}
