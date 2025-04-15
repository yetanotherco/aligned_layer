#!/bin/bash

BASE_URL=https://dashboard.tensordock.com/api/v2

echo "Stopping GPU Server..."

sleep 120

curl -X POST "$BASE_URL/instances/$INSTANCE_ID/stop" \
    -H "Authorization: Bearer $API_KEY" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    --data-urlencode 'disassociate_resources=true' \
    --max-time 60

echo "GPU Server stopped successfully."
