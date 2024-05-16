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
    event NewBatch(
        bytes32 batchMerkleRoot,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );

    event BatchVerified(bytes32 batchMerkleRoot);

    uint256 internal constant _THRESHOLD_DENOMINATOR = 100;
    uint8 internal constant QUORUM_THRESHOLD_PERCENTAGE = 67;

    struct BatchState {
        uint32 taskCreatedBlock;
        bool responded;
    }
    //mapping(uint32 => bytes32) public taskHashes;

    /* STORAGE */
    mapping(bytes32 => BatchState) batchesState;

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

    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer
    ) external payable {
        BatchState memory batchState;

        batchState.taskCreatedBlock = uint32(block.number);
        batchState.responded = false;

        batchesState[batchMerkleRoot] = batchState;

        emit NewBatch(batchMerkleRoot, uint32(block.number), batchDataPointer);
    }

    function respondToTask(
        // Root is signed as a way to verify the batch was right
        bytes32 batchMerkleRoot,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {
        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */

        // Check task hasn't been responsed yet
        // Note: This is a hacky solidity way to see that the element exists
        // Value 0 would mean that the task is in block 0 so this can't happen.

        require(
            batchesState[batchMerkleRoot].taskCreatedBlock != 0,
            "Batch doesn't exists"
        );

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // check that aggregated BLS signature is valid
        (
            QuorumStakeTotals memory quorumStakeTotals,
            bytes32 hashOfNonSigners
        ) = checkSignatures(
                batchMerkleRoot,
                batchesState[batchMerkleRoot].taskCreatedBlock,
                nonSignerStakesAndSignature
            );

        // check that signatories own at least a threshold percentage of each quourm
        require(
            quorumStakeTotals.signedStakeForQuorum[0] *
                _THRESHOLD_DENOMINATOR >=
                quorumStakeTotals.totalStakeForQuorum[0] *
                    QUORUM_THRESHOLD_PERCENTAGE,
            "Signatories do not own at least threshold percentage of a quorum"
        );

        emit BatchVerified(batchMerkleRoot);
    }
}
