
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

2. Compile and run aggregator (this might take long as it uses more aggressive optimization leves):
```shell 
cd aggregation-mode
# This will compile with a mock prover
cargo run --release
# This will run the prover (requires powerful machine)
cargo build --release --features prove
```
