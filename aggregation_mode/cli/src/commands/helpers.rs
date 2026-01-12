use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use clap::{self, Args, ValueEnum};
use std::path::PathBuf;
use std::str::FromStr;

use agg_mode_sdk::types::Network;

pub fn parse_network(value: &str) -> Result<Network, String> {
    Network::from_str(value).map_err(|_| format!("unsupported network supplied: {value}"))
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ProvingSystemArg {
    #[clap(name = "SP1")]
    SP1,
    #[clap(name = "Risc0")]
    Risc0,
}

#[derive(Args, Debug, Clone)]
#[group(required = true, multiple = false)]
pub struct PrivateKeyType {
    #[arg(name = "keystore_path", long = "keystore-path")]
    pub keystore_path: Option<PathBuf>,
    #[arg(name = "private_key", long = "private-key")]
    pub private_key: Option<String>,
}

impl PrivateKeyType {
    /// Creates a LocalSigner from either a keystore file or a raw private key.
    /// If a keystore path is provided, prompts for the password interactively.
    pub fn into_signer(self) -> Result<PrivateKeySigner, String> {
        if let Some(keystore_path) = self.keystore_path {
            let password = rpassword::prompt_password("Please enter your keystore password: ")
                .map_err(|e| format!("Failed to read password: {e}"))?;
            LocalSigner::decrypt_keystore(&keystore_path, password)
                .map_err(|e| format!("Failed to decrypt keystore: {e}"))
        } else if let Some(private_key) = self.private_key {
            LocalSigner::from_str(private_key.trim())
                .map_err(|e| format!("Failed to parse private key: {e}"))
        } else {
            Err("Either --keystore-path or --private-key must be provided".to_string())
        }
    }
}
