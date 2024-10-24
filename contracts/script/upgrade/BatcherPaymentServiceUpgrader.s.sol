// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;
import {BatcherPaymentService} from "../../src/core/BatcherPaymentService.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract BatcherPaymentServiceUpgrader is Script {
    function run(
        string memory alignedLayerDeploymentFilePath
    ) external returns (address, address) {

        string memory aligned_deployment_file = vm.readFile(
            alignedLayerDeploymentFilePath
        );

        vm.startBroadcast();

        BatcherPaymentService BatcherPaymentServiceProxy = BatcherPaymentService(payable(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.batcherPaymentService"
            ))
        );

        BatcherPaymentService newBatcherPaymentServiceImplementation = new BatcherPaymentService();

        // Not link the new implementation to the proxy
        // Because this must be executed in the multisig
        
        vm.stopBroadcast();

        return (address(BatcherPaymentServiceProxy), address(newBatcherPaymentServiceImplementation));
    }
}
