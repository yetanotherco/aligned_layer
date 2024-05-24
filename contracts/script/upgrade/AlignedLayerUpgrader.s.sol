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
    function run() external {
        string memory eigen_deployment_file = vm.readFile(
            "./script/output/devnet/eigenlayer_deployment_output.json"
        );

        string memory aligned_deployment_file = vm.readFile(
            "./script/output/devnet/alignedlayer_deployment_output.json"
        );

        address avsDirectoryAddress = stdJson.readAddress(
            eigen_deployment_file,
            ".addresses.avsDirectory"
        );

        address registryCoordinatorAddress = stdJson.readAddress(
            aligned_deployment_file,
            ".addresses.registryCoordinator"
        );

        address stakeRegistryAddress = stdJson.readAddress(
            aligned_deployment_file,
            ".addresses.stakeRegistry"
        );

        address alignedLayerProxyAdminAddress = stdJson.readAddress(
            aligned_deployment_file,
            ".addresses.alignedLayerProxyAdmin"
        );

        address alignedLayerServiceManagerAddress = stdJson.readAddress(
            aligned_deployment_file,
            ".addresses.alignedLayerServiceManager"
        );

        address deployer = stdJson.readAddress(
            aligned_deployment_file,
            ".permissions.alignedLayerOwner"
        );

        address aggregator = stdJson.readAddress(
            aligned_deployment_file,
            ".permissions.alignedLayerAggregator"
        );

        require(
            avsDirectoryAddress != address(0),
            "AVS directory address not found"
        );

        require(
            registryCoordinatorAddress != address(0),
            "Registry coordinator address not found"
        );

        require(
            stakeRegistryAddress != address(0),
            "Stake registry address not found"
        );

        require(
            alignedLayerProxyAdminAddress != address(0),
            "Aligned layer proxy admin address not found"
        );

        require(
            alignedLayerServiceManagerAddress != address(0),
            "Aligned layer service manager address not found"
        );

        require(deployer != address(0), "Deployer address not found");

        require(aggregator != address(0), "Aggregator address not found");

        ProxyAdmin alignedLayerProxyAdmin = ProxyAdmin(
            alignedLayerProxyAdminAddress
        );

        RegistryCoordinator registryCoordinator = RegistryCoordinator(
            address(
                new TransparentUpgradeableProxy(
                    address(registryCoordinatorAddress),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );

        AVSDirectory avsDirectory = AVSDirectory(
            address(
                new TransparentUpgradeableProxy(
                    address(avsDirectoryAddress),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );

        StakeRegistry stakeRegistry = StakeRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(stakeRegistryAddress),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );

        AlignedLayerServiceManager alignedLayerServiceManagerImplementation = new AlignedLayerServiceManager(
                avsDirectory,
                registryCoordinator,
                stakeRegistry
            );

        alignedLayerServiceManagerImplementation.initialize(
            deployer,
            aggregator
        );

        // alignedLayerServiceManager is the proxy
        AlignedLayerServiceManager alignedLayerServiceManager = AlignedLayerServiceManager(
                alignedLayerServiceManagerAddress
            );

        alignedLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(alignedLayerServiceManager))
            ),
            address(alignedLayerServiceManagerImplementation),
            abi.encodeWithSelector(
                AlignedLayerServiceManager.initialize.selector,
                deployer,
                aggregator
            )
        );
    }
}
