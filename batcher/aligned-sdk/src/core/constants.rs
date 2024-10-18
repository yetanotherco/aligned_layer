/// Batcher ///
pub const AGGREGATOR_GAS_COST: u128 = 400_000;
pub const BATCHER_SUBMISSION_BASE_GAS_COST: u128 = 125_000;
pub const ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF: u128 = 13_000;
pub const CONSTANT_GAS_COST: u128 =
    ((AGGREGATOR_GAS_COST * DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER) / PERCENTAGE_DIVIDER)
        + BATCHER_SUBMISSION_BASE_GAS_COST;
pub const DEFAULT_MAX_FEE_PER_PROOF: u128 =
    ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000_000; // gas_price = 100 Gwei = 0.0000001 ether (high gas price)
pub const MIN_FEE_PER_PROOF: u128 = ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000; // gas_price = 0.1 Gwei = 0.0000000001 ether (low gas price)

// % modifiers: (100% is x1, 10% is x0.1, 1000% is x10)
pub const RESPOND_TO_TASK_FEE_LIMIT_PERCENTAGE_MULTIPLIER: u128 = 250; // fee_for_aggregator -> respondToTaskFeeLimit modifier
pub const DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER: u128 = 150; // feeForAggregator modifier
pub const GAS_PRICE_PERCENTAGE_MULTIPLIER: u128 = 110; // gasPrice modifier
pub const PERCENTAGE_DIVIDER: u128 = 100;

/// SDK ///
/// Number of proofs we a batch for estimation.
/// This is the number of proofs in a batch of size n, where we set n = 32.
/// i.e. the user pays for the entire batch and his proof is instantly submitted.
pub const MAX_FEE_BATCH_PROOF_NUMBER: usize = 32;
/// Estimated number of proofs for batch submission.
/// This corresponds to the number of proofs to compute for a default max_fee.
pub const MAX_FEE_DEFAULT_PROOF_NUMBER: usize = 10;
