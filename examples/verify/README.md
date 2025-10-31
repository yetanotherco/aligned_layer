# Example Scripts for Batch Inclusion Verification

## Requirements

- [Python 3.9 or higher](https://www.python.org/downloads/) 
- For the deployment script you need to install [Foundry](https://book.getfoundry.sh/getting-started/installation)

Then, install the required dependencies by running the following command:

```bash
pip3 install -r requirements.txt
```

## Deploying Example Contract

Before you can interact with the `VerifyBatchInclusionCaller` contract, you need to deploy it to the blockchain. Here are the steps to do that:

First create a `.env` file in the root directory of the project with the following content:

| Variable                    | Value                                                                                                                                                                                                                                   |
|-----------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `PRIVATE_KEY`               | Your ethereum private key                                                                                                                                                                                                               |
| `RPC_URL`                   | Your ethereum RPC url. You can use public node: https://ethereum-hoodi-rpc.publicnode.com                                                                                                                                             |
| `ALIGNED_DEPLOYMENT_OUTPUT` | Path to aligned layer deployment output. This is needed to get service manager address. You can get it from https://github.com/yetanotherco/aligned_layer/blob/main/contracts/script/output/hoodi/alignedlayer_deployment_output.json |

Then, you can deploy the contract by running the following command:

```bash
./scripts/deploy_verify_batch_inclusion_caller.sh
```

This will output the address of the deployed contract. You will need this address to interact with the contract.

## Verifying Batch Inclusion

### Using curl

First encode the ethereum call to the contract using the following command:

```bash
python3 encode_verification_data.py --aligned-verification-data [PATH_TO_ALIGNED_VERIFICATION_DATA] --sender-address [SENDER_ADDRESS]
```

Replace `[PATH_TO_ALIGNED_VERIFICATION_DATA]` with the path to the json file containing the verification data. 
This is the output when submitting a proof from the aligned cli.

Replace `[SENDER_ADDRESS]` with the address of the `BatcherPaymentService` contract.

This will output the encoded call. You can then use this encoded call to check your submitted proof with the associated data is verified in Ethereum by running the following command:

```bash
curl -X POST <RPC_URL> \
-H "Content-Type: application/json" \
-d '{
    "jsonrpc": "2.0",
    "method": "eth_call",
    "params": [{
        "to": "<CONTRACT_ADDRESS>",
        "data": "<CALLDATA>"
    }],
    "id": 1
}'
```

Replace `<CONTRACT_ADDRESS>` with the address of the contract you deployed earlier (or `0x87CD431F160e88EC34fA48EC6F6cF7F2C0E8248c` for Aligned ServiceManager in Hoodi), `<CALLDATA>` with the encoded call, 
and `<RPC_URL>` with the RPC URL of the blockchain you are using (`https://ethereum-hoodi-rpc.publicnode.com` for Hoodi).

The output data should be something like this:

```json
{
  "jsonrpc":"2.0",
  "result":"0x0000000000000000000000000000000000000000000000000000000000000001",
  "id":1
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
python3 verify.py --contract-address [CONTRACT_ADDRESS] --aligned-verification-data [PATH_TO_ALIGNED_VERIFICATION_DATA] --sender-address [SENDER_ADDRESS]
```

Replace `[CONTRACT_ADDRESS]`, `[PATH_TO_ALIGNED_VERIFICATION_DATA]` and `[SENDER_ADDRESS]` with your actual values.

#### Example Command

```bash
python3 verify.py --contract-address 0x87CD431F160e88EC34fA48EC6F6cF7F2C0E8248c --aligned-verification-data ../../aligned_verification_data/b8c17406_4.json --sender-address 0x041af25Fce2413570aaa0029D36DeA1eFdeff083
```

In this case, `--contract-address` is the address of the `AlignedLayerServiceManager` and `--sender-address` is the address of the `BatcherPaymentService` in Hoodi Testnet.

You need to replace the `--aligned-verification-data` with the path to the JSON file containing the verification data. This is the output when submitting a proof.
