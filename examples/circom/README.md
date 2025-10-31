# Circom Example

This example demonstrates how to generate a Circom proof, submit it to **Aligned**, verify it on-chain, and read the public inputs from the proof.

The circuit is a simple Fibonacci example that outputs the result as a **public input**.

A Rust program is used to:

1. Run the Circom prover
2. Submit the proof to **Aligned** for verification
3. Once verified, send the verification data to the smart contract
4. Update the contract state using the proof’s public input

## Project structure

-   **`circuits/`**: the Circom circuit (Fibonacci example)
-   **`contracts/`**: the smart contract that update the fibonacci value with the Aligned verification
-   **`src/`** – Rust code to:
    -   Run the prover
    -   Submit the proof to Aligned
    -   Send the resulting verification data to the contract
    -   Update the contract state with the proof’s public output

## Requirements

-   [snarkjs](https://github.com/iden3/snarkjs)
-   [circom compiler](https://docs.circom.io/)

## How to run it

1.  Generate the trusted setup:

```shell
make generate_circom_groth16_bn256_setup
```

This command will output the vk commitment like this:

```shell
VK COMMITMENT IS: `0xd8aeedbdfc90bfc0f61d81efdd23bcf119d7825f74d2af7071fee8fa144a3cb1`
```

2. Fill the `.env` variables in `contracts/.env`

Follow the `contracts/.env.example`.

3. Deploy the contract

```shell
make deploy_contract
```

This command will output the contract address like this:

```shell
##### anvil-hardhat
✅  [Success] Hash: 0x37679e098b58e952f3e4ec7f228f6a2b95d62608bcaa54585d1be2ef71ddc9ff
Contract Address: 0xDC11f7E700A4c898AE5CAddB1082cFfa76512aDD
Block: 2001
Paid: 0.000000000005023288 ETH (627911 gas * 0.000000008 gwei)
```

Save it for the next step.

4. Fill the program `.env` variable:

Follow the `.env.example`.

5. Run the program:

```shell
make run
```

## How to run it locally

Set up all the components of aligned locally following the [aligned setup guide](../../docs/3_guides/6_setup_aligned.md).

1.  Generate the trusted setup:

```shell
make generate_circom_groth16_bn256_setup
```

This command will output the vk commitment like this:

```shell
VK COMMITMENT IS: `0xd8aeedbdfc90bfc0f61d81efdd23bcf119d7825f74d2af7071fee8fa144a3cb1`
```

2. Complete the `VK_COMMITMENT` variable in `contracts/.env.devnet`

3. Deploy the contract

```shell
make deploy_contract_devnet
```

This command will output the contract address like this:

```shell
##### anvil-hardhat
✅  [Success] Hash: 0x37679e098b58e952f3e4ec7f228f6a2b95d62608bcaa54585d1be2ef71ddc9ff
Contract Address: 0xDC11f7E700A4c898AE5CAddB1082cFfa76512aDD
Block: 2001
Paid: 0.000000000005023288 ETH (627911 gas * 0.000000008 gwei)
```

4. Complete the `FIBONACCI_CONTRACT_ADDRESS` variable in the program env at `.env.devnet`:

5. Run the program:

```shell
make run_devnet
```
