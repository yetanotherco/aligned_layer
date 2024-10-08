// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {stdStorage, StdStorage, Test} from "forge-std/Test.sol";
import "../src/core/AlignedLayerServiceManager.sol";
import {IAlignedLayerServiceManager} from "../src/core/IAlignedLayerServiceManager.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IRewardsCoordinator} from "eigenlayer-contracts/src/contracts/interfaces/IRewardsCoordinator.sol";
import {BLSMockAVSDeployer} from "eigenlayer-middleware/../test/utils/BLSMockAVSDeployer.sol";

contract AlignedLayerServiceManagerTest is Test, BLSMockAVSDeployer {
    AlignedLayerServiceManager alignedLayerServiceManager;
    address initialOwner = address(0x123);
    address aggregator = address(0x456);

    using stdStorage for StdStorage;

    event NewBatchV2(
        bytes32 indexed batchMerkleRoot,
        address senderAddress,
        uint32 taskCreatedBlock,
        string batchDataPointer
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
            IRewardsCoordinator(address(rewardsCoordinatorMock)),
            IRegistryCoordinator(address(registryCoordinator)),
            IStakeRegistry(address(stakeRegistry))
        );

        // alignedLayerServiceManager.initialize(initialOwner, aggregator);
    }

    // function testInitialize() public {
    //     assertEq(alignedLayerServiceManager.owner(), initialOwner);
    //     assertEq(alignedLayerServiceManager.isAggregator(aggregator), true);
    // }

    function testCreateNewTask(
        string memory root,
        string memory batchDataPointer,
        uint256 maxFeeToRespond
    ) public {
        vm.assume(bytes(batchDataPointer).length > 50);
        bytes32 batchMerkleRoot = keccak256(abi.encodePacked(root));

        address batcher = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
        hoax(batcher, maxFeeToRespond);

        // transfer to serviceManager
        address(alignedLayerServiceManager).call{value: maxFeeToRespond}("");

        vm.expectEmit(true, true, true, true);
        emit NewBatchV3(
            batchMerkleRoot,
            batcher,
            uint32(block.number),
            batchDataPointer,
            maxFeeToRespond
        );

        vm.prank(batcher);
        alignedLayerServiceManager.createNewTask(
            batchMerkleRoot,
            batchDataPointer,
            maxFeeToRespond
        );

        bytes32 batchIdentifierHash = keccak256(
            abi.encodePacked(batchMerkleRoot, batcher)
        );

        (
            uint32 taskCreatedBlock,
            bool responded,
            uint256 _maxFeeToRespond
        ) = alignedLayerServiceManager.batchesState(batchIdentifierHash);

        assertEq(taskCreatedBlock, uint32(block.number));
        assertEq(responded, false);
        assertEq(_maxFeeToRespond, maxFeeToRespond);
    }

    /* =============== Disabled verifiers tests =============== */

    function test_SetVerifiersList_WorksAsExpected() public {
        vm.prank(address(0));
        uint256 newBitmap = 1234;
        alignedLayerServiceManager.setDisabledVerifiers(newBitmap);
        uint256 actualBitmap = alignedLayerServiceManager.disabledVerifiers();

        assertEq(newBitmap, actualBitmap);
    }

    function test_DisabledAndEnableVerifier_WorksAsExpected() public {
        uint8 verifierIdx = 28;

        // make sure it is false by default
        bool res = alignedLayerServiceManager.isVerifierDisabled(
            verifierIdx
        );
        assertEq(res, false);

        vm.expectEmit(true, true, true, true);
        emit IAlignedLayerServiceManager.VerifierDisabled(verifierIdx);
        // disable the verifier and check that it has been actually disable
        vm.prank(address(0));
        alignedLayerServiceManager.disableVerifier(verifierIdx);
        res = alignedLayerServiceManager.isVerifierDisabled(verifierIdx);
        assertEq(res, true);

        // now whitelist the verifier again and make sure is not disabled anymore
        vm.expectEmit(true, true, true, true);
        emit IAlignedLayerServiceManager.VerifierEnabled(verifierIdx);
        vm.prank(address(0));
        alignedLayerServiceManager.enableVerifier(verifierIdx);
        res = alignedLayerServiceManager.isVerifierDisabled(verifierIdx);
        assertEq(res, false);
    }

    // here we test the filures

    // test ownership
    function test_SetVerifiersDisabled_FailsWhenNotOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        alignedLayerServiceManager.setDisabledVerifiers(213);
    }

    function test_DisableVerifier_FailsWhenNotOwner() public {
        uint8 newBitmap = 10;
        vm.expectRevert("Ownable: caller is not the owner");
        alignedLayerServiceManager.disableVerifier(newBitmap);
    }

    function test_EnabledVerifier_FailsWhenNotOwner() public {
        uint8 newBitmap = 10;
        vm.expectRevert("Ownable: caller is not the owner");
        alignedLayerServiceManager.enableVerifier(newBitmap);
    }
}
