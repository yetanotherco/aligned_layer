# Task Sender
This CLI is made to stress-test the network.

It has the following commands:

## GenerateProofs

This command is to generate N Groth16 proofs.

To run it, you can:
```bash
cargo run --release -- generate-proofs \
        --number-of-proofs <NUMBER_OF_PROOFS> --proof-type groth16 \
        --dir-to-save-proofs $(PWD)/scripts/test_files/task_sender/proofs
```

We also have a make target:
```bash
NUMBER_OF_PROOFS=15 make task_sender_generate_groth16_proofs
```
## GenerateAndFundWallets

This command is to generate N wallets, and fund them in the BatcherPaymentService.

To run it, you can:
```bash
cargo run --release -- generate-and-fund-wallets \
        --eth-rpc-url <RPC_URL> \
        --network <NETWORK> \
        --funding-wallet-private-key <FUNDING_WALLET_PRIVATE_KEY> \
        --number-wallets <NUM_WALLETS> \
        --amount-to-deposit <AMOUNT_TO_DEPOSIT> \
        --amount-to-deposit-to-aligned <AMOUNT_TO_DEPOSIT_TO_ALIGNED> \
        --private-keys-filepath <PATH_TO_PRIVATE_KEYS_FILE>
```

### In Testnet
```bash
NUM_WALLETS=<N> make task_sender_generate_and_fund_wallets_holesky_stage
```

### In Devnet:
Run anvil with more prefunded accounts, using the following make target:
```bash
make anvil_start_with_block_time_with_more_prefunded
```

Then run the following make target, with `NUM_WALLETS` being the amount of wallets you want to deposit funds to aligned payment service, up to 1000.
```bash
NUM_WALLETS=<N> make task_sender_generate_and_fund_wallets_devnet
```

## SendInfiniteProofs

This command sends `BURST_SIZE` proofs from each private key in `PATH_TO_PRIVATE_KEYS_FILE` every `BURST_TIME_SECS` seconds.

To vary the amount of senders, it is recommended to have a backup with all private keys, and add/remove keys from the file being used.

To run it, you can:
```bash
cargo run --release -- send-infinite-proofs \
        --burst-size <BURST_SIZE> --burst-time-secs <BURST_TIME_SECS> \
        --eth-rpc-url <RPC_URL> \
        --network <network> \
        --proofs-dirpath $(PWD)/scripts/test_files/task_sender/proofs \
        --private-keys-filepath <PATH_TO_PRIVATE_KEYS_FILE>
```

We also have the following related make targets
```bash
BURST_SIZE=<N> BURST_TIME_SECS=<N> make task_sender_send_infinite_proofs_devnet
```
```bash
BURST_SIZE=<N> BURST_TIME_SECS=<N> make task_sender_send_infinite_proofs_holesky_stage
```

## TestConnections

This command enables and hangs N connections with the Batcher.

To run it, you can:
```
cargo run --release -- test-connections \
        --num-senders <NUM_SENDERS>
```

We also have the following related make targets:
```bash
NUM_SENDERS=<N> make task_sender_test_connections_devnet
```
```bash
NUM_SENDERS=<N> make task_sender_test_connections_holesky_stage
```
