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

DATE=$(date -d "yesterday" +"%Y_%m_%d")

# Determine log file name based on current date
LOG_FILE="./alerts/notification_logs/log_$DATE.txt"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"
cd ..

batches=0
submitted_total=0
submitted_by_aligned=0
verified_total=0
unverified_total=0
eth_by_aligned="0"
usd_by_aligned="0"
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
                verified_total=$((verified_total + proofs_submitted))
            fi

            proofs_submitted_by_aligned=$(printf '%s\n' "$line" \
                | grep -oE '\([0-9]+ sent\)' \
                | grep -oE '[0-9]+' \
                | head -1)
            if [[ -n "$proofs_submitted_by_aligned" ]]; then
                submitted_by_aligned=$((submitted_by_aligned + proofs_submitted_by_aligned))
            fi

            eth_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*Spent \([0-9.]*\) ETH.*/\1/p')
            if [[ -n "$eth_spent" ]]; then
                eth_total=$(echo "$eth_total + $eth_spent" | bc -l)
                eth_by_aligned=$(echo "$eth_by_aligned + $eth_spent / $proofs_submitted * $proofs_submitted_by_aligned" | bc -l)
            fi

            usd_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*(\$ *\([0-9.]*\)).*/\1/p')
            if [[ -n "$usd_spent" ]]; then
                usd_total=$(echo "$usd_total + $usd_spent" | bc -l)
                usd_by_aligned=$(echo "$usd_by_aligned + $usd_spent / $proofs_submitted * $proofs_submitted_by_aligned" | bc -l)
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
                unverified_total=$((unverified_total + proofs_submitted))
            fi

            proofs_submitted_by_aligned=$(printf '%s\n' "$line" \
                | grep -oE '\([0-9]+ sent\)' \
                | grep -oE '[0-9]+' \
                | head -1)
            if [[ -n "$proofs_submitted_by_aligned" ]]; then
                submitted_by_aligned=$((submitted_by_aligned + proofs_submitted_by_aligned))
            fi

            eth_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*Spent \([0-9.]*\) ETH.*/\1/p')
            if [[ -n "$eth_spent" ]]; then
                eth_total=$(echo "$eth_total + $eth_spent" | bc -l)
                eth_by_aligned=$(echo "$eth_by_aligned + $eth_spent / $proofs_submitted * $proofs_submitted_by_aligned" | bc -l)
            fi

            usd_spent=$(printf '%s\n' "$line" \
                | sed -n 's/.*(\$ *\([0-9.]*\)).*/\1/p')
            if [[ -n "$usd_spent" ]]; then
                usd_total=$(echo "$usd_total + $usd_spent" | bc -l)
                usd_by_aligned=$(echo "$usd_by_aligned + $usd_spent / $proofs_submitted * $proofs_submitted_by_aligned" | bc -l)
            fi
        esac
    done < "$LOG_FILE"

    summary=$(
        printf "Daily Proof Submission Summary\n"
        printf "From %s 00:00 to %s 23:59\n" "$DATE" "$DATE"
        echo "----------------------------------------------------"
        printf "Processed batches:              %d\n" "$batches"
        printf "Total Proofs submitted:         %d\n" "$submitted_total"
        printf "Total Proofs verified:          %d\n" "$verified_total"
        printf "Total Proofs not verified:      %d\n" "$unverified_total"
        printf "Submitted by Aligned:           %d\n" "$submitted_by_aligned"
        printf "Submitted by 3rd parties:       %d\n" "$((submitted_total - submitted_by_aligned))"
        echo "----------------------------------------------------"
        printf "Spent by Aligned (ETH):         %.12f ETH\n" "$eth_by_aligned"
        printf "Spent by Aligned (USD):         $ %.2f\n" "$usd_by_aligned"
        printf "Spent by 3rd parties (ETH):     %.12f ETH\n" "$(echo "$eth_total - $eth_by_aligned" | bc -l)"
        printf "Spent by 3rd parties (USD):     $ %.2f\n" "$(echo "$usd_total - $usd_by_aligned" | bc -l)"
        printf "Total spent (ETH):              %.12f ETH\n" "$eth_total"
        printf "Total spent (USD):              $ %.2f\n" "$usd_total"
        echo "----------------------------------------------------"
    )

    echo "$summary"

    # Send the summary to Slack
    if [[ -n "$SLACK_WEBHOOK_URL" ]]; then
        safe_summary=$(printf '%s\n' "$summary" | sed 's/"/\\"/g')
        curl -s -X POST -H 'Content-type: application/json' \
        --data "{\"text\":\"\`\`\`$safe_summary\`\`\`\"}" \
        "$SLACK_WEBHOOK_URL" >/dev/null 2>&1
    fi
else
    echo "Proof Submission Summary - $DATE"
    echo "----------------------------------------"
    echo "No log file found for today: $LOG_FILE"
    echo "----------------------------------------"
fi
