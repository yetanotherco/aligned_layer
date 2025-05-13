# Integrating your application with Aligned Aggregation Mode

This guide demonstrates how to build a minimal L2 application that integrates with Aligned Aggregation Mode. The L2 is private, that is: it does not post state diffs or any data to Ethereum, only commitments. This way, the prover has to prove that:

1. The state database used in the proof must match the commitment stored in the on-chain contract. This is validated by computing the commitment of the received data in the zkvm and then exposing it as a public input.
2. The users performing the transfers have enough balance

After processing the transfers, the vm computes the commitment of the post state, which is exposed as a public input. The smart contract then updates the on-chain state root. If a user later wants to retrieve their state, the application must return it along with a Merkle proof, so they can verify it against the contract’s state root.

## Further improvements

This is a very basic and minimal L2 design and can be extended. For example:

-   Block Support: Add support for batching transactions into blocks and publishing those blocks.
-   Soft Finality: Use the Verification Layer or Fast Mode for faster confirmation of L2 blocks.
-   Hard Finality: Use Aggregation Mode to finalize state transitions with high security.

### How it works: Step by Step

1. Load or initialize the database state.
2. Load user transfers.
3. Run the zkvm with db + transfers to perform.
4. Generate and submit the proof to Aligned.
5. Wait for the proof to be aggregated.
6. Call the smart contract function `updateStateTransition`, which:
    1. Calls `verifyProofInclusion` in the `AlignedProofAggregationService`, which:
        - Computes the proof commitment with the provided `public_inputs` and `program_id`
        - Computes the Merkle Root, using the provided Merkle Proof.
        - Checks the root exists in the aggregation root.
    2. Verifies that the initial_state_root public input matches the on-chain state.
    3. If successful, updates the state root with the post_state_root public input.
7. If the contract call succeeds, updates the local database.

### Usage

#### Requirements

1. [Rust](https://www.rust-lang.org/tools/install): we have tested in v1.85.1
2. [Foundry](https://book.getfoundry.sh/getting-started/installation)
3. [Docker](https://docs.docker.com/engine/): for SP1 prover

#### 1. Create keystore

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

This same wallet is used to send the proof via aligned, so you'll also need to fund it on aligned. Follow this [guide](https://docs.alignedlayer.com/guides/0_submitting_proofs#id-2.-send-funds-to-aligned).

#### 2. Deploy the contract

Generate the base `.env`. For `Holesky` you can run:

```shell
make gen_env_contract_holesky
```

And then in `contracts/.env` you have to complete the missing variables:

-   `INITIAL_STATE_ROOT`: you can leave it as it is unless you change the initial state in `crates/l2/db.rs`
-   `PROGRAM_ID`: you need to ensure it matches the one on your machine:
    1. Run `make generate_program_id` to generate it.
    2. Check `crates/l2/programs_ids.json` for the ID.
-   `OWNER_ADDRESS`: you have to provide the address of the wallet created in step `1.`.
-   `PRIVATE_KEY`: the private key used for the deployment, it needs to have some funds to pay for the deployment.

Once you have completed the `.env`, you can deploy the contract:

```shell
make deploy_contract
```

Save the output contract address.

#### 3. Run L2 program

Generate the base `.env`. For `Holesky` you can run:

```shell
make gen_env_l2_holesky
```

And complete the variables:

-   `BEACON_CLIENT_URL`: A beacon client url, public node usually don't work as they don't support the endpoints to retrieve blob data
-   `PRIVATE_KEY_STORE_PATH`: The path to the keystore created in `1.`.
-   `PRIVATE_KEY_STORE_PASSWORD`: The password of the keystore crated in step `1.`.
-   `STATE_TRANSITION_CONTRACT_ADDRESS`: The address of the contract deployed in step `2.`

Finally, run the L2:

```shell
make run_l2
```

You should see a transaction receipt in the console and the stateRoot updated on-chain. You can run this process repeatedly, but make sure to not delete the db file, or the application will not be able to prove valid state transitions.
