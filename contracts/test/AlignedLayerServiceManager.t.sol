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

    event NewBatch(
        bytes32 indexed batchMerkleRoot,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );

    event BatchVerified(bytes32 batchMerkleRoot);

    function setUp() virtual public {
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

    function testCreateNewTask(string memory root, string memory batchDataPointer) public {
        vm.assume(bytes(batchDataPointer).length > 50);
        bytes32 batchMerkleRoot = keccak256(abi.encodePacked(root));
        // string memory batchDataPointer = "ipfs://batch1";

        vm.expectEmit(true, true, true, true);
        emit NewBatch(batchMerkleRoot, uint32(block.number), batchDataPointer);

        alignedLayerServiceManager.createNewTask{value: 0}(batchMerkleRoot, batchDataPointer);

        (uint32 taskCreatedBlock, bool responded) = alignedLayerServiceManager.batchesState(batchMerkleRoot);

        assertEq(taskCreatedBlock, uint32(block.number));
        assertEq(responded, false);
    }
}
