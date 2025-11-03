#!/bin/bash

# ENV VARIABLES:
# - REPETITIONS
# - EXPLORER_URL
# - SENDER_ADDRESS
# - RPC_URL
# - EXPLORER_URL
# - NETWORK
# - PRIVATE_KEY
# - VERIFICATION_WAIT_TIME
# - LOGS_BLOCK_RANGE
# - PAGER_DUTY_KEY
# - PAGER_DUTY_EMAIL
# - PAGER_DUTY_SERVICE_ID
# - SLACK_WEBHOOK_URL

# Load env file from $1 path
source "$1"

# Determine log file name based on current date
LOG_FILE="./alerts/notification_logs/log_$(date +"%Y_%m_%d").txt"

batches=0
submitted_total=0
verified_total=0
unverified_total=0
eth_total="0"
usd_total="0"

# Read the log file entries and generate a summary
if [[ -f "$LOG_FILE" ]]; then
    while IFS= read -r line; do
        case "$line" in
        *"SUCCESS:"*)
            batches=$((batches + 1))

            proofs_submitted=$(printf '%s\n' "$line" \
                | grep -oE '[0-9]+ proofs submitted' \
                | head -1 \
                | cut -d' ' -f1)
            if [[ -n "$proofs_submitted" ]]; then
                submitted_total=$((submitted_total + proofs_submitted))
            fi

            proofs_verified=$(printf '%s\n' "$line" \
                | grep -oE '\([0-9]+ sent\)' \
                | grep -oE '[0-9]+' \
                | head -1)
            if [[ -n "$proofs_verified" ]]; then
                verified_total=$((verified_total + proofs_verified))
            fi

            eth_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*Spent \([0-9.]*\) ETH.*/\1/p')
            if [[ -n "$eth_spent" ]]; then
                eth_total=$(echo "$eth_total + $eth_spent" | bc -l)
            fi

            usd_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*(\$ *\([0-9.]*\)).*/\1/p')
            if [[ -n "$usd_spent" ]]; then
                usd_total=$(echo "$usd_total + $usd_spent" | bc -l)
            fi
            ;;
        *"FAILURE:"*)
            batches=$((batches + 1))

            proofs_submitted=$(printf '%s\n' "$line" \
                | grep -oE '[0-9]+ proofs submitted' \
                | head -1 \
                | cut -d' ' -f1)
            if [[ -n "$proofs_submitted" ]]; then
                submitted_total=$((submitted_total + proofs_submitted))
            fi

            proofs_unverified=$(printf '%s\n' "$line" \
                | grep -oE '\([0-9]+ sent\)' \
                | grep -oE '[0-9]+' \
                | head -1)
            if [[ -n "$proofs_verified" ]]; then
                unverified_total=$((verified_total + proofs_verified))
            fi

            eth_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*Spent \([0-9.]*\) ETH.*/\1/p')
            if [[ -n "$eth_spent" ]]; then
                eth_total=$(echo "$eth_total + $eth_spent" | bc -l)
            fi

            usd_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*(\$ *\([0-9.]*\)).*/\1/p')
            if [[ -n "$usd_spent" ]]; then
                usd_total=$(echo "$usd_total + $usd_spent" | bc -l)
            fi
        esac
    done < "$LOG_FILE"

    summary=$(
        printf "Daily Proof Submission Summary\n"
        printf "From %s 00:00 to %s 23:59\n" "$(date +'%d-%m-%Y')" "$(date +'%d-%m-%Y')"
        echo "----------------------------------------------------"
        printf "Processed batches:      %d\n" "$batches"
        printf "Proofs submitted:       %d\n" "$submitted_total"
        printf "Proofs verified :       %d\n" "$verified_total"
        printf "Proofs not verified:    %d\n" "$unverified_total"
        printf "Total spent (ETH):     %.12f ETH\n" "$eth_total"
        printf "Total spent (USD):     $ %.2f\n" "$usd_total"
        echo "----------------------------------------------------"
    )

    echo "$summary"

    # Send the summary to Slack
    if [[ -n "$SLACK_WEBHOOK_URL" ]]; then
        safe_summary=$(printf '%s\n' "$summary" | sed 's/"/\\"/g')
        curl -s -X POST -H 'Content-type: application/json' \
        --data "{\"text\":\"$safe_summary\"}" \
        "$SLACK_WEBHOOK_URL" >/dev/null 2>&1
    fi
else
    echo "Proof Submission Summary - $(date +'%Y-%m-%d %H:%M:%S')"
    echo "----------------------------------------"
    echo "No log file found for today: $LOG_FILE"
    echo "----------------------------------------"
fi
