# Example Scripts for Batch Inclusion Verification

## Requirements

- [Python 3.9](https://www.python.org/downloads/) or higher
- For the deployment script you need to install [Foundry](https://book.getfoundry.sh/getting-started/installation)

## Deploying Example Contract

Before you can interact with the `VerifyBatchInclusionCaller` contract, you need to deploy it to the blockchain. Here are the steps to do that:

First create a `.env` file in the root directory of the project with the following content:

| Variable                    | Value                                                                                                                                                                                                                                   |
|-----------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `PRIVATE_KEY`               | Your ethereum private key                                                                                                                                                                                                               |
| `RPC_URL`                   | Your ethereum RPC url. You can use public node: https://ethereum-holesky-rpc.publicnode.com                                                                                                                                             |
| `ALIGNED_DEPLOYMENT_OUTPUT` | Path to aligned layer deployment output. This is needed to get service manager address. You can get it from https://github.com/yetanotherco/aligned_layer/blob/main/contracts/script/output/holesky/alignedlayer_deployment_output.json |

Then, you can deploy the contract by running the following command:

```bash
./scripts/deploy_verify_batch_inclusion_caller.sh
```

This will output the address of the deployed contract. You will need this address to interact with the contract.

## Verifying Batch Inclusion

### Parameters

1. `--contract-address`: The address of the contract you want to interact with.
2. `--aligned-verification-data`: The path to the JSON file containing the verification data. This is the output when submitting a proof from the aligned cli.

### Running the Script

Install the required dependencies by running the following command:
```bash
pip3 install -r scripts/requirements.txt
```

Then, you can run the script by running the following command:
```bash
python3 scripts/main.py --contract-address [CONTRACT_ADDRESS] --aligned-verification-data [PATH_TO_ALIGNED_VERIFICATION_DATA]
```

Replace `[CONTRACT_ADDRESS]`, and `[PATH_TO_ALIGNED_VERIFICATION_DATA]` with your actual values.

## Example Command

```bash
python3 scripts/main.py --contract-address 0x623926229DD27c45AE40B4e16ba4CD6522fC4d22 --aligned-verification-data ../../aligned_verification_data/7553cb14bff387c06e016cb3e7946e91d9fe44a54ad5d888ce8343ddb16116a7_118.json
```
