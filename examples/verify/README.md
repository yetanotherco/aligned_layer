# Example Scripts for Batch Inclusion Verification

## Requirements

- [Python 3.9 or higher](https://www.python.org/downloads/) 
- For the deployment script you need to install [Foundry](https://book.getfoundry.sh/getting-started/installation)

Then, install the required dependencies by running the following command:

```bash
pip3 install -r scripts/requirements.txt
```

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

### Using curl

First encode the ethereum call to the contract using the following command:

```bash
python3 scripts/encode_call.py --aligned-verification-data [PATH_TO_ALIGNED_VERIFICATION_DATA]
```

Replace `[PATH_TO_ALIGNED_VERIFICATION_DATA]` with the path to the json file containing the verification data. 
This is the output when submitting a proof from the aligned cli.

This will output the encoded call. You can then use this encoded call to check your submitted proof with the associated data is verified in Ethereum by running the following command:

```bash
curl -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"eth_call","params":[{"to": "<CONTRACT_ADDRESS>", "data": "<CALL_DATA>"}]}]' \
    -X POST <RPC_URL>
```

Replace `<CONTRACT_ADDRESS>` with the address of the contract you deployed earlier, `<CALL_DATA>` with the encoded call, 
and `<RPC_URL>` with the RPC URL of the blockchain you are using.

The output data should be something like this:

```json
{
  "jsonrpc":"2.0",
  "result":"0x0000000000000000000000000000000000000000000000000000000000000001",
  "id":null
}
```

Note that if result ends in 1 it means that your submitted proof with the associated data is verified in Ethereum, otherwise it is not.

### Using Python Script

#### Parameters

1. `--contract-address`: The address of the contract you want to interact with.
2. `--aligned-verification-data`: The path to the JSON file containing the verification data. This is the output when submitting a proof from the aligned cli.

#### Running the Script

Then, you can run the script by running the following command:
```bash
python3 scripts/main.py --contract-address [CONTRACT_ADDRESS] --aligned-verification-data [PATH_TO_ALIGNED_VERIFICATION_DATA]
```

Replace `[CONTRACT_ADDRESS]`, and `[PATH_TO_ALIGNED_VERIFICATION_DATA]` with your actual values.

#### Example Command

```bash
python3 scripts/main.py --contract-address 0x623926229DD27c45AE40B4e16ba4CD6522fC4d22 --aligned-verification-data ../../aligned_verification_data/7553cb14bff387c06e016cb3e7946e91d9fe44a54ad5d888ce8343ddb16116a7_118.json
```
