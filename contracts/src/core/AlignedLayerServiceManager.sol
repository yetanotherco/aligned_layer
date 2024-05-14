// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {Pausable} from "eigenlayer-core/contracts/permissions/Pausable.sol";
import {IPauserRegistry} from "eigenlayer-core/contracts/interfaces/IPauserRegistry.sol";

import {ServiceManagerBase, IAVSDirectory} from "eigenlayer-middleware/ServiceManagerBase.sol";
import {BLSSignatureChecker} from "eigenlayer-middleware/BLSSignatureChecker.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";

/**
 * @title Primary entrypoint for procuring services from AlignedLayer.
 * @author Layr Labs, Inc.
 * @notice This contract is used for:
 * - confirming the data store by the disperser with inferred aggregated signatures of the quorum
 * - freezing operators as the result of various "challenges"
 */
contract AlignedLayerServiceManager is ServiceManagerBase, BLSSignatureChecker {
    address aggregator;

    // EVENTS
    event NewTaskCreated(uint32 indexed taskIndex, Task task);
    event TaskResponded(uint32 indexed taskIndex, TaskResponse taskResponse);

    uint256 internal constant _THRESHOLD_DENOMINATOR = 100;

    // STRUCTS
    enum DASolution {
        Calldata,
        EigenDA,
        Celestia,
        S3Bucket
    }

    struct DAPayload {
        DASolution solution;
        bytes proof_associated_data; // Proof bytes for calldata - BatchHeaderHash for EigenDA - Commitment for Celestia
        uint64 index; // BlobIndex for EigenDA - Height for Celestia
    }

    struct Task {
        uint16 provingSystemId;
        DAPayload DAPayload;
        bytes pubInput;
        bytes verificationKey;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        bytes quorumThresholdPercentages;
        uint256 fee;
    }

    // Task Response
    // In case of changing this response, change AbiEncodeTaskResponse
    // since it won't be updated automatically
    struct TaskResponse {
        uint32 taskIndex;
        bool proofIsCorrect;
    }

    /* STORAGE */
    uint32 public latestTaskIndexPlusOne;

    mapping(uint32 => bytes32) public taskHashes;

    // mapping of task indices to hash of abi.encode(taskResponse, taskResponseMetadata)
    mapping(uint32 => bytes32) public taskResponses;

    constructor(
        IAVSDirectory __avsDirectory,
        IRegistryCoordinator __registryCoordinator,
        IStakeRegistry __stakeRegistry
    )
        BLSSignatureChecker(__registryCoordinator)
        ServiceManagerBase(
            __avsDirectory,
            __registryCoordinator,
            __stakeRegistry
        )
    {
        _disableInitializers();
    }

    function initialize(
        address _initialOwner,
        address _aggregator
    ) public initializer {
        _transferOwnership(_initialOwner);
        _setAggregator(_aggregator);
    }

    function _setAggregator(address _aggregator) internal {
        aggregator = _aggregator;
    }

    function isAggregator(address _aggregator) public view returns (bool) {
        return aggregator == _aggregator;
    }

    // NOTE(marian): Dummy function for testing contract integration
    function getMeaning() external view returns (uint) {
        return 42;
    }

    function createNewTask(
        uint16 provingSystemId,
        DAPayload calldata payload,
        bytes calldata pubInput,
        // This parameter is only mandatory for KZG based proving systems
        bytes calldata verificationKey,
        bytes calldata quorumNumbers,
        bytes calldata quorumThresholdPercentages
    ) external payable {
        require(msg.value > 0, "fee must be greater than 0");

        Task memory newTask;

        newTask.provingSystemId = provingSystemId;
        newTask.DAPayload = payload;
        newTask.pubInput = pubInput;
        newTask.verificationKey = verificationKey;
        newTask.taskCreatedBlock = uint32(block.number);
        newTask.quorumNumbers = quorumNumbers;
        newTask.quorumThresholdPercentages = quorumThresholdPercentages;
        newTask.fee = msg.value;

        taskHashes[latestTaskIndexPlusOne] = keccak256(abi.encode(newTask));

        emit NewTaskCreated(latestTaskIndexPlusOne, newTask);

        latestTaskIndexPlusOne = latestTaskIndexPlusOne + 1;
    }

    function respondToTask(
        Task calldata task,
        TaskResponse calldata taskResponse,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {
        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        uint32 taskCreatedBlock = task.taskCreatedBlock;
        bytes calldata quorumNumbers = task.quorumNumbers;
        bytes calldata quorumThresholdPercentages = task
            .quorumThresholdPercentages;

        // check that the task is valid, hasn't been responsed yet, and is being responsed in time
        require(
            keccak256(abi.encode(task)) == taskHashes[taskResponse.taskIndex],
            "Supplied task does not match the one recorded in the contract"
        );

        require(
            taskResponses[taskResponse.taskIndex] == bytes32(0),
            "Aggregator has already responded to the task"
        );

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // calculate message which operators signed
        bytes32 message = keccak256(abi.encode(taskResponse));

        // check that aggregated BLS signature is valid
        (
            QuorumStakeTotals memory quorumStakeTotals,
            bytes32 hashOfNonSigners
        ) = checkSignatures(
                message,
                quorumNumbers,
                taskCreatedBlock,
                nonSignerStakesAndSignature
            );

        // check that signatories own at least a threshold percentage of each quourm
        for (uint i = 0; i < quorumNumbers.length; i++) {
            require(
                quorumStakeTotals.signedStakeForQuorum[i] *
                    _THRESHOLD_DENOMINATOR >=
                    quorumStakeTotals.totalStakeForQuorum[i] *
                        uint8(quorumThresholdPercentages[i]),
                "Signatories do not own at least threshold percentage of a quorum"
            );
        }

        payable(aggregator).transfer(task.fee);

        emit TaskResponded(
            taskResponse.taskIndex,
            TaskResponse(taskResponse.taskIndex, taskResponse.proofIsCorrect)
        );
    }
}
