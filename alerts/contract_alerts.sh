#!/bin/bash

# Load env file from $1 path
source $1

# Function to send slack message
# @param message
function send_slack_message() {
  curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"$1\"}" \
    $SLACK_WEBHOOK_URL
}

# Flags to avoid sending multiple alerts
no_new_batches_alert=false
no_verified_batches_alert=false

while :
do
  last_block=$(cast block --rpc-url $RPC_URL -f number)
  printf "Last block: %s\n" $last_block

  from_block=$(($last_block - 25))

  new_batch_logs=$(cast logs --rpc-url $RPC_URL --from-block $from_block --address $CONTRACT_ADDRESS $NEW_BATCH_TOPIC)
  if [ -z "$new_batch_logs" ]; then
    printf "No new batches logs found\n"
    if [ "$no_new_batches_alert" = false ]; then
      send_slack_message "🚨 ALERT: No new batches in Service Manager since block $from_block"
    fi
    no_new_batches_alert=true
  else
    printf "New batches logs found\n"
    if [ "$no_new_batches_alert" = true ]; then
      send_slack_message "🟩 INFO: Batches creation resumed in Service Manager since block $from_block"
    fi
    no_new_batches_alert=false
  fi

  verified_batch_logs=$(cast logs --rpc-url $RPC_URL --from-block $from_block --address $CONTRACT_ADDRESS $VERIFIED_BATCH_TOPIC)
  if [ -z "$verified_batch_logs" ]; then
    printf "No verified batches logs found\n"
    if [ "$no_verified_batches_alert" = false ]; then
      send_slack_message "🚨 ALERT: No verified batches in Service Manager since block $from_block"
    fi
    no_verified_batches_alert=true
  else
    printf "Verified batches logs found\n"
    if [ "$no_verified_batches_alert" = true ]; then
      send_slack_message "🟩 INFO: Batches verification resumed in Service Manager since block $from_block"
    fi
    no_verified_batches_alert=false
  fi

  sleep 300
done
