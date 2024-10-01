/// Batcher ///
pub const AGGREGATOR_GAS_COST: u128 = 400_000;
pub const BATCHER_SUBMISSION_BASE_GAS_COST: u128 = 125_000;
pub const ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF: u128 = 13_000;
pub const CONSTANT_GAS_COST: u128 = ((AGGREGATOR_GAS_COST * DEFAULT_AGGREGATOR_FEE_MULTIPLIER)
    / DEFAULT_AGGREGATOR_FEE_DIVIDER)
    + BATCHER_SUBMISSION_BASE_GAS_COST;
pub const DEFAULT_MAX_FEE_PER_PROOF: u128 =
    ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000_000; // gas_price = 100 Gwei = 0.0000001 ether (high gas price)
pub const MIN_FEE_PER_PROOF: u128 = ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000; // gas_price = 0.1 Gwei = 0.0000000001 ether (low gas price)
pub const RESPOND_TO_TASK_FEE_LIMIT_MULTIPLIER: u128 = 5; // to set the respondToTaskFeeLimit variable higher than fee_for_aggregator
pub const RESPOND_TO_TASK_FEE_LIMIT_DIVIDER: u128 = 2;
pub const DEFAULT_AGGREGATOR_FEE_MULTIPLIER: u128 = 3; // to set the feeForAggregator variable higher than what was calculated
pub const DEFAULT_AGGREGATOR_FEE_DIVIDER: u128 = 2;

/// SDK ///
/// Estimated number of proofs for instant batch submission.
pub const MAX_FEE_INSTANT_BATCH_SIZE: usize = 32;
/// Estimated number of proofs for instant batch submission.
pub const MAX_FEE_DEFAULT_BATCH_SIZE: usize = 10;
