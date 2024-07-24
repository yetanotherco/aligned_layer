// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import {Pausable} from "eigenlayer-core/contracts/permissions/Pausable.sol";
import {IPauserRegistry} from "eigenlayer-core/contracts/interfaces/IPauserRegistry.sol";

import {ServiceManagerBase, IAVSDirectory} from "eigenlayer-middleware/ServiceManagerBase.sol";
import {BLSSignatureChecker} from "eigenlayer-middleware/BLSSignatureChecker.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";
import {Merkle} from "eigenlayer-core/contracts/libraries/Merkle.sol";
import {IRewardsCoordinator} from "eigenlayer-contracts/src/contracts/interfaces/IRewardsCoordinator.sol";
import {AlignedLayerServiceManagerStorage} from "./AlignedLayerServiceManagerStorage.sol";

/**
 * @title Primary entrypoint for procuring services from Aligned.
 */
contract AlignedLayerServiceManager is
    ServiceManagerBase,
    BLSSignatureChecker,
    AlignedLayerServiceManagerStorage
{
    uint256 internal constant THRESHOLD_DENOMINATOR = 100;
    uint8 internal constant QUORUM_THRESHOLD_PERCENTAGE = 67;

    // EVENTS
    event NewBatch(
        bytes32 indexed batchMerkleRoot,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );

    event BatchVerified(bytes32 indexed batchMerkleRoot);

    constructor(
        IAVSDirectory __avsDirectory,
        IRewardsCoordinator __rewardsCoordinator,
        IRegistryCoordinator __registryCoordinator,
        IStakeRegistry __stakeRegistry
    )
        BLSSignatureChecker(__registryCoordinator)
        ServiceManagerBase(
            __avsDirectory,
            __rewardsCoordinator,
            __registryCoordinator,
            __stakeRegistry
        )
    {
        _disableInitializers();
    }

    function initialize(address _initialOwner) public initializer {
        _transferOwnership(_initialOwner);
    }

    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer
    ) external payable {
        require(
            batchesState[batchMerkleRoot].taskCreatedBlock == 0,
            "Batch was already submitted"
        );

        if (msg.value > 0) {
            batchersBalances[msg.sender] += msg.value;
        }

        require(batchersBalances[msg.sender] > 0, "Batcher balance is empty");

        BatchState memory batchState;

        batchState.taskCreatedBlock = uint32(block.number);
        batchState.responded = false;
        batchState.batcherAddress = msg.sender;

        batchesState[batchMerkleRoot] = batchState;

        emit NewBatch(batchMerkleRoot, uint32(block.number), batchDataPointer);
    }

    function respondToTask(
        // Root is signed as a way to verify the batch was right
        bytes32 batchMerkleRoot,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {
        uint256 initialGasLeft = gasleft();

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

        require(
            batchersBalances[batchesState[batchMerkleRoot].batcherAddress] > 0,
            "Batcher has no balance"
        );

        batchesState[batchMerkleRoot].responded = true;

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // check that aggregated BLS signature is valid
        (
            QuorumStakeTotals memory quorumStakeTotals,
            bytes32 _hashOfNonSigners
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

        // Calculate estimation of gas used, check that batcher has sufficient funds
        // and send transaction cost to aggregator.
        uint256 finalGasLeft = gasleft();

        // 70k was measured by trial and error until the aggregator got paid a bit over what it needed
        uint256 txCost = (initialGasLeft - finalGasLeft + 70000) * tx.gasprice;

        require(
            batchersBalances[batchesState[batchMerkleRoot].batcherAddress] >=
                txCost,
            "Batcher has not sufficient funds for paying this transaction"
        );

        batchersBalances[
            batchesState[batchMerkleRoot].batcherAddress
        ] -= txCost;
        payable(msg.sender).transfer(txCost);
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) external view returns (bool) {
        if (batchesState[batchMerkleRoot].taskCreatedBlock == 0) {
            return false;
        }

        if (!batchesState[batchMerkleRoot].responded) {
            return false;
        }

        bytes memory leaf = abi.encodePacked(
            proofCommitment,
            pubInputCommitment,
            provingSystemAuxDataCommitment,
            proofGeneratorAddr
        );

        bytes32 hashedLeaf = keccak256(leaf);

        return
            Merkle.verifyInclusionKeccak(
                merkleProof,
                batchMerkleRoot,
                hashedLeaf,
                verificationDataBatchIndex
            );
    }

    function balanceOf(address account) public view returns (uint256) {
        return batchersBalances[account];
    }

    receive() external payable {
        batchersBalances[msg.sender] += msg.value;
    }

    function checkPublicInput(
        bytes calldata publicInput,
        bytes32 hash
    ) public pure returns (bool) {
        return keccak256(publicInput) == hash;
    }
}
