
# Getting started!

1. Download and install Aligned to send proofs in the testnet:

```bash
curl -L https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/install_aligned.sh | bash
```

2. Then run the ```source``` command that should appear in the shell


3. Download an example SP1 proof file with it's ELF file using:

```bash
curl -L https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/get_proof_test_files.sh | bash
```

We are downloading a proof previously generated, sending it to Aligned, and retrieving the results from Ethereum Holesky testnet. Aligned is using EigenLayer to do a fast and cheap verification of more than one thousand proofs per second.

4. Let's send the proof to be verified in Aligned:

```bash
rm -rf ~/.aligned/aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof ~/.aligned/test_files/sp1_fibonacci.proof \
--vm_program ~/.aligned/test_files/sp1_fibonacci-elf \
--aligned_verification_data_path ~/.aligned/aligned_verification_data \
--conn wss://batcher.alignedlayer.com
```

5. You should get a response like this:

```bash
[2024-06-17T22:06:03Z INFO  aligned] Proof submitted to aligned. See the batch in the explorer:
    https://explorer.alignedlayer.com/batches/0x8ea98526e48f72d4b49ad39902fb320020d3cf02e6506c444300eb3619db4c13
[2024-06-17T22:06:03Z INFO  aligned] Batch inclusion data written into /Users/maurofab/aligned_verification_data/8ea98526e48f72d4b49ad39902fb320020d3cf02e6506c444300eb3619db4c13_225.json
[2024-06-17T22:06:03Z INFO  aligned] All messages responded. Closing connection...
https://explorer.alignedlayer.com/batches/0x8ea98526e48f72d4b49ad39902fb320020d3cf02e6506c444300eb3619db4c13```
```

You can use the link to the explorer to check the status of your transaction. 

6. After three Ethereum blocks, you can check if it has been verified with:

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
