# ZK Aligned Quiz

## Requirements

1. [Rust](https://www.rust-lang.org/tools/install)
2. [Python](https://www.python.org/downloads/)
3. [Aligned](https://github.com/yetanotherco/aligned_layer)

## Usage

First, install dependencies by running:
```bash
make deps
```
This will create a virtual environment and install python dependencies.

To answer quiz and generate proof run:
```bash
make answer_quiz
```

This will ask questions and generate a proof if you answer correctly.

To submit the proof to aligned for verification run:
```bash
make submit_proof ADDRESS=<your_address>
```

Make sure to use your own address as this is the address that will receive the reward.

Head to [Aligned Explorer](https://explorer.alignedlayer.com/batches) and wait for the batch to be verified.

Then to verify the proof was verified on aligned, and mint your nft run:
```bash
make verify_and_get_reward VERIFICATION_DATA=<path_to_aligned_verification_data> PRIVATE_KEY=<your_private_key>
```
Note that the path to your proof verification data will be printed out when you submit the proof.

This will verify the proof and mint your nft. 
You can check your nft on the [Chainlens Explorer](https://holesky.chainlens.com/nfts/0x01f7556ea65a5b4142c0a0e36a45c65ca9f3a4ff).
