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

contract RegistryCoordinatorUpgrader is Script {
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
        TransparentUpgradeableProxy registryCoordinator = TransparentUpgradeableProxy(
                payable(
                    stdJson.readAddress(
                        aligned_deployment_file,
                        ".addresses.registryCoordinator"
                    )
                )
            );

        // Load RegistryCoordinator dependencies
        AlignedLayerServiceManager alignedLayerServiceManager = AlignedLayerServiceManager(
                payable(
                    stdJson.readAddress(
                        aligned_deployment_file,
                        ".addresses.alignedLayerServiceManager"
                    )
                )
            );
        StakeRegistry stakeRegistry = StakeRegistry(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.stakeRegistry"
            )
        );
        BLSApkRegistry apkRegistry = BLSApkRegistry(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.blsApkRegistry"
            )
        );
        IndexRegistry indexRegistry = IndexRegistry(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.indexRegistry"
            )
        );

        // Create a new instance of the RegistryCoordinatorImplementation
        vm.startBroadcast();
        RegistryCoordinator registryCoordinatorImplementation = new RegistryCoordinator(
                IServiceManager(address(alignedLayerServiceManager)),
                stakeRegistry,
                apkRegistry,
                indexRegistry
            );
        vm.stopBroadcast();

        // Not link the new implementation to the proxy
        // Because this must be executed in the multisig

        return (
            address(registryCoordinator),
            address(registryCoordinatorImplementation)
        );
    }
}
