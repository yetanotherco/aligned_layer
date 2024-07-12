# Getting started!

In this tutorial you will learn how to send your first SP1 proofs to get verified in Aligned in under 3 minutes.

## Quickstart
We will download a previously generated SP1 proof, send it to Aligned for verification, and retrieve the results from Ethereum Holesky testnet.

1. Download and install Aligned to send proofs in the testnet:

```bash
curl -L https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/install_aligned.sh | bash
```

2. Run the ```source``` command that should appear in the shell

3. Download the example SP1 proof file together with the ELF file of the proved program using:

```bash
curl -L https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/get_proof_test_files.sh | bash
```

4. Send the proof to be verified in Aligned with 

```bash
rm -rf ~/.aligned/aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof ~/.aligned/test_files/sp1_fibonacci.proof \
--vm_program ~/.aligned/test_files/sp1_fibonacci.elf \
--aligned_verification_data_path ~/.aligned/aligned_verification_data \
--conn wss://batcher.alignedlayer.com
```

5. You should get a response like this:

```bash
[2024-07-01T19:17:54Z WARN  aligned] Missing keystore used for payment. This proof will not be included if sent to Eth Mainnet
[2024-07-01T19:17:54Z INFO  aligned] Submitting proofs to the Aligned batcher...
[2024-07-01T19:19:18Z INFO  aligned] Batch inclusion data written into ./aligned_verification_data/e367d76e_0.json
[2024-07-01T19:19:18Z INFO  aligned] Proofs submitted to aligned. See the batch in the explorer:
[2024-07-01T19:19:18Z INFO  aligned] https://explorer.alignedlayer.com/batches/0xe367d76e832edec893d3a9027b3c231b2e3994c47acfac2e67197c13c9be0c4c
```

Use the link in the response to check the status of your transaction in the Aligned explorer.

6. After three Ethereum blocks, you should be able to check if it has been verified with the CLI using

```bash
aligned verify-proof-onchain \
--aligned-verification-data ~/.aligned/aligned_verification_data/*.json \
--rpc https://ethereum-holesky-rpc.publicnode.com \
--chain holesky
```

This is reading the result of the verification of the proof in Ethereum.

7. You should get this result:

```bash
[2024-06-17T21:58:43Z INFO  aligned] Your proof was verified in Aligned and included in the batch!
```

If the proof wasn't verified you should get this result:

```bash
[2024-06-17T21:59:09Z INFO  aligned] Your proof was not included in the batch.
```

Aligned works in:
- MacOS Arm64 (M1 or higher)
- Linux x86 with GLIBC_2.32 or superior (For example, Ubuntu 22.04 or higher)

If you don't meet these requirements, you can compile the binaries yourself following the [README](https://github.com/yetanotherco/aligned_layer)

To try Aligned with other proving systems, check [this](https://docs.alignedlayer.com/guides/0_submitting_proofs) guide
