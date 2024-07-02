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

## Holesky deployments

|   Contract                 |     Address                                |
|----------------------------|--------------------------------------------|
| AlignedLayerServiceManager | [0x58F280BeBE9B34c9939C3C39e0890C81f163B623](https://holesky.etherscan.io/address/0x58F280BeBE9B34c9939C3C39e0890C81f163B623) |
| BlsApkRegistry             | [0xD0A725d82649f9e4155D7A60B638Fe33b3F25e3b](https://holesky.etherscan.io/address/0xD0A725d82649f9e4155D7A60B638Fe33b3F25e3b) |
| IndexRegistry              | [0x4A7DE0a9fBBAa4fF0270d31852B363592F68B81F](https://holesky.etherscan.io/address/0x4A7DE0a9fBBAa4fF0270d31852B363592F68B81F) |
| OperatorStateRetriever     | [0x59755AF41dB1680dC6F47CaFc09e40C0e757C5E9](https://holesky.etherscan.io/address/0x59755AF41dB1680dC6F47CaFc09e40C0e757C5E9) |
| RegistryCoordinator        | [0x3aD77134c986193c9ef98e55e800B71e72835b62](https://holesky.etherscan.io/address/0x3aD77134c986193c9ef98e55e800B71e72835b62) |
| StakeRegistry              | [0x51462D5511563A0F97Bb3Ce5475E1c3905b83F4b](https://holesky.etherscan.io/address/0x51462D5511563A0F97Bb3Ce5475E1c3905b83F4b) |
| BatcherPaymentService      | [0x815aeCA64a974297942D2Bbf034ABEe22a38A003](https://holesky.etherscan.io/address/0x815aeCA64a974297942D2Bbf034ABEe22a38A003) |
