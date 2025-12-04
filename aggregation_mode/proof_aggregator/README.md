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
make batcher_start_local
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
make proof_aggregator_start_dev AGGREGATOR="sp1|risc0"
```

or

```shell
make proof_aggregator_start AGGREGATOR="sp1|risc0"
```

Notes:

-   Stark2Snark is only supported for x86 architecture in Risc0, so you won't be able to run the risc0 aggregator on Apple Silicon.
-   Proving can be quite slow without GPUs, to activate gpu run:

```shell
make proof_aggregator_start_gpu AGGREGATOR="sp1|risc0"
```

### Check the logs

1. Get latest aggregated proof:

```shell
cast logs 0xc351628EB244ec633d5f21fBD6621e1a683B1181 'AggregatedProofVerified(bytes32,bytes32)' --from-block 0 --to-block latest --rpc-url http://localhost:8545
```

## Compiling programs

Whenever any of the programs change, you must recompile them and update their corresponding program ids in `aggregation_mode/program_ids.json`. To do this, run the following command:

```shell
make proof_aggregator_write_program_ids
```

We are using docker to produce deterministic builds so that the program ids are the same for all systems.

### Updating the verifier program commitment in `AlignedProofAggregationService` contract

If the verifier program commitments have changed, you will also need to add the updated one to the `AlignedProofAggregationService` contract. You can do this by calling the `allowVerifyingProgram` method with the new commitment and the proving system ID as parameters (you can check the last parameter in the ProvingSystemId definition at `contracts/src/core/IAlignedProofAggregationService.sol`).

You can fetch the verifier program commitment values from the following:

- Risc0: Use the value of `risc0_chunk_aggregator_image_id` from `aggregation_mode/program_ids.json`.
- SP1: Use the value of `sp1_chunk_aggregator_vk_hash` from `aggregation_mode/program_ids.json`.
