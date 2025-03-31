
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

3. Start proof aggregator:
```shell 
# This will not run a real prover but a mocked
make start_proof_aggregator_local
# This will run an actual prover (requires powerful machine)
make start_proof_aggregator_local_with_proving
```

Note: it might take a while to compile as it uses more aggressive optimization levels.
