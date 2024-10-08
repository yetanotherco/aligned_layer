#!/bin/bash

source .env

if [ -z "$1" ]; then
  echo "Usage: ./explorer/create_migration.sh MIGRATION_NAME"
  exit 1
fi

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
)

for var in "${env_vars[@]}"; do
  export "$var=${!var}"
done

mix ecto.gen.migration $1
