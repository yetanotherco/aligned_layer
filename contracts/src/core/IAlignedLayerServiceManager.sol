// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import {IBLSSignatureChecker} from "eigenlayer-middleware/interfaces/IBLSSignatureChecker.sol";

interface IAlignedLayerServiceManager {
    // EVENTS
    event NewBatchV2(
        bytes32 indexed batchMerkleRoot,
        address senderAddress,
        uint32 taskCreatedBlock,
        string batchDataPointer,
        uint256 maxFeeToRespond
    );
    event BatchVerified(bytes32 indexed batchMerkleRoot, address senderAddress);
    event BatcherBalanceUpdated(address indexed batcher, uint256 newBalance);

    // ERRORS
    error BatchAlreadySubmitted(bytes32 batchIdentifierHash); // 3102f10c
    error BatchDoesNotExist(bytes32 batchIdentifierHash); // 2396d34e
    error BatchAlreadyResponded(bytes32 batchIdentifierHash); // 9cf1aff2
    error InsufficientFunds(
        address batcher,
        uint256 required,
        uint256 available
    ); // 5c54305e
    error InvalidQuorumThreshold(uint256 signedStake, uint256 requiredStake); // a61eb88a
    error InvalidDepositAmount(uint256 amount); // 412ed242
    error ExceededMaxRespondFee(uint256 maxFeeAllowedToRespond, uint256 txCost); // 86fc507e

    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer,
        uint256 maxFeeToRespond
    ) external payable;

    function respondToTaskV2(
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
