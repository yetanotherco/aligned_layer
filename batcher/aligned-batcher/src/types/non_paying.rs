use aligned_sdk::common::types::NoncedVerificationData;
use ethers::types::{Address, Signature};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NonPayingReplacementData {
    pub address: Address,
    pub nonced_verification_data: NoncedVerificationData,
    pub signature: Signature,
}
