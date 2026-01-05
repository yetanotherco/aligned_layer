use std::str::FromStr;

use crate::constants::{
    ALIGNED_AGG_MODE_GATEWAY_URL_DEVNET, ALIGNED_AGG_MODE_GATEWAY_URL_HOODI,
    ALIGNED_AGG_PAYMENT_SERVICE_ADDRESS_DEVNET, ALIGNED_AGG_PAYMENT_SERVICE_ADDRESS_HOODI,
    ALIGNED_PROOF_AGG_SERVICE_ADDRESS_DEVNET, ALIGNED_PROOF_AGG_SERVICE_ADDRESS_HOODI,
};

#[derive(Debug, Clone)]
pub enum Network {
    Devnet,
    Hoodi,
}

#[derive(Debug, Clone)]
pub enum NetworkError {
    InvalidNetwork,
}

impl FromStr for Network {
    type Err = NetworkError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "devnet" => Ok(Self::Devnet),
            "hoodi" => Ok(Self::Hoodi),
            _ => Err(NetworkError::InvalidNetwork),
        }
    }
}

impl Network {
    pub fn chain_id(&self) -> u64 {
        match self {
            Self::Devnet => 31_337,
            Self::Hoodi => 56_0048,
        }
    }

    pub fn gateway_url(&self) -> String {
        match self {
            Self::Hoodi => ALIGNED_AGG_MODE_GATEWAY_URL_HOODI.to_string(),
            Self::Devnet => ALIGNED_AGG_MODE_GATEWAY_URL_DEVNET.to_string(),
        }
    }

    pub fn proof_aggregator_contract_address(&self) -> String {
        match self {
            Self::Hoodi => ALIGNED_PROOF_AGG_SERVICE_ADDRESS_HOODI.to_string(),
            Self::Devnet => ALIGNED_PROOF_AGG_SERVICE_ADDRESS_DEVNET.to_string(),
        }
    }

    pub fn aggregation_mode_payment_service_address(&self) -> String {
        match self {
            Self::Hoodi => ALIGNED_AGG_PAYMENT_SERVICE_ADDRESS_HOODI.to_string(),
            Self::Devnet => ALIGNED_AGG_PAYMENT_SERVICE_ADDRESS_DEVNET.to_string(),
        }
    }
}
