#!/bin/bash

# Load env file from first parameter
# Env variables:
#   - PROMETHEUS_URL
#   - SYSTEMD_SERVICE
#   - PROMETHEUS_COUNTER
#   - PROMETHEUS_BOT
#   - PROMETHEUS_INTERVAL
#   - SLACK_WEBHOOK_URL
source $1

# Function to send slack message
# @param message
function send_slack_message() {
  curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"$1\"}" \
    $SLACK_WEBHOOK_URL
}

# Get rate from prometheus
rate=$(curl -gs 'http://'$PROMETHEUS_URL'/api/v1/query?query=floor(increase('$PROMETHEUS_COUNTER'{bot="'$PROMETHEUS_BOT'"}['$PROMETHEUS_INTERVAL']))' | jq '.data.result[0].value[1]')

echo "$(date): tasks created in the last $PROMETHEUS_INTERVAL: $rate"

# Check if rate is 0
if [ "$rate" = \"0\" ]; then
  # Restart systemd service
  echo "$(date): restarting $SYSTEMD_SERVICE"
  sudo systemctl restart $SYSTEMD_SERVICE
  message="$(date): $SYSTEMD_SERVICE restarted by watchdog"
  echo $message
  send_slack_message "$message"
fi
