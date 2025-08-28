use aligned_sdk::common::types::Network;

#[derive(Clone)]
pub struct EnvConfig {
    pub eth_rpc_url: String,
    pub private_key_store_path: String,
    pub private_key_store_password: String,
    pub fibonacci_contract_address: String,
    pub network: Network,
}
