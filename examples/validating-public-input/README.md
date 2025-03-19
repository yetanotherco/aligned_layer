# Validating public input

## Testing locally

Set up all the components of aligned locally following the [aligned setup guide](../../docs/3_guides/6_setup_aligned.md).

This example is designed to do either with SP1 or risc0 proofs these are the commands to use depending on which verifier want to be used.

### Risc0

> [NOTE]  
> In running on MacOS you may need to install xcode or update `metal` dependencies.

1. `make generate_risc0_fibonacci_proof`

2. `make submit_fibonacci_risc0_proof_devnet`

> The batch needs at least two proofs to be sealed, in another terminal run `make batcher_send_risc0_task` to actually submit the batch to aligned.

The command will log the file where all the aligned verification data was saved like so:

```
[2024-10-09T15:54:42Z INFO  aligned_integration] Saved batch inclusion data to ".../aligned_test/examples/validating-public-input/aligned-integration/batch_inclusion_data/<DATA_FILE_NAME>"
```

Save the name since it will be necessary, you can found it in the path logged in the previous command otherwise.

3. `make deploy_fibonacci_validator_devnet`

The command will log the address where the validator was deployed:

```
##### anvil-hardhat
✅  [Success]Hash: 0xbdb6bbe9403b59de6d960dd231dae658b401735506feb0745eebe76d5f95e8f4
Contract Address: 0x90d6A3E189C70E1Cce16c29151077cB9Badb2448
Block: 2963688
Paid: 0.000727925005095475 ETH (727925 gas * 1.000000007 gwei)
```

> [NOTE]  
> The `Contract Address` listed above corresponds to a current valid deployment of the examples contract on the `holesky` testnet.

save the contract address for the next command.

4. `make verify_risc0_batch_inclusion_devnet FIBONACCI_VALIDATOR_ADDRESS=<FIBONACCI_VALIDATOR_ADDRESS> DATA_FILE_NAME=<DATA_FILE_NAME>`

Where `FIBONACCI_VALIDATOR_ADDRESS` is the address of the deployed validator contract and `DATA_FILE_NAME` the name of the file where the aligned verification data was saved (including the extension `.json`).

If everything goes well you should see a transaction receipt with a `success` label in the status:

```
...
root                    <ROOT_HASH>
status                  1 (success)
transactionHash         <TX_HASH>
...
```

### SP1

> [NOTE]  
> In running on MacOS you may need to install xcode or update `metal` dependencies.

1. `make generate_sp1_fibonacci_proof`

2. `make submit_fibonacci_sp1_proof_devnet`

> The batch needs at least two proofs to be selaed, in another terminal run `make batcher_send_sp1_task` to actually submit the batch to aligned.

The command will log the file where all the aligned verification data was saved like so:

```
[2024-10-09T15:54:42Z INFO  aligned_integration] Saved batch inclusion data to "aligned-layer/examples/validating-public-input/aligned-integration/batch_inclusion_data/<DATA_FILE_NAME>"
```

Save the name since it will be necessary, you can found it in the path logged in the previous command otherwise.

3. `make deploy_fibonacci_validator_devnet`

The command will log the address where the validator was deployed:

```
##### anvil-hardhat
✅  [Success]Hash: 0xbdb6bbe9403b59de6d960dd231dae658b401735506feb0745eebe76d5f95e8f4
Contract Address: 0x90d6A3E189C70E1Cce16c29151077cB9Badb2448
Block: 2963688
Paid: 0.000727925005095475 ETH (727925 gas * 1.000000007 gwei)
```
> [NOTE]  
> The `Contract Address` listed above corresponds to a current valid deployment of the examples contract on the `holesky` testnet.

save the contract address for the next command.

4. `make verify_sp1_batch_inclusion_devnet FIBONACCI_VALIDATOR_ADDRESS=<FIBONACCI_VALIDATOR_ADDRESS> DATA_FILE_NAME=<DATA_FILE_NAME>`

Where `FIBONACCI_VALIDATOR_ADDRESS` is the address of the deployed validator contract and `DATA_FILE_NAME` the name of the file where the aligned verification data was saved (including the extension `.json`).

If everything goes well you should see a transaction receipt with a `success` label in the status:

```
...
root                    <ROOT_HASH>
status                  1 (success)
transactionHash         <TX_HASH>
...
```
