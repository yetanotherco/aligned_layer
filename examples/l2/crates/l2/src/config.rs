pub struct Config {
    pub network: aligned_sdk::common::types::Network,
    pub eth_rpc_url: String,
    pub ws_eth_rpc_url: String,
    pub beacon_client_url: String,
    pub private_key_store_path: String,
    pub private_key_store_password: String,
    pub state_transition_contract_address: String,
    pub db_path: Option<String>,
}
