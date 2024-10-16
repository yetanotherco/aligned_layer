#!/bin/bash

source .env

# Add new environment variables here
env_vars=(
  "ENVIRONMENT"
  "RPC_URL"
  "PHX_HOST"
  "DB_NAME"
  "DB_USER"
  "DB_PASS"
  "DB_HOST"
  "ALIGNED_CONFIG_FILE"
  "DEBUG_ERRORS"
  "TRACKER_API_URL"
  "MAX_BATCH_SIZE"
)

for var in "${env_vars[@]}"; do
  export "$var=${!var}"
done

mix compile --force #force recompile to get the latest .env values

elixir --sname explorer -S mix phx.server
