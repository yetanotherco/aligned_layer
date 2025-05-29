# Integrating your application with Aligned Aggregation Mode

This guide demonstrates how to build a toy L2 application that integrates with Aligned Aggregation Mode. The L2 does not post state diffs or any data to Ethereum, only commitments. The prover has to prove that:

1. The state database used in the proof must match the commitment stored in the on-chain contract. This is validated by computing the commitment of the received data in the zkvm and then exposing it as a public input.
2. The users performing the transfers have enough balance

After processing the transfers, the vm computes the commitment of the post state, which is exposed as a public input. The smart contract then updates the on-chain state root. If a user later wants to retrieve their state, the application must return it along with a Merkle proof, so they can verify it against the contract’s state root.

Notice a lot of checks that a real L2 should have are missing, since the focus are on the integration of Aligned.

## L2 workflow overview

This Layer 2 (L2) system operates in two main steps:

-   Off-chain execution and proof generation + verification with Aligned Verification Layer (a.k.a Fast Mode).
-   On-chain state update via proof verification with Aligned Aggregation Mode.

In Step 1, we execute user transfers and generate a zkVM-based proof of the state transition, which is submitted to Aligned’s verification layer.

In Step 2, once the proof is aggregated (every 24 hours), it is verified on-chain to update the global state.

### Step 1: Off-Chain Execution + Proof Generation

1. Initialize State: Load or initialize the current system state.
2. Load Transfers: Retrieve or receive the user transfer data for this batch.
3. Execute in zkVM: Run the zkVM with the loaded transfers to compute the new state.
4. Generate Proof: Produce a zk-proof for the executed state transition committing the commitment of the received + the commitment of the new state.
5. Submit Proof to Aligned: Send the proof to Aligned Verification Layer
6. Save the binary proof locally for later on-chain verification.

### Step 2: Proof Verification + On-Chain State Update

7. Load the proof binary: Retrieve the saved proof binary from disk.
8. Update On-Chain State: Call the smart contract method `updateStateTransition`, which:

    - Internally calls `verifyProofInclusion` on AlignedProofAggregationService which:
        1. Computes the proof commitment from the proof `public_inputs` and `program_id`.
        2. Uses the Merkle proof to reconstruct and validate the Merkle root.
        3. Confirms whether there exists and aggregated proof with that root.
    - Validates that the `initial_state_root` proof public input matches the on-chain state.
    - If valid, updates the on-chain state root to the `post_state_root`.

# Usage

### Requirements

1. [Rust](https://www.rust-lang.org/tools/install): we have tested in v1.85.1
2. [Foundry](https://book.getfoundry.sh/getting-started/installation)
3. [Docker](https://docs.docker.com/engine/): for SP1 prover

Submodules of the repo should be imported by running on the root folder:

```shell
make submodules
```

You can run the example on:
- [Holesky](#setup-holeksy)
- [Localnet](#setup-localnet)

## Setup Holeksy

### 1. Create keystore

You can use cast to create a local keystore. If you already have one you can skip this step.

```bash
cast wallet new-mnemonic
```

Then you can import your created keystore using:

```bash
cast wallet import --interactive <path_to_keystore.json>
```

Then you need to obtain some funds to pay for gas and proof verification.
You can do this by using this [faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)

*This same wallet is used to send the proof via aligned, so you'll also need to fund it on aligned. Follow this [guide](https://docs.alignedlayer.com/guides/0_submitting_proofs#id-2.-send-funds-to-aligned).*

### 2. Deploy the contract

-   Generate the base `.env`:

```shell
make gen_env_contract_holesky
```

-   Get the program ID of the l2 program you are proving:

```shell
make generate_program_id
```

-   Complete the following fields `contracts/.env` file:

    -   `PROGRAM_ID=` (use the previously generated ID, you can re check with a `cat ./crates/l2/programs_ids.json` )
    -   `PRIVATE_KEY`: the private key used for the deployment, it needs to have some funds to pay for the deployment.
    -   `OWNER_ADDRESS`: you have to provide the *address of the wallet created in step `1.`*.

-   Deploy the contracts with:

```shell
make deploy_contract
```

*Save the output contract address.*

### 3. Setup the L2

-   Generate the base `.env` run:

```shell
make gen_env_l2_holesky
```

-   Complete the missing fields on the `.env`:

    -   `PRIVATE_KEY_STORE_PATH`: The path to the keystore created in `1.`.
    -   `PRIVATE_KEY_STORE_PASSWORD`: The password of the keystore crated in step `1.`.
    -   `STATE_TRANSITION_CONTRACT_ADDRESS`: The address of the contract deployed in step `2.`

Finally [run the l2](#running-the-l2).

## Setup Localnet

You can also run this example on a local devnet. To get started, navigate to the root of the Aligned repository

- Start Ethereum package and the Batcher

```shell
# This will start the local net
make ethereum_package_start
# Start the batcher
make batcher_start_ethereum_package
```

- Navigate back to the example directory:

```shell
cd examples/l2
``` 

- Generate the `.env` files for the contracts and L2:

```shell
make gen_env_contract_devnet
make gen_env_l2_devnet
```

- Generate a pre funded wallet (or create one as specified [previously here](#1-create-keystore)):

```shell
# This will generate the keystore and fund it on aligned
make gen_devnet_owner_wallet
```

- Generate the program ID of the program that is going to be proven:

```shell
make generate_program_id
```

- Set the generated program ID on `contracts/.env`.

- Deploy the contract

```shell
make deploy_contract
```

- Set the output address of the contract in `.env`

- [run the l2](#running-the-l2)


## Running the L2

- Set up the initial State

```shell
make init_state
```

-   Perform the L2 account updates and prove them in the zkvm:

```shell
make prove_state_transition
```

- Wait 24 hs for the proof to be aggregated, or if running locally, run the aggregator with either:

    ```make start_proof_aggregator_ethereum_package AGGREGATOR=sp1``` 
or with cuda:
    ```make start_proof_aggregator_gpu_ethereum_package AGGREGATOR=sp1```

-   Update state transition on chain:

```shell
make update_state_on_chain
```

You should see a transaction receipt in the console and after the stateRoot updated on-chain.
