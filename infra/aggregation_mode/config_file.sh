#!/bin/bash

# Check if template file path is provided as argument
if [ $# -ne 1 ]; then
    echo "Usage: $0 <template_file_path>"
    exit 1
fi

TEMPLATE_FILE="$1"

# Verify template file exists
if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "Error: Template file '$TEMPLATE_FILE' not found"
    exit 1
fi

# Create temporary file by copying the template
TEMP_FILE=$(mktemp)
cp "$TEMPLATE_FILE" "$TEMP_FILE"

# Function to prompt for input and replace placeholder
prompt_and_replace() {
    local placeholder=$1
    local description=$2

    read -p "Enter $description: " value
    sed -i "s|$placeholder|$value|g" "$TEMP_FILE"
}

# Prompt for each placeholder found in the template
prompt_and_replace "<aligned_service_manager_address>" "Aligned Service Manager Address"
prompt_and_replace "<proof_aggregation_service_address>" "Proof Aggregation Service Address"
prompt_and_replace "<eth_rpc_url>" "Ethereum RPC URL"
prompt_and_replace "<eth_ws_url>" "Ethereum WebSocket URL"
prompt_and_replace "<private_key_store_path>" "ECDSA Private Key Store Path (~/.keystores/proof_aggregation.keystore)"
prompt_and_replace "<private_key_store_password>" "ECDSA Private Key Store Password"

# Create destination directory if it doesn't exist
mkdir -p /home/user/config

# Copy the completed file to destination
cp "$TEMP_FILE" $HOME/config/config-proof-aggregator.yaml

# Clean up temporary file
rm "$TEMP_FILE"

echo "Configuration file has been created and copied to $HOME/config/config-proof-aggregator.yaml"
