# Pausable
This doc contains a guide on how to use the Pausable functionality of Aligned.

To run the make targets specified in this guide, you must first have the relevant following env vars:
```
export RPC_URL=<rpc_url>
export ALIGNED_SERVICE_MANAGER=<aligned_contract_address>
export ALIGNED_SERVICE_MANAGER_PAUSER_PRIVATE_KEY=<aligned_service_manager_pauser_private_key>
export BATCHER_PAYMENT_SERVICE=<payment_service_contract_address>
export BATCHER_PAYMENT_SERVICE_PAUSER_PRIVATE_KEY=<batcher_payment_service_pauser_private_key>
```

## Aligned Service Manager

Aligned Service Manager is granulary pausable, which means you can pause the whole contract, or only specific functions. For this,
Aligned uses the Pauser Registry contract provided by Eigenlayer. This contract stores the role of different accounts, so
you can have X pausers and Y unpausers.

To interact with it you can:

- Get current paused state:
```
make get_paused_state_aligned_service_manager
```

- Pause or Unpause the whole aligned service manager contract:
```
make pause_all_aligned_service_manager
```
```
make unpause_all_aligned_service_manager
```

- Pause only specific functions, receiving a list of the functions to pause/remain paused:
For example, if you want to pause functions 0, 2 and 3, you can run
```
contracts/scripts/pause_aligned_service_manager.sh 0 2 3
```
Then, if you want to unpause, for example, function 2, you must run
```
contracts/scripts/unpause_aligned_service_manager.sh 0 3
```

Note: when executing a Pause, you can only ADD functions to the paused list, and when executing an Unpause, you can only REMOVE functions from the paused list. This is because the base pausable contract has different ACL for Pausers and Unpausers.

Note: the list of pausable functions and their numbers can be seen in the `AlignedLayerServiceManager.sol` contract. But the list is the following:

0. createNewTask
1. respondToTaskV2
2. verifyBatchInclusion
3. withdraw
4. depositToBatcher
5. receive

## BatcherPaymentsService

BatcherPayments is also pausable, but without so much detail. You can either pause or unpause the contract running the following:

- Get current paused state:
```
make get_paused_state_batcher_payments_service
```

```
make pause_batcher_payment_service
```
```
make unpause_batcher_payment_service
```

And this will either pause or unpause the following functions:
- createNewTask
- unlock
- lock
- withdraw
