use ethers::types::{Address, Signature};
use serde::{Deserialize, Serialize};
use aligned_sdk::common::types::NoncedVerificationData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NonPayingData {
    pub address: Address,
    pub nonced_verification_data: NoncedVerificationData,
    pub signature: Signature,
}
