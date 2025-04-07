
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

or

```shell
make start_proof_aggregator_local_with_proving
```

Note: Proving can be quite slow without GPUs

### Check the logs

1. Get latest aggregated proof:

```shell
cast call 0xcbEAF3BDe82155F56486Fb5a1072cb8baAf547cc "currentAggregatedProofNumber()" --rpc-url http://localhost:8545
```

2. Get aggregated proof info:

```shell
cast call 0xcbEAF3BDe82155F56486Fb5a1072cb8baAf547cc "getAggregatedProof(uint64)(uint8,bytes32,bytes32)" <AGG_PROOF_NUMBER>  --rpc-url http://localhost:8545
```
