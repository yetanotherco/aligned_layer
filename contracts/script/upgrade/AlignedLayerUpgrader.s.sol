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
    function run() external returns (address, address) {
        string memory eigen_deployment_file = vm.readFile(
            "./script/output/devnet/eigenlayer_deployment_output.json"
        );

        string memory aligned_deployment_file = vm.readFile(
            "./script/output/devnet/alignedlayer_deployment_output.json"
        );

        uint256 alignedLayerUpgraderPrivateKey = vm.envUint(
            "ALIGNED_LAYER_UPGRADER_PRIVATE_KEY"
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

        vm.startBroadcast(alignedLayerUpgraderPrivateKey);

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

        require(
            alignedLayerProxyAdmin.getProxyAdmin(
                TransparentUpgradeableProxy(
                    payable(address(alignedLayerServiceManager))
                )
            ) == address(alignedLayerProxyAdmin),
            "AlignedLayerServiceManager is not owned by AlignedLayerProxyAdmin"
        );

        vm.startBroadcast(alignedLayerUpgraderPrivateKey);

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
