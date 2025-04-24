#!/bin/bash

BASE_URL=https://dashboard.tensordock.com/api/v2

echo "Starting GPU Server..."

curl -X POST "$BASE_URL/instances/$INSTANCE_ID/start" \
    -H "Authorization: Bearer $API_KEY" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    --max-time 60

echo "GPU Server started successfully."
