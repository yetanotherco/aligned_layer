curl --request POST -s \
  --url https://api.pagerduty.com/incidents \
  --header 'Accept: application/json' \
  --header "Authorization: Token token=$PAGER_DUTY_KEY" \
  --header 'Content-Type: application/json' \
  --header "From: $PAGER_DUTY_EMAIL"\
  --data "{
  \"incident\": {
    \"type\": \"incident\",
    \"title\": \"$1\",
    \"service\": {
      \"id\": \"$PAGER_DUTY_SERVICE_ID\",
      \"type\": \"service_reference\"
    }
  }
}"
