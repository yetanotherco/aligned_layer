// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import "forge-std/Test.sol";
import {stdStorage, StdStorage} from "forge-std/Test.sol"; 
import "../src/core/AlignedLayerServiceManager.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {BLSMockAVSDeployer} from "eigenlayer-middleware/../test/utils/BLSMockAVSDeployer.sol";

contract AlignedLayerServiceManagerTest is BLSMockAVSDeployer {
    AlignedLayerServiceManager alignedLayerServiceManager;
    address initialOwner = address(0x123);
    address aggregator = address(0x456);

    using stdStorage for StdStorage;

    event NewBatch(
        bytes32 batchMerkleRoot,
        uint32 taskCreatedBlock,
        string batchDataPointer
    );

    event BatchVerified(bytes32 batchMerkleRoot);

    function setUp() virtual public {
        _setUpBLSMockAVSDeployer();

        alignedLayerServiceManager = new AlignedLayerServiceManager(
            avsDirectory,
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

    // TODO fix this test in another PR
    // This test is based on https://github.com/Layr-Labs/eigenda/blob/master/contracts/test/unit/EigenDAServiceManagerUnit.t.sol#L63
    // function testRespondToTask() public {
    //     bytes32 batchMerkleRoot = keccak256(abi.encodePacked("batch1"));
    //     string memory batchDataPointer = "ipfs://batch1";
    //     // IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature;
    //     uint256 nonRandomNumber = 111;
    //     uint256 numNonSigners = 1;
    //     uint256 quorumBitmap = 1;
    //     // bytes memory quorumNumbers = BitmapUtils.bitmapToBytesArray(quorumBitmap);

    //     (uint32 referenceBlockNumber, BLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) = 
    //         _registerSignatoriesAndGetNonSignerStakeAndSignatureRandom(nonRandomNumber, numNonSigners, quorumBitmap);

    //     // Create a new task first TODO use stdstore instead of contract method
    //     alignedLayerServiceManager.createNewTask{value: 0}(batchMerkleRoot, batchDataPointer);
    //     // stdstore
    //     //     .target(address(alignedLayerServiceManager))
    //     //     .sig(alignedLayerServiceManager.batchesState.selector)
    //     //     .with_key(batchMerkleRoot)
    //     //     .depth(0)
    //     //     .checked_write(uint32(block.number));
    //     // stdstore
    //     //     .target(address(alignedLayerServiceManager))
    //     //     .sig(alignedLayerServiceManager.batchesState.selector)
    //     //     .with_key(batchMerkleRoot)
    //     //     .depth(1)
    //     //     .checked_write(false);

    //     // vm.expectEmit(true, true, true, true);
    //     // emit BatchVerified(batchMerkleRoot);

    //     alignedLayerServiceManager.respondToTask(batchMerkleRoot, nonSignerStakesAndSignature);

    //     (uint32 taskCreatedBlock, bool responded) = alignedLayerServiceManager.batchesState(batchMerkleRoot);

    //     assertEq(taskCreatedBlock, uint32(block.number));
    //     assertEq(responded, true);
    // }
}
