use ethers::utils::parse_ether;

pub fn max_to_spend_in_wei(time_elapsed: Duration, monthly_eth_budget: f64) -> U256 {
    const SECONDS_PER_MONTH: u64 = 30 * 24 * 60 * 60;

    // Note: this unwrap is safe because parse_ether only fails for negative numbers or invalid strings
    let monthly_budget_in_wei = parse_ether(monthly_eth_budget).unwrap_or(U256::zero());

    let elapsed_seconds = U256::from(time_elapsed.as_secs());

    let budget_available_per_second_in_wei = monthly_budget_in_wei / SECONDS_PER_MONTH;

    budget_available_per_second_in_wei * elapsed_seconds
}
