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

# Get batcher queue len
function get_queue_length() {
    queue_len=$(curl $BATCHER_METRICS_URL -s | grep queue_len | awk '{print $2}' | tail -n 1)
    echo $queue_len
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

    queue_len=$(get_queue_length)
    if [ "$queue_len" -eq 0 ]; then
        echo "There are no pending proofs in the queue, skipping this pass."
        echo "Sleeping $sleep_time seconds (($((sleep_time / 60)) minutes))"
        sleep "$sleep_time"
        continue
    fi

    # In case current and gas price meet the criteria, send a proof and reset counter
    if { [ $elapsed_intervals -ge 1 ] && [ $elapsed_intervals -lt 3 ] && [ $current_gas_price -lt 500000000 ]; }; then
        # At 1 tick (5 minutes), if gas price is below 0.5 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 3 ] && [ $elapsed_intervals -lt 6 ] && [ $current_gas_price -lt 1500000000 ]; }; then
        # At 3 ticks (15 minutes), if gas price is below 1.5 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 6 ] && [ $elapsed_intervals -lt 12 ] && [ $current_gas_price -lt 3000000000 ]; }; then
        # At 6 ticks (30 minutes), if gas price is below 3 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 12 ] && [ $elapsed_intervals -lt 24 ] && [ $current_gas_price -lt 6000000000 ]; }; then
        # At 12 ticks (1 hour), if gas price is below 6 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 24 ] && [ $elapsed_intervals -lt 48 ] && [ $current_gas_price -lt 12000000000 ]; }; then
        # At 24 ticks (2 hours), if gas price is below 12 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 48 ] && [ $elapsed_intervals -lt 96 ] && [ $current_gas_price -lt 24000000000 ]; }; then
        # At 48 ticks (4 hours), if gas price is below 24 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 96 ] && [ $elapsed_intervals -lt 192 ] && [ $current_gas_price -lt 48000000000 ]; }; then
        # At 96 ticks (8 hours), if gas price is below 48 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    elif { [ $elapsed_intervals -ge 192 ] && [ $current_gas_price -lt 96000000000 ]; }; then
        # At 192 ticks (16 hours), if gas price is below 96 gwei, send a proof
        message="Sending proof at $elapsed_intervals with gas price $current_gas_price wei"
        echo "$message"
        send_proof_background
        elapsed_intervals=0
    fi

    elapsed_intervals=$((elapsed_intervals + 1))

    echo "Sleeping $sleep_time seconds (($((sleep_time / 60)) minutes))"
    sleep "$sleep_time"
done
