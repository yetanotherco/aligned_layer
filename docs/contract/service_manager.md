## AlignedLayerServiceManager

The [AlignedLayerServiceManager contract](https://github.com/yetanotherco/aligned_layer/blob/main/contracts/src/core/AlignedLayerServiceManager.sol) serves the purpose of confirming data stored by the disperser using inferred aggregated signatures of the quorum and freezing operators in response to various challenges. It imports contracts from [EigenLayer core contracts](https://github.com/Layr-Labs/eigenlayer-contracts/tree/master) and [EigenLayer Middleware contracts](https://github.com/Layr-Labs/eigenlayer-middleware) and is still under development.
 
The current functionality is as follows:
 
1. The [constructor](https://github.com/yetanotherco/aligned_layer/blob/main/contracts/src/core/AlignedLayerServiceManager.sol#L42) initializes the contract with the provided AVSDirectory, RegistryCoordinator, and StakeRegistry contracts. It also disables initializers after the contract is initialized.
 
2. The [initialize function](https://github.com/yetanotherco/aligned_layer/blob/main/contracts/src/core/AlignedLayerServiceManager.sol#L57) allows the contract owner to initialize the contract with the initial owner and aggregator addresses.
 
3. The [_setAggregator function](https://github.com/yetanotherco/aligned_layer/blob/main/contracts/src/core/AlignedLayerServiceManager.sol#L65) sets the aggregator address internally.
 
4. The [isAggregator function](https://github.com/yetanotherco/aligned_layer/blob/main/contracts/src/core/AlignedLayerServiceManager.sol#L69) returns a boolean value indicating whether the provided address is the aggregator address.
 
5. The [createNewTask function](https://github.com/yetanotherco/aligned_layer/blob/main/contracts/src/core/AlignedLayerServiceManager.sol#L73) allows for the creation of a new batch with the specified `batchMerkleRoot` and `batchDataPointer`. It ensures that the specified batch has not already been verified by checking the state in `batchesState`. If the batch is new, it records the creation block, sets the response status to false, and updates the state in `batchesState` with this information. Finally, it emits a `NewBatch` event with the batch details, including the Merkle root, block number, and data pointer.
 
6. The [respondToTask function](https://github.com/yetanotherco/aligned_layer/blob/main/contracts/src/core/AlignedLayerServiceManager.sol#L92) is used to validate a batch by checking signatures and ensuring that a threshold is met. It verifies that the batch corresponding to the given Merkle root has not already been responded to and checks the validity of the aggregated BLS signature. The function then confirms that the signatories own at least a specified threshold percentage of a quorum. Finally, it emits an event indicating that the batch has been verified.
