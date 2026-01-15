# Aligned Infrastructure Deployment Guide

## Dependencies

Ensure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install)
-   [Docker](https://docs.docker.com/get-docker/)
-   [Kurtosis](https://docs.kurtosis.com/install/)

## Supported Verifiers

The aggregation mode currently supports the following proving systems:

-   **SP1** - Succinct's zkVM (compressed proofs)

## Step-by-Step Setup

Follow these steps to start the aggregation mode locally using the Ethereum package environment.

### 1. Start the Ethereum Package

Start the local Ethereum network using Kurtosis:

```bash
make ethereum_package_start
```

This command spins up a local Ethereum network with all necessary components. To stop it run:

```bash
make ethereum_package_rm
```

### 2. Start the Gateway

Start the aggregation mode gateway service:

```bash
make agg_mode_gateway_start_ethereum_package
```

The gateway handles proof submissions and manages the proof queue. This command also starts the required Docker containers (PostgreSQL) and runs database migrations automatically.

### 3. Start the Payments Poller

In a separate terminal, start the payments poller:

```bash
make agg_mode_payments_poller_start_ethereum_package
```

The payments poller monitors the blockchain for payment events and updates user quotas accordingly.

### 4. Send a Payment (Deposit)

Deposit funds to get quota for submitting proofs:

```bash
make agg_mode_gateway_send_payment
```

This deposits funds using a default test account. For custom deposits, you can use the CLI directly.

### 5. Submit a Proof

Submit an SP1 proof to the gateway:

```bash
make agg_mode_gateway_send_sp1_proof
```

This sends a test SP1 Fibonacci proof to the gateway.

### 6. Start the Proof Aggregator

In a separate terminal, start the proof aggregator:

```bash
AGGREGATOR=sp1 make proof_aggregator_start_ethereum_package
```

The proof aggregator fetches pending proofs from the database, aggregates them, and submits the aggregated proof on-chain.
