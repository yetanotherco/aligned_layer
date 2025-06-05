# Aggregation Mode Benchmark Results

## Machine Specifications

-   **GPU**: NVIDIA RTX 3090 (24GB VRAM)
-   **RAM**: 32GB
-   **CPU**: AMD EPYC 7443 (24-Core)
-   **Operating System**: Ubuntu

## Benchmark Setup

-   **Total Proofs**: 3968
-   **Proofs per Chunk**: 128
-   **Total Chunks**: 32

---

## RISC Zero (Risc0)

-   **Start Time**: 14:28:36
-   **Aggregation Start**: 14:33:32
-   **End Time**: 16:46:28

**Performance Summary:**

-   **Verification Time (Start to Aggregation Start)**: **4 minutes 56 seconds**
-   **Aggregation Time (Agg to End)**: **132.93 minutes**
-   **Total Time (Start to End)**: **137.87 minutes**

---

## SP1

-   **Start Time**: 21:52:55
-   **Aggregation Start**: 22:26:40
-   **End Time**: 23:24:42

**Performance Summary:**

-   **Verification Time (Start to Aggregation Start)**: **33 minutes 45 seconds**
-   **Aggregation Time (Agg to End)**: **31 minutes 47 seconds**
-   **Total Time (Start to End)**: **92 minutes**

---

## Comparison Summary

|                   | Risc0        | SP1           |
| ----------------- | ------------ | ------------- |
| Verification Time | 4 min 56 sec | 33 min 45 sec |
| Aggregation Time  | 132.93 min   | 31 min 47 sec |
| Total Time        | 137.87 min   | 92 min        |

> **Note**:  
> SP1 longer verification time is due to the big overhead in setting up the prover client to verify the proof when it is fetched.
