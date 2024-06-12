// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import {Script} from "forge-std/Script.sol";
import "eigenlayer-contracts/src/contracts/core/AVSDirectory.sol";
import {RegistryCoordinator} from "eigenlayer-middleware/RegistryCoordinator.sol";
import {StakeRegistry} from "eigenlayer-middleware/StakeRegistry.sol";
import {AlignedLayerServiceManager} from "src/core/AlignedLayerServiceManager.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "forge-std/StdJson.sol";

contract AlignedLayerUpgrader is Script {
    function run(
        string memory eigenLayerDeploymentFilePath,
        string memory alignedLayerDeploymentFilePath
    ) external returns (address, address) {
        string memory eigen_deployment_file = vm.readFile(
            eigenLayerDeploymentFilePath
        );

        string memory aligned_deployment_file = vm.readFile(
            alignedLayerDeploymentFilePath
        );

        ProxyAdmin alignedLayerProxyAdmin = ProxyAdmin(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.alignedLayerProxyAdmin"
            )
        );

        RegistryCoordinator registryCoordinator = RegistryCoordinator(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.registryCoordinator"
            )
        );

        AVSDirectory avsDirectory = AVSDirectory(
            stdJson.readAddress(
                eigen_deployment_file,
                ".addresses.avsDirectory"
            )
        );

        StakeRegistry stakeRegistry = StakeRegistry(
            stdJson.readAddress(
                aligned_deployment_file,
                ".addresses.stakeRegistry"
            )
        );

        vm.startBroadcast();

        AlignedLayerServiceManager alignedLayerServiceManagerImplementation = new AlignedLayerServiceManager(
                avsDirectory,
                registryCoordinator,
                stakeRegistry
            );

        vm.stopBroadcast();

        // alignedLayerServiceManager is the proxy
        AlignedLayerServiceManager alignedLayerServiceManager = AlignedLayerServiceManager(
                stdJson.readAddress(
                    aligned_deployment_file,
                    ".addresses.alignedLayerServiceManager"
                )
            );

        vm.startBroadcast();

        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(
                payable(address(alignedLayerServiceManager))
            ),
            address(alignedLayerServiceManagerImplementation)
        );

        vm.stopBroadcast();

        return (
            address(alignedLayerServiceManager),
            address(alignedLayerServiceManagerImplementation)
        );
    }
}
