// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import {Pausable} from "eigenlayer-core/contracts/permissions/Pausable.sol";
import {IPauserRegistry} from "eigenlayer-core/contracts/interfaces/IPauserRegistry.sol";

import {ServiceManagerBase, IAVSDirectory} from "eigenlayer-middleware/ServiceManagerBase.sol";
import {BLSSignatureChecker} from "eigenlayer-middleware/BLSSignatureChecker.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";

/**
 * @title Primary entrypoint for procuring services from Aligned.
 * @author Layr Labs, Inc.
 * @notice This contract is used for:
 * -
 * -
 */
contract AlignedLayerServiceManager is ServiceManagerBase, BLSSignatureChecker {
    address aggregator;

    uint256 internal constant THRESHOLD_DENOMINATOR = 100;
    uint8 internal constant QUORUM_THRESHOLD_PERCENTAGE = 67;

    // EVENTS
    event NewBatch(
        bytes32 indexed batchMerkleRoot,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );

    event BatchVerified(bytes32 indexed batchMerkleRoot);

    struct BatchState {
        uint32 taskCreatedBlock;
        bool responded;
    }

    struct BatchInclusionData {
        bytes32 batchMerkleRoot;
        bytes32 proofCommitment;
    }

    /* STORAGE */
    mapping(bytes32 => BatchState) public batchesState;

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
        require(
            batchesState[batchMerkleRoot].taskCreatedBlock == 0,
            "Batch was already verified"
        );

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

        // Note: This is a hacky solidity way to see that the element exists
        // Value 0 would mean that the task is in block 0 so this can't happen.
        require(
            batchesState[batchMerkleRoot].taskCreatedBlock != 0,
            "Batch doesn't exists"
        );

        // Check task hasn't been responsed yet
        require(
            batchesState[batchMerkleRoot].responded == false,
            "Batch already responded"
        );
        batchesState[batchMerkleRoot].responded = true;

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
            quorumStakeTotals.signedStakeForQuorum[0] * THRESHOLD_DENOMINATOR >=
                quorumStakeTotals.totalStakeForQuorum[0] *
                    QUORUM_THRESHOLD_PERCENTAGE,
            "Signatories do not own at least threshold percentage of a quorum"
        );

        emit BatchVerified(batchMerkleRoot);
    }

    function verifyBatchInclusion(
        // bytes32 batchMerkleRoot,
        // bytes32 proofCommitment,
        // bytes32 pubInputCommitment,
        // bytes32 provingSystemAuxDataCommitment,
        // bytes20 proofGeneratorAddr,
        // bytes batchInclusionProof
        // BatchInclusionData calldata batchInclusionData
        bytes32 batchMerkleRoot,
        bytes32 proofCommitment
    ) external returns (bool) {
        return true;
    }
}
