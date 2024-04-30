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
    event NewTaskCreated(uint64 indexed taskIndex, Task task);
    event TaskResponded(TaskResponse taskResponse);

    // STRUCTS
    struct Task {
        uint16 verificationSystemId;
        bytes proof;
        bytes pubInput;
        uint32 taskCreatedBlock;
        uint8 quorumThresholdPercentage;
    }

    // Task Response
    // In case of changing this response, change AbiEncodeTaskResponse
    // since it won't be updated automatically
    struct TaskResponse {
        uint64 taskIndex;
        bool proofIsCorrect;
    }

    /* STORAGE */
    // The latest task index
    uint64 public latestTaskNum;

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
        uint16 verificationSystemId,
        bytes calldata proof,
        bytes calldata pubInput,
        uint8 quorumThresholdPercentage
    ) external {
        // create a new task struct
        Task memory newTask;
        newTask.verificationSystemId = verificationSystemId;
        newTask.proof = proof;
        newTask.pubInput = pubInput;
        newTask.taskCreatedBlock = uint32(block.number);
        newTask.quorumThresholdPercentage = quorumThresholdPercentage;

        emit NewTaskCreated(latestTaskNum, newTask);
        latestTaskNum = latestTaskNum + 1;
    }

    function respondToTask(
        uint64 taskIndex,
        bool proofIsCorrect // TODO: aggregated signature field
    ) external {
        // TODO: actually do something with the aggregated signature
        emit TaskResponded(TaskResponse(taskIndex, proofIsCorrect));
    }
}
