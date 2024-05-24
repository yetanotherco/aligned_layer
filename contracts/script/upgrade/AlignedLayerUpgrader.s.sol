// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import {Script} from "forge-std/Script.sol";
import "eigenlayer-contracts/src/contracts/core/AVSDirectory.sol";
import {RegistryCoordinator} from "eigenlayer-middleware/RegistryCoordinator.sol";
import {StakeRegistry} from "eigenlayer-middleware/StakeRegistry.sol";
import {AlignedLayerServiceManager} from "src/core/AlignedLayerServiceManager.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "forge-std/StdJson.sol";

contract AlignedLayerUpgrader is Script {
    function run() external returns (address, address) {
        string memory eigen_deployment_file = vm.readFile(
            "./script/output/devnet/eigenlayer_deployment_output.json"
        );

        string memory aligned_deployment_file = vm.readFile(
            "./script/output/devnet/alignedlayer_deployment_output.json"
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

        AlignedLayerServiceManager alignedLayerServiceManagerImplementation = new AlignedLayerServiceManager(
                avsDirectory,
                registryCoordinator,
                stakeRegistry
            );

        // alignedLayerServiceManager is the proxy
        AlignedLayerServiceManager alignedLayerServiceManager = AlignedLayerServiceManager(
                stdJson.readAddress(
                    aligned_deployment_file,
                    ".addresses.alignedLayerServiceManager"
                )
            );

        alignedLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(alignedLayerServiceManager))
            ),
            address(alignedLayerServiceManagerImplementation),
            abi.encodeWithSelector(
                AlignedLayerServiceManager.initialize.selector,
                stdJson.readAddress(
                    aligned_deployment_file,
                    ".permissions.alignedLayerOwner"
                ),
                stdJson.readAddress(
                    aligned_deployment_file,
                    ".permissions.alignedLayerAggregator"
                )
            )
        );

        return (
            address(registryCoordinator.stakeRegistry()),
            address(stakeRegistry)
        );
    }
}
