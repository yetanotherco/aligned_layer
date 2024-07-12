# Aligned

> [!CAUTION]
> To be used in testnet only.

## Table of Contents

- [Aligned](#aligned)
  - [Table of Contents](#table-of-contents)
  - [The Project](#the-project)
  - [How to use the testnet](#how-to-use-the-testnet)
  - [Operator Guide](#operator-guide)
  - [Aligned Infrastructure Guide](#aligned-infrastructure-guide)
  - [FAQ](https://docs.alignedlayer.com/guides/4_faq)

## The Project

Aligned is a decentralized network of nodes that verifies Zero-Knowledge and Validity proofs, and post the results in Ethereum. 

These proofs can be generated and used for a tenth of the price, and with extremely low latency, allowing novel types of applications that weren't possible before in Ethereum.

## How to use the testnet

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
If you don't meet these requirements, clone the repository, install rust, and then run:

```bash
make uninstall_aligned
make install_aligned_compiling
```

### Reading the results of proof verification in Ethereum


#### Using CURL and an Ethereum RPC
In step 6 of the previous section, we used the `aligned verify-proof-onchain` to check that our proof was verified in Aligned.

Internally, this is making a call to our Aligned contract, verifying commitments are right, and that the proof is included in the batch.

That command is doing the same as the following `curl` to an Ethereum node.

```bash
curl -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"eth_call","id":1, "params":[{"to": "0x58F280BeBE9B34c9939C3C39e0890C81f163B623", "data": "<CALL_DATA>"}]}' \
    -X POST https://ethereum-holesky-rpc.publicnode.com
```

This will return 0x1 if the proof and it's associated data is correct and verified in Aligned, and 0x0 if not.

For example, this a correct calldata for a verified proof:

```bash
curl -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_call","id":1,"params":[{"to": "0x58F280BeBE9B34c9939C3C39e0890C81f163B623", "data": "0xfa534dc0c181e470901eecf693bfa6f0e89e837dcf35700cdd91c210a0ce0660e86742080000000000000000000000000000000000000000000000000000000000000000836371a502bf5ad67be837b21fa99bc381f7e8124f02042ffb80fa7ce27bc8f6f39fd6e51aad88f6f4ce6ab8827279cfffb922660000000000000000000000007553cb14bff387c06e016cb3e7946e91d9fe44a54ad5d888ce8343ddb16116a700000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000007600000000000000000000000000000000000000000000000000000000000001007b2f4966c3ab3e59d213eda057734df28c323055a2a02f50bd286585cc80128c967250f2b9ad990485338fd2d49e83f47917983f5566da551d4c32e9063ea5641d94b04bac222e06ea18cbb617d0d52c7007cc8f8b30c435b8b8101bdff0ea8482436acf251652f00397f4cefa0bb8eea1c8addb6cf2ca843004b89d80c7e1e41344fd2387535fe4afcaafde27b04543d993bbbc7286154044913e5bd65b86d7cc4d47a90132a95d9ffecb913b414ba2d2f0b1d7b826eb5025a27bcadcc0d94cb125c9c9d556eac08dd6b0f5f55f68afe699f3c529442dbf1b47e968b3705ee2e1be4acb884d184a139a390cb94e9e5806686605dc0a025269bc3afd990c8302"}]}' \
  -X POST https://ethereum-holesky-rpc.publicnode.com
```

To generate the calldata yourself, follow these steps:

1. Clone the repository and move into it
2. Create a Python virtual environment and install the dependencies with

```bash
python3 -m venv .aligned_venv
source .aligned_venv/bin/activate
python3 -m pip install -r examples/verify/requirements.txt
```

3. Encode your proof verification data with

```bash
python3 examples/verify/encode_verification_data.py --aligned-verification-data ~/.aligned/aligned_verification_data/*.json
```
 
If your verification data is in another path, just change the `--aligned-verification-data` parameter.

#### Using a caller contract 

To verify a proof in your own contract, use a static call to the Aligned contract. You can use the following [Caller Contract](examples/verify/src/VerifyBatchInclusionCaller.sol) as an example. The code will look like this:

```solidity
(bool callWasSuccessfull, bytes memory proofIsIncluded) = targetContract.staticcall(
    abi.encodeWithSignature(
        "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)",
        proofCommitment,
        pubInputCommitment,
        provingSystemAuxDataCommitment,
        proofGeneratorAddr,
        batchMerkleRoot,
        merkleProof,
        verificationDataBatchIndex
    )
);
require(callWasSuccessfull, "static_call failed");
```

## Operator Guide

If you want to run an operator, check our [Operator Guide](./docs/operator_guides/0_running_an_operator.md)

## Aligned Infrastructure Guide

If you are developing in Aligned, or want to run your own devnet, check our [setup Aligned guide](docs/guides/3_setup_aligned.md)
