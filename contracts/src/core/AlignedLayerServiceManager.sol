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
import {IAlignedLayerServiceManager} from "./IAlignedLayerServiceManager.sol";

/**
 * @title Primary entrypoint for procuring services from Aligned.
 */
contract AlignedLayerServiceManager is
    IAlignedLayerServiceManager,
    ServiceManagerBase,
    BLSSignatureChecker,
    AlignedLayerServiceManagerStorage
{
    uint256 internal constant THRESHOLD_DENOMINATOR = 100;
    uint8 internal constant QUORUM_THRESHOLD_PERCENTAGE = 67;

    //old NewBatch event, for smooth Operator upgradeability
    event NewBatch(
        bytes32 indexed batchMerkleRoot,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );
    // EVENTS
    event NewBatch(
        bytes32 indexed batchMerkleRoot,
        address senderAddress,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );

    event BatchVerified(bytes32 indexed batchMerkleRoot, address senderAddress);
    event BatcherBalanceUpdated(address indexed batcher, uint256 newBalance);

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

    // @param _rewardsInitiator The address which is allowed to create AVS rewards submissions.
    function initialize(
        address _initialOwner,
        address _rewardsInitiator
    ) public initializer {
        __ServiceManagerBase_init(_initialOwner, _rewardsInitiator);
    }

    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer
    ) external payable {
        bytes32 batchIdentifierHash = keccak256(
            abi.encodePacked(batchMerkleRoot, msg.sender)
        );

        require(
            batchesState[batchIdentifierHash].taskCreatedBlock == 0,
            "Batch was already submitted"
        );

        if (msg.value > 0) {
            batchersBalances[msg.sender] += msg.value;
            emit BatcherBalanceUpdated(
                msg.sender,
                batchersBalances[msg.sender]
            );
        }

        require(batchersBalances[msg.sender] > 0, "Batcher balance is empty");

        BatchState memory batchState;

        batchState.taskCreatedBlock = uint32(block.number);
        batchState.responded = false;

        batchesState[batchIdentifierHash] = batchState;

        emit NewBatch(
            batchMerkleRoot,
            msg.sender,
            uint32(block.number),
            batchDataPointer
        );
        // old event for smooth Operator upgradeability:
        emit NewBatch(
            batchMerkleRoot,
            uint32(block.number),
            batchDataPointer
        );
    }

    // previous version of this function, for smooth upgradeability
    function respondToTask_old(
        // Root is signed as a way to verify the batch was right
        bytes32 batchMerkleRoot,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {
            // batcherAddress [address(0x7969c5eD335650692Bc04293B07F5BF2e7A673C0)] > 0, // Devnet
            // batcherAddress [address(0x7577Ec4ccC1E6C529162ec8019A49C13F6DAd98b)] > 0, // Stage
            // batcherAddress [address(0x815aeCA64a974297942D2Bbf034ABEe22a38A003)] > 0, // Prod
        address batcherAddress = address(0x7969c5eD335650692Bc04293B07F5BF2e7A673C0);
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
            batchersBalances[batcherAddress] > 0,
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

        emit BatchVerified(batchMerkleRoot, batcherAddress);

        // Calculate estimation of gas used, check that batcher has sufficient funds
        // and send transaction cost to aggregator.
        uint256 finalGasLeft = gasleft();
        // 70k was measured by trial and error until the aggregator got paid a bit over what it needed
        uint256 txCost = (initialGasLeft - finalGasLeft + 70000) * tx.gasprice;

        require(
            batchersBalances[batcherAddress] >=
                txCost,
            "Batcher has not sufficient funds for paying this transaction"
        );

        batchersBalances[
            batcherAddress
        ] -= txCost;
        payable(msg.sender).transfer(txCost);
    }

    function respondToTask(
        // (batchMerkleRoot,senderAddress) is signed as a way to verify the batch was right
        bytes32 batchMerkleRoot,
        address senderAddress,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {
        uint256 initialGasLeft = gasleft();

        bytes32 batchIdentifierHash = keccak256(
            abi.encodePacked(batchMerkleRoot, senderAddress)
        );

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */

        // Note: This is a hacky solidity way to see that the element exists
        // Value 0 would mean that the task is in block 0 so this can't happen.
        require(
            batchesState[batchIdentifierHash].taskCreatedBlock != 0,
            "Batch doesn't exists"
        );

        // Check task hasn't been responsed yet
        require(
            batchesState[batchIdentifierHash].responded == false,
            "Batch already responded"
        );

        require(batchersBalances[senderAddress] > 0, "Batcher has no balance");

        batchesState[batchIdentifierHash].responded = true;

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // check that aggregated BLS signature is valid
        (
            QuorumStakeTotals memory quorumStakeTotals,
            bytes32 _hashOfNonSigners
        ) = checkSignatures(
                batchIdentifierHash,
                batchesState[batchIdentifierHash].taskCreatedBlock,
                nonSignerStakesAndSignature
            );

        // check that signatories own at least a threshold percentage of each quourm
        require(
            quorumStakeTotals.signedStakeForQuorum[0] * THRESHOLD_DENOMINATOR >=
                quorumStakeTotals.totalStakeForQuorum[0] *
                    QUORUM_THRESHOLD_PERCENTAGE,
            "Signatories do not own at least threshold percentage of a quorum"
        );

        emit BatchVerified(batchMerkleRoot, senderAddress);

        // Calculate estimation of gas used, check that batcher has sufficient funds
        // and send transaction cost to aggregator.
        uint256 finalGasLeft = gasleft();

        // 70k was measured by trial and error until the aggregator got paid a bit over what it needed
        uint256 txCost = (initialGasLeft - finalGasLeft + 70000) * tx.gasprice;

        require(
            batchersBalances[senderAddress] >= txCost,
            "Batcher has not sufficient funds for paying this transaction"
        );

        batchersBalances[senderAddress] -= txCost;
        emit BatcherBalanceUpdated(
            senderAddress,
            batchersBalances[senderAddress]
        );
        payable(msg.sender).transfer(txCost);
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        address senderAddress
    ) external view returns (bool) {
        bytes32 batchIdentifierHash = keccak256(
            abi.encodePacked(batchMerkleRoot, senderAddress)
        );

        if (batchesState[batchIdentifierHash].taskCreatedBlock == 0) {
            return false;
        }

        if (!batchesState[batchIdentifierHash].responded) {
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
                batchIdentifierHash,
                hashedLeaf,
                verificationDataBatchIndex
            );
    }

    function balanceOf(address account) public view returns (uint256) {
        return batchersBalances[account];
    }

    receive() external payable {
        batchersBalances[msg.sender] += msg.value;
        emit BatcherBalanceUpdated(msg.sender, batchersBalances[msg.sender]);
    }

    function checkPublicInput(
        bytes calldata publicInput,
        bytes32 hash
    ) public pure returns (bool) {
        return keccak256(publicInput) == hash;
    }
}
