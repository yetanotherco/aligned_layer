# Function to send slack message
# @param message
curl -X POST -H 'Content-type: application/json' \
  --data "{\"text\":\"$1\"}" \
  $SLACK_WEBHOOK_URL