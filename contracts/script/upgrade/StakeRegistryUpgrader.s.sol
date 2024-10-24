// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script} from "forge-std/Script.sol";
import "eigenlayer-contracts/src/contracts/core/AVSDirectory.sol";
import {RegistryCoordinator} from "eigenlayer-middleware/RegistryCoordinator.sol";
import {StakeRegistry} from "eigenlayer-middleware/StakeRegistry.sol";
import {BLSApkRegistry} from "eigenlayer-middleware/BLSApkRegistry.sol";
import {IndexRegistry} from "eigenlayer-middleware/IndexRegistry.sol";
import {AlignedLayerServiceManager} from "src/core/AlignedLayerServiceManager.sol";
import {IServiceManager} from "eigenlayer-middleware/interfaces/IServiceManager.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "forge-std/StdJson.sol";

contract StakeRegistryUpgrader is Script {
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

        // Load RegistryCoordinator Proxy
        TransparentUpgradeableProxy stakeRegistry = TransparentUpgradeableProxy(
            payable(stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.stakeRegistry"
            ))
        );

        // Load RegistryCoordinator dependencies
        RegistryCoordinator registryCoordinator = RegistryCoordinator(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.registryCoordinator"
            )
        );
        IDelegationManager delegationManager = IDelegationManager(
            stdJson.readAddress(
                eigen_deployment_file,
                ".addresses.delegationManager"
            )
        );

        // Create a new instance of the RegistryCoordinatorImplementation
        vm.startBroadcast();
        StakeRegistry stakeRegistryImplementation = new StakeRegistry(
            registryCoordinator,
            delegationManager
        );
        vm.stopBroadcast();

        // Not link the new implementation to the proxy
        // Because this must be executed in the multisig
        
        return (
            address(stakeRegistry),
            address(stakeRegistryImplementation)
        );
    }
}
