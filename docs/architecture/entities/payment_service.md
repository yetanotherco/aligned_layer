# Payment Service

## Introduction

To be able to use the batcher, a user must fund its transactions. For this, there is a simple Batcher Payment System.

The Batcher has a Batcher Payments smart contract, which is in charge of recieving user's payments, and which guarantees the batcher can only spend this money to send users' proofs to Aligned.

Users must first deposit into this contract, via a normal transfer to its address.

Then, users can send proofs to the Batcher, the Batcher will preemptively check if the user has funds for this, and once accumulating the whole batch, the Batcher will call its smart contract with the data it has recieved from the users.

The smart contract will then discount the corresponding amount of funds from each of the senders' balances, and create a new Task in Aligned, sending also the corresponding amount of tokens for the batch verification.

Users are also allowed to withdraw extra funds deposited to the Batcher Payments smart contract.

This way, the Batcher can only use User funds to fund the verification of the User's proofs.

## Helpful commands

If you are a User, and want to fund your Batcher so that he can include your proofs in its batch, you can send funds with the following command:

```bash
cast send <batcher_payments_smart_contract_address> --value <desired_amount_to_transfer> --rpc-url <your_rpc_url> --private-key <your_private_key>
```

For example:
```bash
cast send 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 --value 1ether --rpc-url http://localhost:8545 --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
```

After sending funds, you can check your current balance:
```bash
cast call 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 "UserBalances(address)(uint256)" 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720
```

The Batcher will then call something similar to the following command, to submit the batch to Aligned:
```bash
cast send 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 "createNewTask(bytes32, string, address[], uint256)" 0xc1b2a3c3aec88bb41478922438b0698add6a9a6c57170176115bda61748df59a "http://storage.alignedlayer.com/c1b2a3c3aec88bb41478922438b0698add6a9a6c57170176115bda61748df59a.json" "[0xa0Ee7A142d267C1f36714E4a8F75612F20a79720]" 1000000000000000 --private-key 0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba
```

Finally, the User can extract excess funds from the smart contract if he desires, or he can leave them there to fund future proofs:

```bash
cast send 0x7969c5eD335650692Bc04293B07F5BF2e7A673C0 "withdraw(uint256)" 1000 --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
```
