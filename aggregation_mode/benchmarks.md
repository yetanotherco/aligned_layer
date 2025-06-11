# Aggregation Mode Benchmark Results

## Machine Specifications

-   **GPU**: NVIDIA RTX 3090 (24GB VRAM)
-   **RAM**: 32GB
-   **CPU**: AMD EPYC 7443 (16-Core)
-   **Operating System**: Ubuntu

## Benchmark Setup

-   **Total Proofs**: 3968
-   **Proofs per Chunk**: 128
-   **Total Chunks**: 32

### Notes

-   The total number of proofs (3968) is the **maximum that can be aggregated in a single run**, limited by blob capacity.
-   Increasing the **proofs per chunk** generally improves performance, but requires **more powerful hardware** to avoid out of memory.
-   In this benches the Aligned infrastructure was setup locally in the same machine using `ethereum-package`.

## Reproduce it

The step by step to run the benchmarks:

1. Deploy aligned infrastructure locally with `ethereum-package`, see the [guide here](https://github.com/yetanotherco/aligned_layer/blob/testnet/docs/0_internal/ethereum_package.md).
2. Fund a wallet on aligned, for example with rich account number 7:

```shell
# Install aligned cli
make aligned_install_compiling
```

```shell
aligned deposit-to-batcher \
    --network devnet \
    --private_key 0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356 \
    --amount 1ether
```

3. Send `3968` fibonacci proofs for `Risc0` and `Sp1`:

```shell
## Send SP1 Proofs
aligned submit \
    --proving_system SP1 \
    --proof scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof \
    --vm_program scripts/test_files/sp1/sp1_fibonacci_5_0_0.elf \
    --repetitions 3968 \
    --private_key 0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356 \
    --instant_fee_estimate \
    --network devnet \
    --random_address

## Send Risc0 Proofs
aligned submit \
    --proving_system Risc0 \
    --proof scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_0.proof \
    --vm_program scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id_2_0.bin \
    --public_input scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_0.pub \
    --repetitions 3968 \
    --private_key 0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356
```

4. Modify the `proofs_per_chunk` in `config-files/config-proof-aggregator-ethereum-package.yaml` to `128`.
5. Run the aggregator with time:

```shell
time make start_proof_aggregator_gpu_ethereum_package AGGREGATOR=sp1
time make start_proof_aggregator_gpu_ethereum_package AGGREGATOR=risc0
```

---

## RISC Zero (Risc0)

-   **Start Time**: 14:28:36
-   **Aggregation Start**: 14:33:32
-   **End Time**: 16:46:28

**Performance Summary:**

-   **Verification Time**: **4 minutes 56 seconds**
-   **Aggregation Time**: **132.93 minutes**
-   **Total Time**: **137.87 minutes**
-   **Aggregation time per proof**: **2 seconds**
-   **Total time per proof**: **2,1 seconds**

---

## SP1

-   **Start Time**: 21:52:55
-   **Aggregation Start**: 22:26:40
-   **End Time**: 23:24:42

**Performance Summary:**

-   **Verification Time**: **33 minutes 45 seconds**
-   **Aggregation Time**: **31 minutes 47 seconds**
-   **Total Time**: **92 minutes**
-   **Aggregation time per proof**: **0,5 seconds**
-   **Total time per proof**: **1,4 seconds**

---

## Comparison Summary

|                            | Risc0        | SP1           |
| -------------------------- | ------------ | ------------- |
| Verification Time          | 4 min 56 sec | 33 min 45 sec |
| Aggregation Time           | 132.93 min   | 31 min 47 sec |
| Total Time                 | 137.87 min   | 92 min        |
| Aggregation Time per Proof | 2 seconds    | 0,5 seconds   |
| Total Time per Proof       | 2,1 seconds  | 1,4 seconds   |

> **Note**:
> SP1 longer verification time is due to the big overhead in setting up the prover client to verify the proof when it is fetched.
