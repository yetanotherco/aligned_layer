// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {AlignedProofAggregationService} from "../../src/core/AlignedProofAggregationService.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract AlignedProofAggregationServiceUpgrader is Script {
    function run(string memory alignedLayerDeploymentFilePath) external returns (address, address) {
        string memory aligned_deployment_file = vm.readFile(alignedLayerDeploymentFilePath);

        vm.startBroadcast();

        AlignedProofAggregationService proofAggregationServiceProxy = AlignedProofAggregationService(
            payable(stdJson.readAddress(aligned_deployment_file, ".addresses.alignedProofAggregationService"))
        );

        AlignedProofAggregationService newProofAggregatorServiceImplementation = new AlignedProofAggregationService();

        // Not link the new implementation to the proxy
        // Because this must be executed in the multisig

        vm.stopBroadcast();

        return (address(proofAggregationServiceProxy), address(newProofAggregatorServiceImplementation));
    }
}
