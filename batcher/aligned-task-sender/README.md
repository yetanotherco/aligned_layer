# Task Sender
This CLI is made to stress-test the network.

It has the following commands:

## GenerateProofs

This command is to generate N Groth16 proofs.

To run it, you can:
```
	cargo run --release -- generate-proofs \
        --number-of-proofs <NUMBER_OF_PROOFS> --proof-type groth16 \
        --dir-to-save-proofs $(PWD)/scripts/test_files/task_sender/proofs
```

We also have a make target:
```
NUMBER_OF_PROOFS=15 make task_sender_generate_groth16_proofs
```
## GenerateAndFundWallets

This command is to generate N wallets, and fund them in the BatcherPaymentService.

To run it, you can:
```
	cargo run --release -- generate-and-fund-wallets \
        --eth-rpc-url <RPC_URL> \
        --network <NETWORK> \
        --funding-wallet-private-key <FUNDING_WALLET_PRIVATE_KEY> \
        --number-wallets <NUM_WALLETS> \
        --amount-to-deposit <AMOUNT_TO_DEPOSIT> \
        --amount-to-deposit-to-aligned <AMOUNT_TO_DEPOSIT_TO_ALIGNED> \
        --private-keys-filepath <PATH_TO_PRIVATE_KEYS_FILE>
```

In Devnet:
Run anvil with more prefunded accounts, using the `-a 1000` flag.
Then run the following make target, `NUM_WALLETS` being the amount of wallets you want to deposit funds to aligned payment service, up to 1000.
```bash
NUM_WALLETS=123 make task_sender_generate_and_fund_wallets_devnet
```

## SendInfiniteProofs

This command sends `BURST_SIZE` proofs from each private key in `PATH_TO_PRIVATE_KEYS_FILE`.

To vary the amount of senders, it is recommended to have a backup with all private keys, and add/remove keys from the file being used.

To run it, you can:
```
	cargo run --release -- send-infinite-proofs \
        --burst-size <BURST_SIZE> --burst-time-secs <BURST_TIME_SECS> \
        --eth-rpc-url <RPC_URL> \
        --batcher-url <BATCHER_URL> \
        --network holesky-stage \
        --proofs-dirpath $(PWD)/scripts/test_files/task_sender/proofs \
        --private-keys-filepath <PATH_TO_PRIVATE_KEYS_FILE>
```


## TestConnections

This command enables and hangs N connections with the Batcher.

To run it, you can:
```
	cargo run --release -- test-connections \
        --batcher-url <BATCHER_URL> \
        --num-senders <NUM_SENDERS>
```

