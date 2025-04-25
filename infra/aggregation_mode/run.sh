#!/bin/bash

SLEEP=5

echo "Starting Aggregation Mode in $SLEEP seconds..."
sleep $SLEEP

echo "Starting SP1 Aggregation Mode..."
AGGREGATOR=sp1 SP1_PROVER=cuda /home/user/.cargo/bin/proof_aggregator /home/user/config/config-proof-aggregator-sp1.yaml
docker stop $(docker ps -a -q) ## stop all containers
echo "SP1 Aggregation Mode finished"

echo "Starting Risc0 Aggregation Mode..."
AGGREGATOR=risc0 /home/user/.cargo/bin/proof_aggregator /home/user/config/config-proof-aggregator-risc0.yaml
echo "Risc0 Aggregation Mode finished"
