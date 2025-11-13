#!/bin/bash
# This script is responsible for the handling of periodic sending, and the clauses for sending proofs.
# It forwards the path of the .env file as the first argument to the sender_with_alert.sh script when
# the conditions are met.

ENV_FILE="$1"

if [[ -z "$ENV_FILE" ]]; then
    echo "Usage: $0 path/to/.env"
    exit 1
fi

# Fetches the current ETH gas price
function fetch_gas_price() {
    gas_price=$(cast gas-price --rpc-url $RPC_URL)
    if [[ -z "$gas_price" || "$gas_price" == "0" ]]; then
        echo "Primary RPC_URL failed to fetch gas price, trying fallback..."
        gas_price=$(cast gas-price --rpc-url $RPC_URL_FALLBACK)
    fi

    echo $gas_price
}

source "$ENV_FILE"

# Each elapsed interval lasts for 30 minutes
sleep_time=1800
elapsed_intervals=0

./alerts/sender_with_alert.sh "$ENV_FILE"

while true; do
    echo "Starting pass #$elapsed_intervals"

    current_gas_price=$(fetch_gas_price)
    if [[ -z "$current_gas_price" || "$current_gas_price" == "0" ]]; then
        echo "Failed to fetch current gas price from both RPC URLs, skipping this pass."

        elapsed_intervals=$((elapsed_intervals + 1))

        echo "Sleeping $sleep_time seconds (($((sleep_time / 60)) minutes))"
        sleep "$sleep_time"
        continue
    fi
    echo "Current gas price: $current_gas_price wei"

    # In case current and gas price meet the criteria, send a proof and reset counter
    if { [ $elapsed_intervals -ge 10 ] && [ $elapsed_intervals -lt 14 ] && [ $current_gas_price -lt 2000000000 ]; }; then
        # Between 10 and 14 elapsed intervals (5 to 7 hours), if gas price is below 2 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 14 ] && [ $elapsed_intervals -lt 16 ] && [ $current_gas_price -lt 5000000000 ]; }; then
        # Between 14 and 16 elapsed intervals (7 to 8 hours), if gas price is below 5 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 16 ] && [ $elapsed_intervals -lt 24 ] && [ $current_gas_price -lt 15000000000 ]; }; then
        # Between 16 and 24 elapsed intervals (8 to 12 hours), if gas price is below 15 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 50 ]; }; then
        # After 50 elapsed intervals (25 hours) send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        elapsed_intervals=0
    fi

    elapsed_intervals=$((elapsed_intervals + 1))

    echo "Sleeping $sleep_time seconds (($((sleep_time / 60)) minutes))"
    sleep "$sleep_time"
done
