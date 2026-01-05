#!/bin/bash

cli_bin="${AGG_MODE_CLI_BIN:-agg_mode_cli}"
if ! command -v "$cli_bin" >/dev/null 2>&1; then
  echo "agg_mode CLI not found in PATH. Run: make agg_mode_install_cli"
  exit 1
fi

interval_hours=$INTERVAL_HOURS
network=$NETWORK
private_key=$PRIVATE_KEY
proof_path=$PROOF_PATH
vk_path=$VK_PATH

if [[ -z "$interval_hours" ]]; then
  echo "INTERVAL_HOURS not found"
  exit 1
fi

if [[ -z "$network" ]]; then
  echo "NETWORK not found"
  exit 1
fi

if [[ -z "$private_key" ]]; then
  echo "PRIVATE_KEY not found"
  exit 1
fi

if [[ ! -f "$proof_path" ]]; then
  echo "PROOF_PATH not found: $proof_path"
  exit 1
fi

if [[ ! -f "$vk_path" ]]; then
  echo "VK_PATH key not found: $vk_path"
  exit 1
fi


sleep_seconds=$((interval_hours * 3600))
echo "Sending SP1 proof every ${interval_hours} hour(s) using ${cli_bin}..."

while true; do
  "$cli_bin" submit sp1 \
    --proof "$proof_path" \
    --vk "$vk_path" \
    --private-key "$private_key" \
    --network "$network"
  echo "sleeping for ${sleep_seconds} seconds"
  sleep "$sleep_seconds"
done
