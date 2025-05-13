#!/bin/bash

# ENV_VARIABLES
#
# - CONTRACT_ADDRESS
#   - Holesky:  0xe84CD4084d8131841CE6DC265361f81F4C59a1d4
#   - Stage:    0x7Eace34A8d4C4CacE633946C6F7CF4BeF3F33513
# - AGGREGATED_PROOF_VERIFIED_TOPIC (0xfe3e9e971000ab9c80c7e06aba2933aae5419d0e44693e3046913e9e58053f62)
# - RPC_URL
# - LOGS_BLOCK_RANGE (25hs -> 7500 blocks)
# - SLEEP_TIME
# - PAGER_DUTY_KEY
# - PAGER_DUTY_EMAIL
# - PAGER_DUTY_SERVICE_ID
# - SLACK_WEBHOOK_URL
# - TELEGRAM_CHAT_ID
# - TELEGRAM_BOT_TOKEN

# Load env file from $1 path
source "$1"

# Function to send slack message
# @param message
function send_slack_message() {
  . alerts/slack.sh "$1"
}

# Function to send telegram message
# @param message
function send_telegram_message() {
  . alerts/telegram.sh "$1"
}

# Function to send PagerDuty alert
# @param message
function send_pagerduty_alert() {
  . alerts/pagerduty.sh "$1"
}

# Flags to avoid sending multiple alerts
no_new_aggregation_alert=false

while :
do
  last_block=$(cast block --rpc-url $RPC_URL -f number)
  printf "Last block: %s\n" $last_block

  from_block=$(($last_block - $LOGS_BLOCK_RANGE))

  new_aggregated_proofs_logs=$(cast logs --rpc-url $RPC_URL --from-block $from_block --address $CONTRACT_ADDRESS $AGGREGATED_PROOF_VERIFIED_TOPIC)
  if [ -z "$new_aggregated_proofs_logs" ]; then
    printf "No new aggregated proofs logs found\n"
    if [ "$no_new_aggregation_alert" = false ]; then
      message="🚨 $NETWORK ALERT Aggregation Mode: No new aggregated proofs since block $from_block"
      printf "$message\n"
      send_slack_message "$message"
      send_telegram_message "$message"
      send_pagerduty_alert "$message"
    fi
    no_new_aggregation_alert=true
  else
    printf "New aggregated proofs logs found\n"
    if [ "$no_new_aggregation_alert" = true ]; then
      message="🟩 $NETWORK INFO Aggregation Mode: Aggregated proofs creation resumed since block $from_block"
      printf "$message\n"
      send_slack_message "$message"
      send_telegram_message "$message"
    fi
    no_new_aggregation_alert=false
  fi

  sleep $SLEEP_TIME
done
