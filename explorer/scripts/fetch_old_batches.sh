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

DEFAULT_FROM=1600000
DEFAULT_TO=1700000

if [ "$#" -eq 0 ]; then
    echo "No arguments provided, using default values."
    FROM=$DEFAULT_FROM
    TO=$DEFAULT_TO
elif [ "$#" -eq 2 ]; then
    # Two arguments provided, use them
    FROM=$1
    TO=$2
else
    echo "Please provide either 0 or 2 arguments."
    exit 1
fi

echo "Running fetch_old_batches.sh from block: $FROM to block: $TO"


mix compile --force #force recompile to get the latest .env values

iex --sname fetch_old_batches --remsh explorer@$ELIXIR_HOSTNAME -S mix run -e "Scripts.FetchOldBatches.run($FROM, $TO)"