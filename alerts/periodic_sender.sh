#!/bin/bash
# This script is responsible for the infinite loop and the sleep between passes.
# It forwards the path of the .env file as the first argument to action.sh

ENV_FILE="$1"

if [[ -z "$ENV_FILE" ]]; then
    echo "Usage: $0 path/to/.env"
    exit 1
fi

# Fetches the current ETH gas price
function fetch_gas_price() {
    # TODO: We should have a second RPC_URL for fetching gas price to avoid being rate limited
    gas_price=$(cast gas-price --rpc-url $RPC_URL)
    echo $gas_price
}

source "$ENV_FILE"

# Each tic lasts for 30 minutes
sleep_time=1800
tic=0

while true; do
    echo "Starting pass #$tic"

    current_gas_price=$(fetch_gas_price)
    echo "Current gas price: $current_gas_price wei"

    # Conditions for sending proofs:
    if { [ $tic -ge 10 ] && [ $tic -lt 14 ] && [ $current_gas_price -lt 2000000000 ]; }; then
        # - Between 10 and 14 tics (5 to 7 hours), if gas price is below 2 gwei, send a proof
        message="Sending proof at tic $tic with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        tic=0  # Reset tic counter after sending a proof
    elif { [ $tic -ge 14 ] && [ $tic -lt 16 ] && [ $current_gas_price -lt 5000000000 ]; }; then
        # - Between 14 and 16 tics (7 to 8 hours), if gas price is below 5 gwei, send a proof
        message="Sending proof at tic $tic with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        tic=0  # Reset tic counter after sending a proof
    elif { [ $tic -ge 16 ] && [ $tic -lt 24 ] && [ $current_gas_price -lt 15000000000 ]; }; then
        # - Between 16 and 24 tics (8 to 12 hours), if gas price is below 15 gwei, send a proof
        message="Sending proof at tic $tic with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        tic=0  # Reset tic counter after sending a proof
    elif { [ $tic -ge 50 ]; }; then
        # - After 50 tics (25 hours), if gas price is below 50 gwei, send a proof
        message="Sending proof at tic $tic with gas price $current_gas_price wei"
        echo "$message"
        ./alerts/sender_with_alert.sh "$ENV_FILE"
        tic=0  # Reset tic counter after sending a proof
    fi

    tic=$((tic + 1))

    echo "Sleeping $sleep_time seconds (($((sleep_time / 60)) minutes))"
    sleep "$sleep_time"
done
