
# Aligned aggregation mode

## How to run it locally

1. Deploy aligned contracts on anvil:
```shell
make deploy_aligned_contracts
```

2. Start anvil:
```shell
make anvil_start_with_block_time
```

3. Start batcher
```shell
make start_batcher_local
```

4. Send SP1 proofs:
```shell
make batcher_send_sp1_burst
```

5. Start proof aggregator:
```shell 
# This will not run a real prover but a mocked one see below to run a real prover
make start_proof_aggregator_local
```
Note: it might take a while to compile as it uses more aggressive optimization levels.

You should it will fetch the new batch logs from the BatcherService and aggregate the compressed SP1 proofs from them.

### Run it with proving

By default, on dev environments, the proving is mocked and the ProofAggregationService contract skips verification as proves are mocked. To run the service with proving you need to run change the commands on step `1.` and `4.`:

1. Start anvil with verification activated:
```shell
make anvil_start_with_verification
```

4. Start proof aggregator with proving:
```shell
make start_proof_aggregator_local_with_proving
```

Note: Unless you constraint yourself to a few proofs, this requires a powerful machine with GPU.


### Check the logs

1. Get latest aggregated proof:
```shell
cast call 0xcbEAF3BDe82155F56486Fb5a1072cb8baAf547cc "currentAggregatedProofNumber()" --rpc-url http://localhost:8545
```

2. Get aggregated proof info:
```shell
cast call 0xcbEAF3BDe82155F56486Fb5a1072cb8baAf547cc "getAggregatedProof(uint64)(uint8,bytes32,bytes32)" <AGG_PROOF_NUMBER>  --rpc-url http://localhost:8545
```
