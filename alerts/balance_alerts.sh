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

# Function to send telegram message
# @param message
function send_telegram_message() {
  curl -s -X POST https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/sendMessage \
      -d chat_id=$TELEGRAM_CHAT_ID \
      -d text="$1" \
      -d disable_notification=true
}

# Flags to avoid sending multiple alerts
balance_alert=false

while :
do
  balance_wei=$(cast call --rpc-url $RPC_URL $PAYMENT_CONTRACT_ADDRESS "user_balances(address)(uint256)" $WALLET_ADDRESS | cut -d' ' -f1)

  balance_eth=$(cast from-wei $balance_wei)

  if [ 1 -eq "$(echo "$balance_eth < $BALANCE_THRESHOLD" | bc)" ]; then
    message="⚠️ WARNING: $WALLET_NAME ($NETWORK) Wallet ($WALLET_ADDRESS) balance ($balance_eth ETH) is below $BALANCE_THRESHOLD ETH"
    printf "$message\n"
    if [ "$balance_alert" = false ]; then
      send_slack_message "$message"
      send_telegram_message "$message"
    fi
    balance_alert=true
  else
    message="🟩 INFO: $WALLET_NAME ($NETWORK) Wallet $WALLET_ADDRESS balance ($balance_eth ETH) is above $BALANCE_THRESHOLD ETH"
    printf "$message\n"
    if [ "$balance_alert" = true ]; then
      send_slack_message "$message"
      send_telegram_message "$message"
    fi
    balance_alert=false
  fi

  sleep 600
done
