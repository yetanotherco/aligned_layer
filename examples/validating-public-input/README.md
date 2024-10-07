# Validating public input

## Testing locally

Set up all the components of aligned locally following the [aligned setup guide](../../docs/3_guides/6_setup_aligned.md).

This example is designed to do either with SP1 or risc0 proofs these are the commands to use depending on which verifier want to be used.

### Risc0

1. `make generate_risc0_fibonacci_proof`

2. `make submit_fibonacci_risc0_proof_devnet`

> The command will log the file where all the aligned verification data was saved, save the name since it will be necessary.
3. `deploy_local_fibonacci_validator`

> The command will log the address where the validator was deployed, save it for the next command.
4. `make verify_risc0_local_batch_inclusion <FIBONACCI_VALIDATOR_ADDRESS> <DATA_FILE_NAME>`

Where `FIBONACCI_VALIDATOR_ADDRESS` is the address of the deployed validator contract and `DATA_FILE_NAME` the name of the file where the aligned verification data was saved (without the extension `.json`).

### SP1

1. `make generate_sp1_fibonacci_proof`

2. `make submit_fibonacci_sp1_proof_devnet`

> The command will log the file where all the aligned verification data was saved, save the name since it will be necessary.
3. `deploy_local_fibonacci_validator`

> The command will log the address where the validator was deployed, save it for the next command.
4. `make verify_sp1_local_batch_inclusion <FIBONACCI_VALIDATOR_ADDRESS> <DATA_FILE_NAME>`

Where `FIBONACCI_VALIDATOR_ADDRESS` is the address of the deployed validator contract and `DATA_FILE_NAME` the name of the file where the aligned verification data was saved (without the extension `.json`).
