#!/bin/bash

BASE_URL=https://dashboard.tensordock.com/api/v2

echo "Starting GPU Server..."

curl -X GET "$BASE_URL/api/v2/instances" \
    -H "Authorization: Bearer $API_KEY" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json"

echo "GPU Server started successfully."
