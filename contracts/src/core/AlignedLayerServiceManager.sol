// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

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
        if (address(__avsDirectory) == address(0)) {
            revert InvalidAddress("avsDirectory");
        }
        if (address(__rewardsCoordinator) == address(0)) {
            revert InvalidAddress("rewardsCoordinator");
        }
        if (address(__registryCoordinator) == address(0)) {
            revert InvalidAddress("registryCoordinator");
        }
        if (address(__stakeRegistry) == address(0)) {
            revert InvalidAddress("stakeRegistry");
        }
        _disableInitializers();
    }

    // @param _rewardsInitiator The address which is allowed to create AVS rewards submissions.
    function initialize(
        address _initialOwner,
        address _rewardsInitiator,
        address _alignedAggregator
    ) public initializer {
        if (_initialOwner == address(0)) {
            revert InvalidAddress("initialOwner");
        }
        if (_rewardsInitiator == address(0)) {
            revert InvalidAddress("rewardsInitiator");
        }
        if (_alignedAggregator == address(0)) {
            revert InvalidAddress("alignedAggregator");
        }
        __ServiceManagerBase_init(_initialOwner, _rewardsInitiator);
        alignedAggregator = _alignedAggregator; //can't do setAggregator(aggregator) since caller is not the owner
    }

    // This function is to be run only on upgrade
    // If a new contract is deployed, this function should be removed
    // Because this new value is also added in the initializer
    function initializeAggregator(
        address _alignedAggregator
    ) public reinitializer(2) {
        setAggregator(_alignedAggregator);
    }

    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer,
        uint256 respondToTaskFeeLimit
    ) external payable {
        bytes32 batchIdentifier = keccak256(
            abi.encodePacked(batchMerkleRoot, msg.sender)
        );

        if (batchesState[batchIdentifier].taskCreatedBlock != 0) {
            revert BatchAlreadySubmitted(batchIdentifier);
        }

        if (msg.value > 0) {
            batchersBalances[msg.sender] += msg.value;
            emit BatcherBalanceUpdated(
                msg.sender,
                batchersBalances[msg.sender]
            );
        }

        if (batchersBalances[msg.sender] < respondToTaskFeeLimit) {
            revert InsufficientFunds(
                msg.sender,
                respondToTaskFeeLimit,
                batchersBalances[msg.sender]
            );
        }

        BatchState memory batchState;

        batchState.taskCreatedBlock = uint32(block.number);
        batchState.responded = false;
        batchState.respondToTaskFeeLimit = respondToTaskFeeLimit;

        batchesState[batchIdentifier] = batchState;

        // For aggregator and operators in v0.7.0
        emit NewBatchV3(
            batchMerkleRoot,
            msg.sender,
            uint32(block.number),
            batchDataPointer,
            respondToTaskFeeLimit
        );
    }

    function respondToTaskV2(
        // (batchMerkleRoot,senderAddress) is signed as a way to verify the batch was right
        bytes32 batchMerkleRoot,
        address senderAddress,
        NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external onlyAggregator {
        uint256 initialGasLeft = gasleft();

        bytes32 batchIdentifierHash = keccak256(
            abi.encodePacked(batchMerkleRoot, senderAddress)
        );

        BatchState storage currentBatch = batchesState[batchIdentifierHash];

        // Note: This is a hacky solidity way to see that the element exists
        // Value 0 would mean that the task is in block 0 so this can't happen.
        if (currentBatch.taskCreatedBlock == 0) {
            revert BatchDoesNotExist(batchIdentifierHash);
        }

        // Check task hasn't been responsed yet
        if (currentBatch.responded) {
            revert BatchAlreadyResponded(batchIdentifierHash);
        }
        currentBatch.responded = true;

        // Check that batcher has enough funds to fund response
        if (
            batchersBalances[senderAddress] < currentBatch.respondToTaskFeeLimit
        ) {
            revert InsufficientFunds(
                senderAddress,
                currentBatch.respondToTaskFeeLimit,
                batchersBalances[senderAddress]
            );
        }

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */

        // check that aggregated BLS signature is valid
        (QuorumStakeTotals memory quorumStakeTotals, ) = checkSignatures(
            batchIdentifierHash,
            currentBatch.taskCreatedBlock,
            nonSignerStakesAndSignature
        );

        // check that signatories own at least a threshold percentage of each quourm
        if (
            quorumStakeTotals.signedStakeForQuorum[0] * THRESHOLD_DENOMINATOR <
            quorumStakeTotals.totalStakeForQuorum[0] *
                QUORUM_THRESHOLD_PERCENTAGE
        ) {
            revert InvalidQuorumThreshold(
                quorumStakeTotals.signedStakeForQuorum[0] *
                    THRESHOLD_DENOMINATOR,
                quorumStakeTotals.totalStakeForQuorum[0] *
                    QUORUM_THRESHOLD_PERCENTAGE
            );
        }

        emit BatchVerified(batchMerkleRoot, senderAddress);

        // 70k was measured by trial and error until the aggregator got paid a bit over what it needed
        uint256 txCost = (initialGasLeft - gasleft() + 70_000) * tx.gasprice;

        if (txCost > currentBatch.respondToTaskFeeLimit) {
            revert ExceededMaxRespondFee(
                currentBatch.respondToTaskFeeLimit,
                txCost
            );
        }

        // Subtract the txCost from the batcher's balance
        batchersBalances[senderAddress] -= txCost;
        emit BatcherBalanceUpdated(
            senderAddress,
            batchersBalances[senderAddress]
        );
        payable(alignedAggregator).transfer(txCost);
    }

    function isVerifierDisabled(
        uint8 verifierIdx
    ) external view returns (bool) {
        uint256 bit = disabledVerifiers & (1 << verifierIdx);
        return bit > 0;
    }

    function disableVerifier(
        uint8 verifierIdx
    ) external onlyOwner {
        disabledVerifiers |= (1 << verifierIdx);
        emit VerifierDisabled(verifierIdx);
    }

    function enableVerifier(
        uint8 verifierIdx
    ) external onlyOwner {
        disabledVerifiers &= ~(1 << verifierIdx);
        emit VerifierEnabled(verifierIdx);
    }

    function setDisabledVerifiers(uint256 bitmap) external onlyOwner {
        disabledVerifiers = bitmap;
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
        bytes32 batchIdentifier;
        if (senderAddress == address(0)) {
            batchIdentifier = batchMerkleRoot;
        } else {
            batchIdentifier = keccak256(
                abi.encodePacked(batchMerkleRoot, senderAddress)
            );
        }

        if (batchesState[batchIdentifier].taskCreatedBlock == 0) {
            return false;
        }

        if (!batchesState[batchIdentifier].responded) {
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

    // Old function signature for backwards compatibility
    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) external view returns (bool) {
        return
            this.verifyBatchInclusion(
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr,
                batchMerkleRoot,
                merkleProof,
                verificationDataBatchIndex,
                address(0)
            );
    }

    function setAggregator(address _alignedAggregator) public onlyOwner {
        alignedAggregator = _alignedAggregator;
    }

    function withdraw(uint256 amount) external {
        if (batchersBalances[msg.sender] < amount) {
            revert InsufficientFunds(
                msg.sender,
                amount,
                batchersBalances[msg.sender]
            );
        }

        batchersBalances[msg.sender] -= amount;
        emit BatcherBalanceUpdated(msg.sender, batchersBalances[msg.sender]);

        payable(msg.sender).transfer(amount);
    }

    function balanceOf(address account) public view returns (uint256) {
        return batchersBalances[account];
    }

    function depositToBatcher(address account) external payable {
        _depositToBatcher(account, msg.value);
    }

    function _depositToBatcher(address account, uint256 amount) internal {
        if (amount == 0) {
            revert InvalidDepositAmount(amount);
        }
        batchersBalances[account] += amount;
        emit BatcherBalanceUpdated(account, batchersBalances[account]);
    }

    receive() external payable {
        _depositToBatcher(msg.sender, msg.value);
    }

    function checkPublicInput(
        bytes calldata publicInput,
        bytes32 hash
    ) public pure returns (bool) {
        return keccak256(publicInput) == hash;
    }

    modifier onlyAggregator() {
        if (msg.sender != alignedAggregator) {
            revert SenderIsNotAggregator(msg.sender, alignedAggregator);
        }
        _;
    }
}
