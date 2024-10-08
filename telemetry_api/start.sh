#!/bin/bash

source .env

# Add new environment variables here
env_vars=(
  "ENVIRONMENT"
  "ALIGNED_CONFIG_FILE"
)

for var in "${env_vars[@]}"; do
  export "$var=${!var}"
done

mix compile --force #force recompile to get the latest .env values

elixir --sname telemetry -S mix phx.server
