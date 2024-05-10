#!/bin/bash

source .env

export ENVIRONMENT=$ENVIRONMENT
export RPC_URL=$RPC_URL

mix phx.server
