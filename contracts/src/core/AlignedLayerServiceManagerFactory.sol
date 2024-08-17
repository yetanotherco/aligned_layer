// SPDX-License-Identifier: MIT
pragma solidity =0.8.12;

import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {EmptyContract} from "eigenlayer-core/test/mocks/EmptyContract.sol";
import {AlignedLayerServiceManager} from "src/core/AlignedLayerServiceManager.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";
import {IServiceManager} from "eigenlayer-middleware/interfaces/IServiceManager.sol";
import {IPauserRegistry} from "eigenlayer-core/contracts/interfaces/IPauserRegistry.sol";
import {IBLSApkRegistry} from "eigenlayer-middleware/interfaces/IBLSApkRegistry.sol";
import {IIndexRegistry} from "eigenlayer-middleware/interfaces/IIndexRegistry.sol";
import {IAVSDirectory} from "eigenlayer-middleware/ServiceManagerBase.sol";
import {IRewardsCoordinator} from "eigenlayer-contracts/src/contracts/interfaces/IRewardsCoordinator.sol";
import {RegistryCoordinator} from "eigenlayer-middleware/RegistryCoordinator.sol";

contract AlignedLayerServiceManagerFactory {
    struct DeployParams {
        EmptyContract emptyContract;
        ProxyAdmin alignedLayerProxyAdmin;
        IStakeRegistry stakeRegistry;
        IBLSApkRegistry apkRegistry;
        IIndexRegistry indexRegistry;
        address alignedLayerOwner;
        address churner;
        address ejector;
        address pauser;
        address deployer;
        uint256 initalPausedStatus;
        IRegistryCoordinator.OperatorSetParam[] operatorSetParams;
        uint96[] minimumStakeForQuourm;
        IStakeRegistry.StrategyParams[][] strategyAndWeightingMultipliers;
        IAVSDirectory avsDirectory;
        IRewardsCoordinator rewardsCoordinator;
        IRegistryCoordinator registryCoordinator;
        string metadataURI;
    }

    AlignedLayerServiceManager public _alignedLayerServiceManager;
    RegistryCoordinator public _registryCoordinator;
    AlignedLayerServiceManager public _alignedLayerServiceManagerImplementation;
    RegistryCoordinator public _registryCoordinatorImplementation;

    function deploy(
        DeployParams memory params
    )
        public
        returns (
            address alignedLayerServiceManagerAdress,
            address alignedLayerServiceManagerImplementationAddress,
            address registryCoordinatorAddress,
            address registryCoordinatorImplementationAddress
        )
    {
        _deployAlignedLayerServiceManagerProxy(params);
        _deployAndUpgradeRegistryCoordinator(params);
        _deployAndUpgradeAlignedLayerServiceManager(params);

        _alignedLayerServiceManager.updateAVSMetadataURI(params.metadataURI);
        _alignedLayerServiceManager.transferOwnership(params.alignedLayerOwner);

        return (
            address(_alignedLayerServiceManager),
            address(_alignedLayerServiceManagerImplementation),
            address(_registryCoordinator),
            address(_registryCoordinatorImplementation)
        );
    }

    function _deployAlignedLayerServiceManagerProxy(
        DeployParams memory params
    ) internal {
        _alignedLayerServiceManager = AlignedLayerServiceManager(
            payable(
                new TransparentUpgradeableProxy(
                    address(params.emptyContract),
                    address(params.alignedLayerProxyAdmin),
                    ""
                )
            )
        );
    }

    function _deployAndUpgradeRegistryCoordinator(
        DeployParams memory params
    ) internal {
        _registryCoordinatorImplementation = new RegistryCoordinator(
            IServiceManager(address(_alignedLayerServiceManager)),
            params.stakeRegistry,
            params.apkRegistry,
            params.indexRegistry
        );

        params.alignedLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(params.registryCoordinator))
            ),
            address(_registryCoordinatorImplementation),
            abi.encodeWithSelector(
                RegistryCoordinator.initialize.selector,
                params.alignedLayerOwner,
                params.churner,
                params.ejector,
                IPauserRegistry(params.pauser),
                params.initalPausedStatus,
                params.operatorSetParams,
                params.minimumStakeForQuourm,
                params.strategyAndWeightingMultipliers
            )
        );
    }

    function _deployAndUpgradeAlignedLayerServiceManager(
        DeployParams memory params
    ) internal {
        _alignedLayerServiceManagerImplementation = new AlignedLayerServiceManager(
            params.avsDirectory,
            params.rewardsCoordinator,
            params.registryCoordinator,
            params.stakeRegistry
        );

        params.alignedLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(_alignedLayerServiceManager))
            ),
            address(_alignedLayerServiceManagerImplementation),
            abi.encodeWithSelector(
                AlignedLayerServiceManager.initialize.selector,
                params.deployer
            )
        );
    }
}
