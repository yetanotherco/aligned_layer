// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import {stdStorage, StdStorage} from "forge-std/Test.sol";
import "../src/core/AlignedLayerServiceManager.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IRewardsCoordinator} from "eigenlayer-contracts/src/contracts/interfaces/IRewardsCoordinator.sol";
import {BLSMockAVSDeployer} from "eigenlayer-middleware/../test/utils/BLSMockAVSDeployer.sol";

contract AlignedLayerServiceManagerTest is BLSMockAVSDeployer {
    AlignedLayerServiceManager alignedLayerServiceManager;
    address initialOwner = address(0x123);
    address aggregator = address(0x456);

    using stdStorage for StdStorage;

    event NewBatchV2(
        bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer
    );
    event NewBatchV3(
        bytes32 indexed batchMerkleRoot,
        address senderAddress,
        uint32 taskCreatedBlock,
        string batchDataPointer,
        uint256 maxFeeToRespond
    );

    struct BatchIdentifier {
        bytes32 batchMerkleRoot;
        address senderAddress;
    }

    event BatchVerified(bytes32 batchMerkleRoot);

    function setUp() public virtual {
        _setUpBLSMockAVSDeployer();

        alignedLayerServiceManager = new AlignedLayerServiceManager(
            avsDirectory,
            IRewardsCoordinator(address(rewardsCoordinator)),
            IRegistryCoordinator(address(registryCoordinator)),
            IStakeRegistry(address(stakeRegistry))
        );

        // alignedLayerServiceManager.initialize(initialOwner, aggregator);
    }

    // function testInitialize() public {
    //     assertEq(alignedLayerServiceManager.owner(), initialOwner);
    //     assertEq(alignedLayerServiceManager.isAggregator(aggregator), true);
    // }

    function testCreateNewTask(string memory root, string memory batchDataPointer, uint256 maxFeeToRespond) public {
        vm.assume(bytes(batchDataPointer).length > 50);
        bytes32 batchMerkleRoot = keccak256(abi.encodePacked(root));

        address batcher = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
        hoax(batcher, maxFeeToRespond);

        // transfer to serviceManager
        address(alignedLayerServiceManager).call{value: maxFeeToRespond}("");

        vm.expectEmit(true, true, true, true);
        emit NewBatchV3(batchMerkleRoot, batcher, uint32(block.number), batchDataPointer, maxFeeToRespond);

        vm.prank(batcher);
        alignedLayerServiceManager.createNewTask(batchMerkleRoot, batchDataPointer, maxFeeToRespond);

        bytes32 batchIdentifierHash = keccak256(abi.encodePacked(batchMerkleRoot, batcher));

        (uint32 taskCreatedBlock, bool responded, uint256 _maxFeeToRespond) =
            alignedLayerServiceManager.batchesState(batchIdentifierHash);

        assertEq(taskCreatedBlock, uint32(block.number));
        assertEq(responded, false);
        assertEq(_maxFeeToRespond, maxFeeToRespond);
    }
}
