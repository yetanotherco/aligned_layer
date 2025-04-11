#!/bin/bash

BASE_URL=https://dashboard.tensordock.com/api/v2

echo "Stopping GPU Server..."

sleep 120

curl -X POST "https://dashboard.tensordock.com/api/v2/instances/41ef030d-22f2-4e5e-852e-cb21982271c0/stop" \
    -H "Authorization: Bearer fwf7iWW3PEQXqUTFGPJvlmQ8lny3MIUg" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    --data-urlencode 'disassociate_resources=true' \
    --max-time 60

echo "GPU Server stopped successfully."
