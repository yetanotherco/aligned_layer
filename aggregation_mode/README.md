# Aligned aggregation mode

## How to run it locally

1. Deploy aligned contracts on anvil:

```shell
make anvil_deploy_risc0_contracts
make anvil_deploy_sp1_contracts
make anvil_deploy_aligned_contracts
```

2. Start anvil:

```shell
make anvil_start
```

3. Start batcher

```shell
make start_batcher_local
```

4. Send SP1/Risc0 proofs:

```shell
make batcher_send_sp1_burst
make batcher_send_risc0_burst
```

Notes:

-   For SP1 only `compressed` proofs are supported
-   For Risc0 both `succinct` and `composite` proofs are supported.

5. Start proof aggregator:

```shell
# This will not run a real prover but a mocked one see below to run a real prover
make start_proof_aggregator_dev AGGREGATOR="sp1|risc0"
```

or

```shell
make start_proof_aggregator AGGREGATOR="sp1|risc0"
```

Notes:

-   Stark2Snark is only supported for x86 architecture in Risc0, so you won't be able to run the risc0 aggregator on Apple Silicon.
-   Proving can be quite slow without GPUs, to activate gpu run:

```shell
make start_proof_aggregator_gpu AGGREGATOR="sp1|risc0"
```

### Check the logs

1. Get latest aggregated proof:

```shell
cast call 0xcbEAF3BDe82155F56486Fb5a1072cb8baAf547cc "currentAggregatedProofNumber()" --rpc-url http://localhost:8545
```

2. Get aggregated proof info:

```shell
cast call 0xcbEAF3BDe82155F56486Fb5a1072cb8baAf547cc "getAggregatedProof(uint64)(uint8,bytes32,bytes32)" <AGG_PROOF_NUMBER>  --rpc-url http://localhost:8545
```
