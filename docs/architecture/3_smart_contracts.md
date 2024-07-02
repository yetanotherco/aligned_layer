# Smart contracts

## Aligned Layer Service Manager
Besides the base [EigenLayer middleware contracts](https://github.com/Layr-Labs/eigenlayer-middleware/tree/mainnet/src), the core contract for Aligned is [AlignedLayerServiceManager](../../contracts/src/core/AlignedLayerServiceManager.sol). It is in charge of creating new batch verification tasks, storing batches state and verify operator responses.

### API 

#### Create new task

```solidity
function createNewTask(
    bytes32 batchMerkleRoot,
    string calldata batchDataPointer
) external payable
```

This method is called to create a new batch verification task that will broadcast an event to all operators, signaling that there are new proofs awaiting to be verified.
* `batchMerkleRoot` is a 256 bit hash corresponding to the merkle root of the proofs batch to be verified by operators.
* `batchDataPointer` is a string representing a link to some specific data storage location. This is used by operators to download the entire batch of proofs.

#### Respond to task

```solidity
function respondToTask(
    bytes32 batchMerkleRoot,
    NonSignerStakesAndSignature memory nonSignerStakesAndSignature
) external
```

This method is used by the Aggregator once the quorum for a particular task has been reached. Its main purpose is to verify the aggregated signature of the operators for the given task, and also that the quorum was reached. After verifying, an event is emmited signaling to any consumer that the batch has reached soft finality. 
* `batchMerkleRoot` is a 256 bit hash representing the merkle root of the batch that has been verified and signed by operators.
* `nonSignerStakesAndSignature` is a struct provided by EigenLayer middleware with information about operators signatures, stakes and quorum for the given task. 

### Verify batch inclusion

```solidity
function verifyBatchInclusion(
    bytes32 proofCommitment,
    bytes32 pubInputCommitment,
    bytes32 provingSystemAuxDataCommitment,
    bytes20 proofGeneratorAddr,
    bytes32 batchMerkleRoot,
    bytes memory merkleProof,
    uint256 verificationDataBatchIndex
) external view returns (bool)
```

A method used for consumers to check that their proof was verified in Aligned. It checks that the batch were the proof was included was verified and that the proof was included in the batch verifying the merkle path.

* `proofCommitment`, `pubInputCommitment`, `provingSystemAuxDataCommitment`, `proofGeneratorAddr` are the commitments to the verification data sent to the batcher.
* `batchMerkleRoot` is a 256 bit hash representing the batch merkle root the proof was included in. 
* `merkleProof` is the merkle path from the hashed leaf built from the verification data commitments to the root.
* `verificationDataBatchIndex` is the index of the proof in the batch were it was included. 

## Deployments