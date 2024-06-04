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

while :
do
  last_block=$(cast block --rpc-url $RPC_URL -f number)
  printf "Last block: %s\n" $last_block

  from_block=$(($last_block - 10))

  new_batch_logs=$(cast logs --rpc-url $RPC_URL --from-block $from_block --address $CONTRACT_ADDRESS $NEW_BATCH_TOPIC)
  if [ -z "$new_batch_logs" ]; then
    printf "No new batches logs found\n"
    send_slack_message "🚨 ALERT: No new batches in Service Manager since block $from_block"
  fi

  verified_batch_logs=$(cast logs --rpc-url $RPC_URL --from-block $from_block --address $CONTRACT_ADDRESS $VERIFIED_BATCH_TOPIC)
  if [ -z "$verified_batch_logs" ]; then
    printf "No verified batches logs found\n"
    send_slack_message "🚨 ALERT: No new verified batches in Service Manager since block $from_block"
  fi

  sleep 100
done
