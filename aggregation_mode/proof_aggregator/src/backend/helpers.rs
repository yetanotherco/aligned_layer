use alloy::primitives::{utils::parse_ether, U256};
use std::time::Duration;

/// Decides whether to send the aggregated proof to be verified on-chain based on
/// time elapsed since last submission and monthly ETH budget.
/// We make a linear function with the eth to spend this month and the time elapsed since last submission.
/// If eth to spend / elapsed time is over the linear function, we skip the submission.
pub fn should_send_proof_to_verify_on_chain(
    time_elapsed: Duration,
    monthly_eth_budget: f64,
    network_gas_price: U256,
) -> bool {
    // We assume a fixed gas cost of 300,000 for each of the 2 transactions
    const ON_CHAIN_COST_IN_GAS_UNITS: u64 = 600_000u64;

    let on_chain_cost_in_gas: U256 = U256::from(ON_CHAIN_COST_IN_GAS_UNITS);
    let max_to_spend_in_wei = max_to_spend_in_wei(time_elapsed, monthly_eth_budget);

    let expected_cost_in_wei = network_gas_price * on_chain_cost_in_gas;

    expected_cost_in_wei <= max_to_spend_in_wei
}

fn max_to_spend_in_wei(time_elapsed: Duration, monthly_eth_budget: f64) -> U256 {
    const SECONDS_PER_MONTH: u64 = 30 * 24 * 60 * 60;

    // Note: this expect is safe because in case it was invalid, should have been caught at startup
    let monthly_budget_in_wei = parse_ether(&monthly_eth_budget.to_string())
        .expect("The monthly budget should be a non-negative value");

    let elapsed_seconds = U256::from(time_elapsed.as_secs());

    let budget_available_per_second_in_wei = monthly_budget_in_wei / U256::from(SECONDS_PER_MONTH);

    budget_available_per_second_in_wei * elapsed_seconds
}
