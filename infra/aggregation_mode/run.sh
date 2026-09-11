#!/bin/bash

SLEEP=60

echo "Starting Aggregation Mode in $SLEEP seconds..."
sleep $SLEEP

echo "Starting SP1 Aggregation Mode..."
AGGREGATOR=sp1 SP1_PROVER=cuda /home/user/.cargo/bin/proof_aggregator_gpu /home/user/config/config-proof-aggregator-sp1.yaml
docker stop $(docker ps -a -q) ## stop all containers
echo "SP1 Aggregation Mode finished"

echo "Starting Risc0 Aggregation Mode..."
AGGREGATOR=risc0 /home/user/.cargo/bin/proof_aggregator_gpu /home/user/config/config-proof-aggregator-risc0.yaml
echo "Risc0 Aggregation Mode finished"

# Aggregation finished: power off the machine so Paperspace stops billing it.
# The GitHub Actions workflow (.github/workflows/aggregation_mode.yml) starts the
# machine again on the next scheduled run, and aggregation_mode.service runs this
# script automatically on boot.
echo "Aggregation finished, shutting down the machine in 1 minute..."
sleep $SLEEP
sudo shutdown -h now
