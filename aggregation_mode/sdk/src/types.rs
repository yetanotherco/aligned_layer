use std::str::FromStr;

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
            Self::Devnet => "http://127.0.0.1:8089".to_string(),
            Self::Hoodi => "http://hoodi.gateway.alignedlayer.com:8080".to_string(),
        }
    }
}
