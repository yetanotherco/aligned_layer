# Circom Example

This example demonstrates how to generate a Circom proof, submit it to **Aligned**, verify it on-chain, and read the public inputs from the proof.

The circuit is a simple Fibonacci example that outputs the result as a **public input**.

A Rust program is used to:

1. Run the Circom prover
2. Submit the proof to **Aligned** for verification
3. Once verified, send the verification data to the smart contract
4. Update the contract state using the proof’s public input

## What this example showcases

-   How to create a circuit in **Circom**
-   How to verify a Circom proof with **Aligned**
-   How to read and verify **public inputs** on-chain

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
