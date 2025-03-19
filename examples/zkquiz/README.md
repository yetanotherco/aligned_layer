# ZK Aligned Quiz

## Introduction

This is an example of a program running over Aligned, leveraging its ZK verification infrastructure. It consists of a simple quiz program, which if answered correctly, will grant the user an NFT acknowledging this.

The User first runs the ZKQuiz program, answers the questions on the quiz, and ZKQuiz will generate a ZK Proof stating the user has answered the quiz correctly. After this, ZKQuiz will post the ZK Proof on Aligned. Once this proof is verified by Aligned, ZKQuiz mints an NFT through a Smart Contract that checks if the user has indeed verified a correct ZKQuiz proof.

This way, the User can only obtain this NFT if he knows the answers to the ZKQuiz;
- If the User answers incorrectly, the proof generation will fail.
- If the User tries to tamper ZKQuiz code, the ZK Proof will correspond to another Rust code, with another checksum. Therefore, the Smart Contract will not mint an NFT for the User

Next, we will see how to execute ZKQuiz, so you can get your own ZKQuiz NFT!

## Requirements

1. [Rust v1.80.1](https://www.rust-lang.org/tools/install)
2. [Foundry](https://getfoundry.sh)

> [!INFO]
> ELF commitment was generated with Rust v1.80.1 and SP1 v3.0.0.

## Usage

### 1 - Create Keystore

You can use cast to create a local keystore.
If you already have one you can skip this step.

```bash
cast wallet new-mnemonic
```

Then you can import your created keystore using:

```bash
cast wallet import --interactive <path_to_keystore.json>
```

Then you need to obtain some funds to pay for gas and proof verification.
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

## Testing locally

If you want to test the zk quiz on a local network follow these steps:

1. Setup Aligned locally following [this guide](../../docs/3_guides/6_setup_aligned.md)

3. Move into the zkquiz example:
    ```
    cd examples/zkquiz
    ```

4. Deploy the ZKQuiz verifier contract, and locate the `CONTRACT_ADDRESS` from its output:
    ```
    make deploy_verifier_devnet
    ```


5. Run the quiz:
    ```
    CONTRACT_ADDRESS=<VERIFIER_CONTRACT_ADDRESS> make answer_quiz_local
    ```
