#!/bin/bash

source .env

export ENVIRONMENT=$ENVIRONMENT
export RPC_URL=$RPC_URL
export PHX_HOST=$PHX_HOST
export DB_NAME=$DB_NAME
export DB_USER=$DB_USER
export DB_PASS=$DB_PASS
export DB_HOST=$DB_HOST
export ELIXIR_HOSTNAME=$ELIXIR_HOSTNAME
export ALIGNED_CONFIG_FILE=$ALIGNED_CONFIG_FILE

mix compile --force #force recompile to get the latest .env values

iex --sname fetch_restakeable_strategies --remsh explorer@$ELIXIR_HOSTNAME -S mix run -e "AlignedLayerServiceManager.update_restakeable_stratergies()"
