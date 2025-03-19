use ethers::{core::k256::ecdsa::SigningKey, signers::Wallet, types::Address};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ECDSAConfig {
    pub private_key_store_path: String,
    pub private_key_store_password: String,
}

#[derive(Debug)]
pub struct NonPayingConfig {
    pub address: Address,
    pub replacement: Wallet<SigningKey>,
}

#[derive(Debug, Deserialize)]
pub struct NonPayingConfigFromYaml {
    pub address: Address,
    pub replacement_private_key: String,
}

impl NonPayingConfig {
    pub async fn from_yaml_config(config: NonPayingConfigFromYaml) -> Self {
        let replacement = Wallet::from_bytes(
            hex::decode(config.replacement_private_key)
                .expect("Failed to decode replacement private key")
                .as_slice(),
        )
        .expect("Failed to create replacement wallet");

        NonPayingConfig {
            address: config.address,
            replacement,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BatcherConfigFromYaml {
    #[serde(default = "default_aggregator_fee_percentage_multiplier")]
    pub aggregator_fee_percentage_multiplier: u128,
    #[serde(default = "default_aggregator_gas_cost")]
    pub aggregator_gas_cost: u128,
    pub block_interval: u64,
    pub transaction_wait_timeout: u64,
    pub max_proof_size: usize,
    pub max_batch_byte_size: usize,
    pub max_batch_proof_qty: usize,
    pub pre_verification_is_enabled: bool,
    pub metrics_port: u16,
    pub telemetry_ip_port_address: String,
    pub non_paying: Option<NonPayingConfigFromYaml>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFromYaml {
    pub eth_rpc_url: String,
    pub eth_rpc_url_fallback: String,
    pub eth_ws_url: String,
    pub eth_ws_url_fallback: String,
    pub ecdsa: ECDSAConfig,
    pub aligned_layer_deployment_config_file_path: String,
    pub batcher: BatcherConfigFromYaml,
}

impl ConfigFromYaml {
    pub fn new(config_file: String) -> Self {
        let config_file = std::fs::read_to_string(config_file).expect("Failed to read config file");
        serde_yaml::from_str(&config_file).expect("Failed to parse config file")
    }
}

#[derive(Debug, Deserialize)]
pub struct Addresses {
    #[serde(rename = "batcherPaymentService")]
    pub batcher_payment_service: String,
    #[serde(rename = "alignedLayerServiceManager")]
    pub service_manager: String,
}

#[derive(Debug, Deserialize)]
pub struct ContractDeploymentOutput {
    pub addresses: Addresses,
}

impl ContractDeploymentOutput {
    pub fn new(deployment_output: String) -> Self {
        let deployment_output = std::fs::read_to_string(deployment_output)
            .expect("Failed to read deployment output file");
        serde_json::from_str(&deployment_output).expect("Failed to parse deployment output file")
    }
}

fn default_aggregator_fee_percentage_multiplier() -> u128 {
    aligned_sdk::core::constants::DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER
}

fn default_aggregator_gas_cost() -> u128 {
    aligned_sdk::core::constants::DEFAULT_AGGREGATOR_GAS_COST
}
