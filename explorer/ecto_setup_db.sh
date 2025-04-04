#!/bin/bash

source .env

export ENVIRONMENT=$ENVIRONMENT
export RPC_URL=$RPC_URL
export PHX_HOST=$PHX_HOST
export DB_NAME=$DB_NAME
export DB_USER=$DB_USER
export DB_PASS=$DB_PASS
export DB_HOST=$DB_HOST
export ALIGNED_CONFIG_FILE=$ALIGNED_CONFIG_FILE
export ALIGNED_PROOF_AGG_CONFIG_FILE=$ALIGNED_PROOF_AGG_CONFIG_FILE
export BEACON_CLIENT=$BEACON_CLIENT

mix deps.get

mix compile --force #force recompile to get the latest .env values

mix ecto.create
mix ecto.migrate
