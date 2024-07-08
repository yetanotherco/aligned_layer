# Payment Service

The Payment Service handles User's payments to fund the verification of their proofs. 

To be able to use the batcher, a user must fund its transactions. For this, there is a simple Batcher Payment System.

The Batcher has an associated Batcher Payments smart contract, which is in charge of receiving user's payments, and it guarantees that it can only spend these funds to send users' proofs to Aligned.

Users must first deposit into this contract, via a normal transfer to its address, where the Batcher Payment System will update the User's balance.

Then, users can send proofs to the Batcher, the Batcher will preemptively check if the user has funds for this, and once the whole batch is assembled, the Batcher will call its smart contract with the data it has received from the users.

The smart contract will then discount the corresponding amount of funds from each of the senders' balances, and create a new Batch in [Aligned Service Manager](./3_service_manager_contract.md), sending with it the corresponding amount of tokens for the batch verification to be paid to the [Aggregator](./5_aggregator.md).

Users can then withdraw extra funds deposited to the Batcher Payments smart contract, or leave them to fund future proofs.

This way, the Batcher can only use User funds to pay for the verification of the User's proofs.

## Details of the contract

### API

#### Receive funds

```solidity
    receive() external payable
```

This function will be called every time a User transfers funds to the smart contract. It will not only receive the funds, but it will also register internally how much the User deposited, to keep track of each User's funds separately. 


#### Create New Task

```solidity
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer,
        address[] calldata proofSubmitters,
        uint256 gasForAggregator,
        uint256 gasPerProof
    ) external onlyBatcher
```

This function will be executed only by the Batcher, when it has a batch to post to Aligned. It contains all the information needed to post the batch in [Aligned Service Manager](./3_service_manager_contract.md) (`batchMerkleRoot` and `batchDataPointer`), plus an array containing which are the `proofSubmitters`, so as to discount `gasPerProof` from these, and also the `gasForAggregator`, declaring how much will need to go pay for the response of the batch.

#### Withdraw

```solidity
    function withdraw(uint256 amount) external
```

This function can be called by any User, to freely withdraw any amount of their available balance from the contract.
