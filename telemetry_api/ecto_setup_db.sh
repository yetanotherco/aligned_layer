#!/bin/bash

source .env

# Add new environment variables here
env_vars=(
  "ENVIRONMENT"
  "ALIGNED_CONFIG_FILE"
  "OPERATOR_FETCHER_WAIT_TIME_MS"
)

for var in "${env_vars[@]}"; do
  export "$var=${!var}"
done

mix deps.get

mix compile --force #force recompile to get the latest .env values

mix ecto.create
mix ecto.migrate
