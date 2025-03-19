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
    echo "Try running the make target with FROM_BLOCK=<n> TO_BLOCK=<m>"
    exit 1
elif [ "$#" -eq 2 ]; then
    # Two arguments provided, use them
    FROM=$1
    TO=$2
else
    echo "Please provide 2 arguments."
    echo "Try running the make target with FROM_BLOCK=<n> TO_BLOCK=<m>"
    exit 1
fi

echo "Running fetch_old_batches.sh from block: $FROM to block: $TO"


mix compile --force #force recompile to get the latest .env values

iex --sname fetch_old_batches --remsh explorer@$ELIXIR_HOSTNAME -S mix run -e "Scripts.FetchOldBatches.run($FROM, $TO)"
