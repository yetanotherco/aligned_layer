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
    if { [ $elapsed_intervals -ge 264 ] && [ $current_gas_price -lt 10000000000 ]; }; then
        # At 264 tick (22 hours), if gas price is below 10 gwei, send a proof
        # It is set to 22 hours instead of 24 to add a buffer in case of high gas prices
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    fi

    elapsed_intervals=$((elapsed_intervals + 1))

    echo "Sleeping $sleep_time seconds (($((sleep_time / 60)) minutes))"
    sleep "$sleep_time"
done
