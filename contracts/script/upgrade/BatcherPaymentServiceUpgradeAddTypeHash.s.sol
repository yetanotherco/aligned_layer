// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;
import {BatcherPaymentService} from "../../src/core/BatcherPaymentService.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract BatcherPaymentServiceUpgradeAddTypeHash is Script {
    function run(
        string memory alignedLayerDeploymentFilePath,
        string memory batcherPaymentServiceConfigFilePath
    ) external returns (address, address) {
        string memory aligned_deployment_file = vm.readFile(
            alignedLayerDeploymentFilePath
        );

        string memory batcherPaymentServiceConfigFile = vm.readFile(
            batcherPaymentServiceConfigFilePath
        );

        bytes32 noncedVerificationDataTypeHash = stdJson.readBytes32(
            batcherPaymentServiceConfigFile,
            ".eip712.noncedVerificationDataTypeHash"
        );

        vm.startBroadcast();

        BatcherPaymentService BatcherPaymentServiceProxy = BatcherPaymentService(
                payable(
                    stdJson.readAddress(
                        aligned_deployment_file,
                        ".addresses.batcherPaymentService"
                    )
                )
            );

        BatcherPaymentService newBatcherPaymentServiceImplementation = new BatcherPaymentService();
        BatcherPaymentServiceProxy.upgradeToAndCall(
            address(newBatcherPaymentServiceImplementation),
            ""
        );

        vm.stopBroadcast();

        vm.startBroadcast();

        BatcherPaymentServiceProxy.initializeNoncedVerificationDataTypeHash(
            noncedVerificationDataTypeHash
        );

        return (
            address(BatcherPaymentServiceProxy),
            address(newBatcherPaymentServiceImplementation)
        );
    }
}
