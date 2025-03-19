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

if [ "$#" -eq 0 ]; then
    echo "Error, No arguments provided."
    echo "Try running the make target with FROM_BLOCK=<n>"
    exit 1
elif [ "$#" -eq 1 ]; then
    # argument provided, use it
    FROM=$1
else
    echo "Please provide 1 arguments."
    exit 1
fi

echo "Running fetch_old_operators.sh from block: $FROM"

mix compile --force #force recompile to get the latest .env values

iex --sname fetch_old_operators --remsh explorer@$ELIXIR_HOSTNAME -S mix run -e "Scripts.FetchOldOperatorsStrategiesRestakes.run($FROM)"
