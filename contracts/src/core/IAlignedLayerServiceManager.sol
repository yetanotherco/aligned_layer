// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import {IBLSSignatureChecker} from "eigenlayer-middleware/interfaces/IBLSSignatureChecker.sol";

interface IAlignedLayerServiceManager {
    // EVENTS
    event NewBatch(
        bytes32 indexed batchMerkleRoot,
        address senderAddress,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );
    event BatchVerified(bytes32 indexed batchMerkleRoot, address senderAddress);

    // ERRORS
    error BatchAlreadySubmitted(bytes32 batchIdentifierHash);
    error BatcherBalanceIsEmpty(address batcher);
    error BatchDoesNotExist(bytes32 batchIdentifierHash);
    error BatchAlreadyResponded(bytes32 batchIdentifierHash);
    error BatcherHasNoBalance(address batcher);
    error InsufficientFunds(
        address batcher,
        uint256 required,
        uint256 available
    );
    error InvalidQuorumThreshold(uint256 signedStake, uint256 requiredStake);

    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer
    ) external payable;

    function respondToTask(
        bytes32 batchMerkleRoot,
        address senderAddress,
        IBLSSignatureChecker.NonSignerStakesAndSignature
            memory nonSignerStakesAndSignature
    ) external;

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        address senderAddress
    ) external view returns (bool);

    function balanceOf(address account) external view returns (uint256);
}
