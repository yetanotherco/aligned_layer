// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import "eigenlayer-contracts/src/contracts/core/AVSDirectory.sol";
import {RegistryCoordinator} from "eigenlayer-middleware/RegistryCoordinator.sol";
import {IndexRegistry} from "eigenlayer-middleware/IndexRegistry.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "forge-std/StdJson.sol";

contract IndexRegistryUpgrader is Script {
    function run(
        string memory eigenLayerDeploymentFilePath,
        string memory alignedLayerDeploymentFilePath
    ) external returns (address, address) {
        // Load files
        string memory eigen_deployment_file = vm.readFile(
            eigenLayerDeploymentFilePath
        );
        string memory aligned_deployment_file = vm.readFile(
            alignedLayerDeploymentFilePath
        );
        
        // Load proxy admin
        ProxyAdmin alignedLayerProxyAdmin = ProxyAdmin(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.alignedLayerProxyAdmin"
            )
        );

        // Load IndexRegistry Proxy
        TransparentUpgradeableProxy indexRegistry = TransparentUpgradeableProxy(
            payable(stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.indexRegistry"
            ))
        );

        // Load IndexRegistry dependencies
        RegistryCoordinator registryCoordinator = RegistryCoordinator(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.registryCoordinator"
            )
        );

        // Create a new instance of the IndexRegistryImplementation
        vm.startBroadcast();
        IndexRegistry indexRegistryImplementation = new IndexRegistry(
            registryCoordinator
        );
        vm.stopBroadcast();

        // Not link the new implementation to the proxy
        // Because this must be executed in the multisig
        
        return (
            address(indexRegistry),
            address(indexRegistryImplementation)
        );
    }
}
