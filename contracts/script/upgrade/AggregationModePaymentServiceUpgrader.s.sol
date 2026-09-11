// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {AggregationModePaymentService} from "../../src/core/AggregationModePaymentService.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract AggregationModePaymentServiceUpgrader is Script {
    function run(
        string memory alignedLayerDeploymentFilePath
    ) external returns (address, address) {
        string memory aligned_deployment_file = vm.readFile(
            alignedLayerDeploymentFilePath
        );

        vm.startBroadcast();

        AggregationModePaymentService aggregationModePaymentServiceProxy =
            AggregationModePaymentService(payable(
                stdJson.readAddress(
                    aligned_deployment_file,
                    ".addresses.aggregationModePaymentService"
                )
            ));

        AggregationModePaymentService newAggregationModePaymentServiceImplementation =
            new AggregationModePaymentService();

        // Not link the new implementation to the proxy
        // Because this must be executed in the multisig

        vm.stopBroadcast();

        return (
            address(aggregationModePaymentServiceProxy),
            address(newAggregationModePaymentServiceImplementation)
        );
    }
}
