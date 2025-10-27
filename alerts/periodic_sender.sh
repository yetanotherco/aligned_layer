#!/bin/bash
# This script is responsible for the handling of periodic sending, and the clauses for sending proofs.
# It forwards the path of the .env file as the first argument to the sender_with_alert.sh script when
# the conditions are met.

ENV_FILE="$1"

if [[ -z "$ENV_FILE" ]]; then
    echo "Usage: $0 path/to/.env"
    exit 1
fi

function send_proof_background() {
    ./alerts/sender_with_alert.sh "$ENV_FILE" &
}

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

# Each elapsed interval lasts for 5 minutes
sleep_time=300
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
    if { [ $elapsed_intervals -ge 1 ] && [ $elapsed_intervals -lt 3 ] && [ $current_gas_price -lt 1500000000 ]; }; then
        # Between 1 and 3 elapsed intervals (5 to 15 minutes), if gas price is below 1.5 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 3 ] && [ $elapsed_intervals -lt 6 ] && [ $current_gas_price -lt 4500000000 ]; }; then
        # Between 3 and 6 elapsed intervals (15 to 30 minutes), if gas price is below 4.5 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 6 ] && [ $elapsed_intervals -lt 12 ] && [ $current_gas_price -lt 9000000000 ]; }; then
        # Between 6 and 12 elapsed intervals (30 minutes to 1 hour), if gas price is below 9 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 12 ] && [ $elapsed_intervals -lt 24 ] && [ $current_gas_price -lt 24000000000 ]; }; then
        # Between 12 and 24 elapsed intervals (1 to 2 hours), if gas price is below 24 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 24 ] && [ $elapsed_intervals -lt 48 ] && [ $current_gas_price -lt 48000000000 ]; }; then
        # Between 24 and 48 elapsed intervals (2 to 4 hours), if gas price is below 48 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 48 ] && [ $elapsed_intervals -lt 96 ] && [ $current_gas_price -lt 96000000000 ]; }; then
        # Between 48 and 96 elapsed intervals (4 to 8 hours), if gas price is below 96 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 96 ] && [ $current_gas_price -lt 192000000000 ]; }; then
        # After 96 elapsed intervals (8 hours), if gas price is below 192 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    fi

    elapsed_intervals=$((elapsed_intervals + 1))

    echo "Sleeping $sleep_time seconds (($((sleep_time / 60)) minutes))"
    sleep "$sleep_time"
done
