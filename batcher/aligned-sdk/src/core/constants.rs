/// Batcher ///
pub const GAS_PRICE_INCREMENT_PERCENTAGE_PER_ITERATION: usize = 5;
pub const DEFAULT_AGGREGATOR_GAS_COST: u128 = 330_000;
pub const BATCHER_SUBMISSION_BASE_GAS_COST: u128 = 125_000;
pub const ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF: u128 = 2_000;
pub const DEFAULT_CONSTANT_GAS_COST: u128 = ((DEFAULT_AGGREGATOR_GAS_COST
    * DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER)
    / PERCENTAGE_DIVIDER)
    + BATCHER_SUBMISSION_BASE_GAS_COST;
pub const DEFAULT_MAX_FEE_PER_PROOF: u128 =
    ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000_000; // gas_price = 100 Gwei = 0.0000001 ether (high gas price)
pub const CONNECTION_TIMEOUT: u64 = 30; // 30 secs

// According to:
// - https://www.rfc-editor.org/rfc/rfc8949.html#section-3.1-2.10
// - https://www.rfc-editor.org/rfc/rfc8949.html#section-3-3.2
// - https://www.rfc-editor.org/rfc/rfc8949.html#section-3-3.4
// 9 bytes are the maximum overhead from aggregating data into an array in CBOR
// (it may be as little as just 1 byte, but it depends on the number of elements
// and serialization parameters).
pub const CBOR_ARRAY_MAX_OVERHEAD: usize = 9;

// % modifiers: (100% is x1, 10% is x0.1, 1000% is x10)
pub const RESPOND_TO_TASK_FEE_LIMIT_PERCENTAGE_MULTIPLIER: u128 = 250; // fee_for_aggregator -> respondToTaskFeeLimit modifier
pub const DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER: u128 = 125; // feeForAggregator modifier
pub const GAS_PRICE_PERCENTAGE_MULTIPLIER: u128 = 110; // gasPrice modifier
pub const OVERRIDE_GAS_PRICE_PERCENTAGE_MULTIPLIER: u128 = 120; // gasPrice modifier to override previous transactions
pub const PERCENTAGE_DIVIDER: u128 = 100;

/// SDK ///
/// Constants used for `max_fee` estimation in the sdk `estimate_fee()` function.
/// The number of proofs in a batch to compute the `Instant` fee estimate for proof submission to Aligned.
/// i.e. the user pays for the entire batch and his proof is instantly submitted, therefore a batch of one proof.
pub const INSTANT_MAX_FEE_BATCH_SIZE: usize = 1;
/// The number of proofs in a batch to compute the `Default` fee estimate for proof submission to Aligned.
/// We define `10` as the `Default` setting as every 6 hours the batcher receives a batch of `16` proofs
/// sent from Aligned to confirm the network is live and estimating with a batch size of `10` proofs provides a buffer in case of a network fee increase.
pub const DEFAULT_MAX_FEE_BATCH_SIZE: usize = 10;

/// Ethereum calls retry constants
pub const ETHEREUM_CALL_MIN_RETRY_DELAY: u64 = 500; // milliseconds
pub const ETHEREUM_CALL_MAX_RETRIES: usize = 5;
pub const ETHEREUM_CALL_BACKOFF_FACTOR: f32 = 2.0;
pub const ETHEREUM_CALL_MAX_RETRY_DELAY: u64 = 3600; // seconds

/// Ethereum transaction retry constants
pub const BUMP_MIN_RETRY_DELAY: u64 = 500; // milliseconds
pub const BUMP_MAX_RETRIES: usize = 33; // ~ 1 day
pub const BUMP_BACKOFF_FACTOR: f32 = 2.0;
pub const BUMP_MAX_RETRY_DELAY: u64 = 3600; // seconds

/// NETWORK ADDRESSES ///
/// BatcherPaymentService
pub const BATCHER_PAYMENT_SERVICE_ADDRESS_DEVNET: &str =
    "0x7bc06c482DEAd17c0e297aFbC32f6e63d3846650";
pub const BATCHER_PAYMENT_SERVICE_ADDRESS_HOLESKY: &str =
    "0x815aeCA64a974297942D2Bbf034ABEe22a38A003";
pub const BATCHER_PAYMENT_SERVICE_ADDRESS_HOLESKY_STAGE: &str =
    "0x7577Ec4ccC1E6C529162ec8019A49C13F6DAd98b";
pub const BATCHER_PAYMENT_SERVICE_ADDRESS_MAINNET: &str =
    "0xb0567184A52cB40956df6333510d6eF35B89C8de";
/// AlignedServiceManager
pub const ALIGNED_SERVICE_MANAGER_DEVNET: &str = "0x851356ae760d987E095750cCeb3bC6014560891C";
pub const ALIGNED_SERVICE_MANAGER_HOLESKY: &str = "0x58F280BeBE9B34c9939C3C39e0890C81f163B623";
pub const ALIGNED_SERVICE_MANAGER_HOLESKY_STAGE: &str =
    "0x9C5231FC88059C086Ea95712d105A2026048c39B";
pub const ALIGNED_SERVICE_MANAGER_MAINNET: &str = "0xeF2A435e5EE44B2041100EF8cbC8ae035166606c";
/// Batcher URL's
pub const BATCHER_URL_DEVNET: &str = "ws://localhost:8080";
pub const BATCHER_URL_HOLESKY: &str = "wss://batcher.alignedlayer.com";
pub const BATCHER_URL_HOLESKY_STAGE: &str = "wss://stage.batcher.alignedlayer.com";
pub const BATCHER_URL_MAINNET: &str = "wss://mainnet.batcher.alignedlayer.com";
