use std::fmt;

use ethrex_common::{
    constants::MIN_BASE_FEE_PER_BLOB_GAS,
    types::{fake_exponential_checked, BLOB_BASE_FEE_UPDATE_FRACTION},
    U256,
};
use ethrex_rpc::{
    types::block_identifier::{BlockIdentifier, BlockTag},
    EthClient,
};

// values suggested from ethrex codebase
pub const MAXIMUM_ALLOWED_MAX_FEE_PER_BLOB_GAS: u64 = 10000000000; // 10 Gwei
pub const ARBITRARY_BASE_BLOB_GAS_PRICE: u64 = 1000000000; // 1 Gwei

#[derive(Clone, Debug)]
pub enum BlobEstimationError {
    OverflowError,
    FakeExponentialError(String),
}

impl fmt::Display for BlobEstimationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlobEstimationError::OverflowError => write!(f, "Overflow error"),
            BlobEstimationError::FakeExponentialError(e) => {
                write!(f, "Fake exponential error: {}", e)
            }
        }
    }
}

/// Estimates the gas price for blob transactions based on the current state of the blockchain.
///
/// # Parameters:
/// - `eth_client`: The Ethereum client used to fetch the latest block.
/// - `arbitrary_base_blob_gas_price`: The base gas price that serves as the minimum price for blob transactions.
/// - `headroom`: Percentage applied to the estimated gas price to provide a buffer against fluctuations.
///
/// # Formula:
/// The gas price is estimated using an exponential function based on the blob gas used in the latest block and the
/// excess blob gas from the block header, following the formula from EIP-4844:
/// ```txt
///    blob_gas = arbitrary_base_blob_gas_price + (excess_blob_gas + blob_gas_used) * headroom
/// ```
///
/// see: https://github.com/lambdaclass/ethrex/blob/a7655e53f9f75fc590cdc6de0f6ad79a0de551b4/crates/l2/sequencer/l1_committer.rs#L819
pub async fn estimate_blob_gas(
    eth_client: &EthClient,
    headroom: u64,
) -> Result<U256, BlobEstimationError> {
    let latest_block = eth_client
        .get_block_by_number(BlockIdentifier::Tag(BlockTag::Latest), false)
        .await
        .unwrap();

    let blob_gas_used = latest_block.header.blob_gas_used.unwrap_or(0);
    let excess_blob_gas = latest_block.header.excess_blob_gas.unwrap_or(0);

    // Check if adding the blob gas used and excess blob gas would overflow
    let total_blob_gas = excess_blob_gas
        .checked_add(blob_gas_used)
        .ok_or(BlobEstimationError::OverflowError)?;

    // If the blob's market is in high demand, the equation may give a really big number.
    // This function doesn't panic, it performs checked/saturating operations.
    let blob_gas = fake_exponential_checked(
        MIN_BASE_FEE_PER_BLOB_GAS,
        total_blob_gas,
        BLOB_BASE_FEE_UPDATE_FRACTION,
    )
    .map_err(|e| BlobEstimationError::FakeExponentialError(e.to_string()))?;

    let gas_with_headroom = (blob_gas * (100 + headroom)) / 100;

    // Check if we have an overflow when we take the headroom into account.
    let blob_gas = ARBITRARY_BASE_BLOB_GAS_PRICE
        .checked_add(gas_with_headroom)
        .ok_or(BlobEstimationError::OverflowError)?;

    Ok(U256::from(blob_gas))
}
