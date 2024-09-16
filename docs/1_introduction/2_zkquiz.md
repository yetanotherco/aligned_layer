# ZK Quiz

## Introduction

This program leverages Aligned's ZK verification infrastructure to run a small quiz. The proof allows any party to check that the quiz was answered right or wrong. If answered correctly, the user receives an NFT.

The process is as follows:

1. The user runs ZKQuiz and answers the questions.
2. ZKQuiz generates a ZK Proof of correct answers.
3. The proof is posted on Aligned.
4. Upon verification, ZKQuiz mints an NFT via a Smart Contract.

The NFT is only granted if the user's answers correctly. 
Incorrect answers or tampering with the ZKQuiz code will result in proof generation failure or mismatched checksums, 
preventing NFT minting.

Next, we will see how to execute ZKQuiz to get your own ZKQuiz NFT!

## Requirements

1. [Rust](https://www.rust-lang.org/tools/install)
2. [Foundry](https://getfoundry.sh)

## Usage

### 1 - Create Keystore

You can use cast to create a local keystore.
If you already have one, you can skip this step.

```bash
cast wallet new-mnemonic
```

Then you can import your created keystore using:

```bash
cast wallet import --interactive <path_to_keystore.json>
```

Then you need to get some funds to pay for gas and proof verification.
You can do this by using this [faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)

### 2 - Answer Quiz

To answer quiz questions run:

```bash
make answer_quiz KEYSTORE_PATH=<path_to_keystore.json>
```

This will:

1. Ask quiz questions
2. Generate ZK proof
3. Pay & submit proof to aligned for verification
4. Wait for proof to be verified in aligned
5. Claim NFT if proof is verified