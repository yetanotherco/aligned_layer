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
    event NewTaskCreated(
        uint32 indexed taskIndex,
        Task newTask,
    );

    event TaskResponded(uint32 indexed taskIndex, TaskResponse taskResponse);

    uint256 internal constant _THRESHOLD_DENOMINATOR = 100;
    bytes[] internal constant QUORUM_NUMBERS = [0];
    uint8 internal constant QUORUM_THRESHOLD_PERCENTAGE = 67;

    struct BatchState {
        uint32 taskCreatedBlock;
        bool responded;
    }

    /* STORAGE */
    mapping(bytes32 => BatchState)

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
        uint256 batchMerkleRoot,
        string calldata dataStorePointer
    ) external payable {
        BatchState memory batchState;

        batchState.taskCreatedBlock = block.number;
        batchState.responded = false;

        batchesState[batchMerkleRoot] = batchState; 

        /* Esto va, ahora lo ponemos
        emit NewTaskCreated(
            latestTaskIndexPlusOne,
            newTask,
        );
        */
    }

    function respondToTask(
        // Index is a hint, the operator doesn't sign it
        uint32 taskIndex,
        // Root is signed as a way to verify the batch was right
        uint256 batchMerkleRoot,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {
        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */

        // Validate the root in the index hint coincides with the signed information
        require(
            batchesState[taskIndex].batchMerkleRoot == batchMerkleRoot,
            "Task in index doesn't match the provided root"
        );

        // Check task hasn't been responsed yet
        require(
            batchesState[taskIndex].responded == false,
            "Aggregator has already responded to the task"
        );

 

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // calculate message which operators signed
        // operator signed merkleRoot
        bytes32 message = keccak256(batchMerkleRoot);

        // check that aggregated BLS signature is valid
        (
            QuorumStakeTotals memory quorumStakeTotals,
            bytes32 hashOfNonSigners
        ) = checkSignatures(
                message,
                QUORUM_NUMBERS,
                taskCreatedBlock,
                nonSignerStakesAndSignature
            );

        // check that signatories own at least a threshold percentage of each quourm
        for (uint i = 0; i < quorumNumbers.length; i++) {
            require(
                quorumStakeTotals.signedStakeForQuorum[i] *
                    _THRESHOLD_DENOMINATOR >=
                    quorumStakeTotals.totalStakeForQuorum[i] *
                        QUORUM_THRESHOLD_PERCENTAGE,
                "Signatories do not own at least threshold percentage of a quorum"
            );
        }

        emit TaskResponded(
            taskIndex
        );
    }
}
