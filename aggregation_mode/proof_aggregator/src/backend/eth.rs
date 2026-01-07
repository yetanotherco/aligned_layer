use alloy::primitives::{utils::parse_ether, U256};
use std::time::Duration;

// We assume a fixed gas cost of 300,000 for each of the 2 transactions
const ON_CHAIN_COST_IN_GAS_UNITS: u64 = 600_000u64;

/// Decides whether to send the aggregated proof to be verified on-chain based on
/// time elapsed since last submission and monthly ETH budget.
/// We make a linear function with the eth to spend this month and the time elapsed since last submission.
/// If eth to spend / elapsed time is over the linear function, we skip the submission.
pub fn should_send_proof_to_verify_on_chain(
    time_elapsed: Duration,
    monthly_eth_budget: f64,
    network_gas_price: U256,
) -> bool {
    let on_chain_cost_in_gas: U256 = U256::from(ON_CHAIN_COST_IN_GAS_UNITS);
    let max_to_spend_wei = max_to_spend_in_wei(time_elapsed, monthly_eth_budget);

    let expected_cost_in_wei = network_gas_price * on_chain_cost_in_gas;

    expected_cost_in_wei <= max_to_spend_wei
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

#[cfg(test)]
mod tests {
    use super::should_send_proof_to_verify_on_chain;
    use alloy::primitives::U256;
    use std::time::Duration;

    #[test]
    fn test_should_send_proof_to_verify_on_chain_updated_cases() {
        // The should_send_proof_to_verify_on_chain function returns true when:
        // gas_price * 600_000 <= (seconds_elapsed) * (monthly_eth_budget / (30 * 24 * 60 * 60))

        const BUDGET_PER_MONTH_IN_ETH: f64 = 0.15;
        const ONE_DAY_SECONDS: u64 = 24 * 60 * 60;
        let gas_price = U256::from(1_000_000_000u64); // 1 Gwei

        // Case 1: Base case -> should return true
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000058 ETH/hour * 24 hours = 0.005 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            BUDGET_PER_MONTH_IN_ETH,              // 0.15 ETH monthly budget
            gas_price,                            // 1 Gwei gas price
        ));

        // Case 2: Slightly Increased Gas Price -> should return true
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 8 Gwei
        // Max to spend: 0.000000058 ETH/hour * 24 hours = 0.005 ETH
        // Expected cost: 600,000 * 8 Gwei = 0.0048 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            BUDGET_PER_MONTH_IN_ETH,              // 0.15 ETH monthly budget
            U256::from(8_000_000_000u64),         // 8 Gwei gas price
        ));

        // Case 3: Increased Gas Price -> should return false
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 10 Gwei
        // Max to spend: 0.000000058 ETH/hour * 24 hours = 0.005 ETH
        // Expected cost: 600,000 * 10 Gwei = 0.006 ETH
        // Expected cost > Max to spend, so we cannot send the proof
        assert!(!should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            BUDGET_PER_MONTH_IN_ETH,              // 0.15 ETH monthly budget
            U256::from(10_000_000_000u64),        // 10 Gwei gas price
        ));

        // Case 4: Slightly Reduced Time Elapsed -> should return true
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 3 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000058 ETH/hour * 3 hours = 0.000625 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(should_send_proof_to_verify_on_chain(
            Duration::from_secs(3 * 3600), // 3 hours
            BUDGET_PER_MONTH_IN_ETH,       // 0.15 ETH monthly budget
            gas_price,                     // 1 Gwei gas price
        ));

        // Case 5: Reduced Time Elapsed -> should return false
        // Monthly Budget: 0.15 ETH -> 0.005 ETH per day -> 0.000000058 ETH per hour
        // Elapsed Time: 1.2 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000058 ETH/hour * 1.2 hours = 0.00025 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost > Max to spend, so we cannot send the proof
        assert!(!should_send_proof_to_verify_on_chain(
            Duration::from_secs_f64(1.2 * 3600.0), // 1.2 hours
            BUDGET_PER_MONTH_IN_ETH,               // 0.15 ETH monthly budget
            gas_price,                             // 1 Gwei gas price
        ));

        // Case 6: Slightly Reduced Monthly Budget -> should return true
        // Monthly Budget: 0.1 ETH -> 0.0033 ETH per day -> 0.000000038 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.000000038 ETH/hour * 24 hours = 0.0032832 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost < Max to spend, so we can send the proof
        assert!(should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            0.1,                                  // 0.1 ETH monthly budget
            gas_price,                            // 1 Gwei gas price
        ));

        // Case 7: Decreased Monthly Budget -> should return false
        // Monthly Budget: 0.01 ETH -> 0.00033 ETH per day -> 0.0000000038 ETH per hour
        // Elapsed Time: 24 hours
        // Gas Price: 1 Gwei
        // Max to spend: 0.0000000038 ETH/hour * 24 hours = 0.00032832 ETH
        // Expected cost: 600,000 * 1 Gwei = 0.0006 ETH
        // Expected cost > Max to spend, so we cannot send the proof
        assert!(!should_send_proof_to_verify_on_chain(
            Duration::from_secs(ONE_DAY_SECONDS), // 24 hours
            0.01,                                 // 0.01 ETH monthly budget
            gas_price,                            // 1 Gwei gas price
        ));
    }
}
