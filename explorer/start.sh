#!/bin/bash

source .env

export ENVIRONMENT=$ENVIRONMENT
export RPC_URL=$RPC_URL

mix compile --force #force recompile to get the latest .env values

mix phx.server
