// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.12;

/*
    This script is a modified version of the Mainnet_Deploy.s.sol script used by EigenDA:
    https://github.com/Layr-Labs/eigenda/blob/v0.6.1/contracts/script/deploy/mainnet/Mainnet_Deploy.s.sol
*/

import {PauserRegistry} from "eigenlayer-core/contracts/permissions/PauserRegistry.sol";
import {EmptyContract} from "eigenlayer-core/test/mocks/EmptyContract.sol";

import {BLSApkRegistry} from "eigenlayer-middleware/BLSApkRegistry.sol";
import {IBLSApkRegistry} from "eigenlayer-middleware/interfaces/IBLSApkRegistry.sol";
import {RegistryCoordinator} from "eigenlayer-middleware/RegistryCoordinator.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IndexRegistry} from "eigenlayer-middleware/IndexRegistry.sol";
import {IIndexRegistry} from "eigenlayer-middleware/interfaces/IIndexRegistry.sol";
import {StakeRegistry} from "eigenlayer-middleware/StakeRegistry.sol";
import {IStakeRegistry} from "eigenlayer-middleware/interfaces/IStakeRegistry.sol";
import {AlignedLayerServiceManager} from "src/core/AlignedLayerServiceManager.sol";
import {IServiceManager} from "eigenlayer-middleware/interfaces/IServiceManager.sol";
import {OperatorStateRetriever} from "eigenlayer-middleware/OperatorStateRetriever.sol";
import {ServiceManagerRouter} from "eigenlayer-middleware/ServiceManagerRouter.sol";

import "script/deploy/utils/ExistingDeploymentParser.sol";
import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract AlignedLayerDeployer is ExistingDeploymentParser {
    ProxyAdmin public alignedLayerProxyAdmin;
    address public alignedLayerOwner;
    address public alignedLayerUpgrader;
    address public pauser;
    uint256 public initalPausedStatus;
    address public deployer;
    address public alignedLayerAggregator;

    BLSApkRegistry public apkRegistry;
    AlignedLayerServiceManager public alignedLayerServiceManager;
    RegistryCoordinator public registryCoordinator;
    IndexRegistry public indexRegistry;
    StakeRegistry public stakeRegistry;
    OperatorStateRetriever public operatorStateRetriever;
    ServiceManagerRouter public serviceManagerRouter;

    BLSApkRegistry public apkRegistryImplementation;
    AlignedLayerServiceManager public alignedLayerServiceManagerImplementation;
    RegistryCoordinator public registryCoordinatorImplementation;
    IndexRegistry public indexRegistryImplementation;
    StakeRegistry public stakeRegistryImplementation;

    function run(
        string memory existingDeploymentInfoPath,
        string memory deployConfigPath,
        string memory outputPath
    ) external {
        // get info on all the already-deployed contracts
        _parseDeployedContracts(existingDeploymentInfoPath);

        // READ JSON CONFIG DATA
        string memory config_data = vm.readFile(deployConfigPath);

        // check that the chainID matches the one in the config
        uint256 currentChainId = block.chainid;
        uint256 configChainId = stdJson.readUint(
            config_data,
            ".chainInfo.chainId"
        );
        emit log_named_uint("You are deploying on ChainID", currentChainId);
        require(
            configChainId == currentChainId,
            "You are on the wrong chain for this config"
        );

        // parse the addresses of permissioned roles
        alignedLayerOwner = stdJson.readAddress(
            config_data,
            ".permissions.owner"
        );
        alignedLayerUpgrader = stdJson.readAddress(
            config_data,
            ".permissions.upgrader"
        );
        initalPausedStatus = stdJson.readUint(
            config_data,
            ".permissions.initalPausedStatus"
        );

        pauser = address(eigenLayerPauserReg);

        deployer = stdJson.readAddress(config_data, ".permissions.deployer");
        require(
            deployer == tx.origin,
            "Deployer address must be the same as the tx.origin"
        );
        emit log_named_address("You are deploying from", deployer);

        alignedLayerAggregator = stdJson.readAddress(
            config_data,
            ".permissions.aggregator"
        );

        vm.startBroadcast();

        // deploy proxy admin for ability to upgrade proxy contracts
        alignedLayerProxyAdmin = new ProxyAdmin();

        //deploy service manager router
        serviceManagerRouter = new ServiceManagerRouter();

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        alignedLayerServiceManager = AlignedLayerServiceManager(
            payable(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        registryCoordinator = RegistryCoordinator(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        indexRegistry = IndexRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        stakeRegistry = StakeRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        apkRegistry = BLSApkRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );

        //deploy index registry implementation
        indexRegistryImplementation = new IndexRegistry(registryCoordinator);

        //upgrade index registry proxy to implementation
        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(indexRegistry))),
            address(indexRegistryImplementation)
        );

        //deploy stake registry implementation
        stakeRegistryImplementation = new StakeRegistry(
            registryCoordinator,
            delegationManager
        );

        //upgrade stake registry proxy to implementation
        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(stakeRegistry))),
            address(stakeRegistryImplementation)
        );

        //deploy apk registry implementation
        apkRegistryImplementation = new BLSApkRegistry(registryCoordinator);

        //upgrade apk registry proxy to implementation
        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(apkRegistry))),
            address(apkRegistryImplementation)
        );

        //deploy the registry coordinator implementation.
        registryCoordinatorImplementation = new RegistryCoordinator(
            IServiceManager(address(alignedLayerServiceManager)),
            stakeRegistry,
            apkRegistry,
            indexRegistry
        );

        {
            // parse initalization params and permissions from config data
            (
                uint96[] memory minimumStakeForQuourm,
                IStakeRegistry.StrategyParams[][]
                    memory strategyAndWeightingMultipliers
            ) = _parseStakeRegistryParams(config_data);
            (
                IRegistryCoordinator.OperatorSetParam[]
                    memory operatorSetParams,
                address churner,
                address ejector
            ) = _parseRegistryCoordinatorParams(config_data);

            //upgrade the registry coordinator proxy to implementation
            alignedLayerProxyAdmin.upgradeAndCall(
                TransparentUpgradeableProxy(
                    payable(address(registryCoordinator))
                ),
                address(registryCoordinatorImplementation),
                abi.encodeWithSelector(
                    RegistryCoordinator.initialize.selector,
                    alignedLayerOwner,
                    churner,
                    ejector,
                    IPauserRegistry(pauser),
                    initalPausedStatus,
                    operatorSetParams,
                    minimumStakeForQuourm,
                    strategyAndWeightingMultipliers
                )
            );
        }

        //deploy the alignedLayer service manager implementation
        alignedLayerServiceManagerImplementation = new AlignedLayerServiceManager(
            avsDirectory,
            rewardsCoordinator,
            registryCoordinator,
            stakeRegistry
        );

        //upgrade the alignedLayer service manager proxy to implementation
        alignedLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(alignedLayerServiceManager))
            ),
            address(alignedLayerServiceManagerImplementation),
            abi.encodeWithSelector(
                AlignedLayerServiceManager.initialize.selector,
                deployer,
                deployer,
                alignedLayerAggregator
            )
        );

        string memory metadataURI = stdJson.readString(config_data, ".uri");
        alignedLayerServiceManager.updateAVSMetadataURI(metadataURI);
        alignedLayerServiceManager.transferOwnership(alignedLayerOwner);

        //deploy the operator state retriever
        operatorStateRetriever = new OperatorStateRetriever();

        // transfer ownership of proxy admin to upgrader
        alignedLayerProxyAdmin.transferOwnership(alignedLayerUpgrader);

        vm.stopBroadcast();

        // sanity checks
        __verifyContractPointers(
            apkRegistry,
            alignedLayerServiceManager,
            registryCoordinator,
            indexRegistry,
            stakeRegistry
        );

        __verifyContractPointers(
            apkRegistryImplementation,
            alignedLayerServiceManagerImplementation,
            registryCoordinatorImplementation,
            indexRegistryImplementation,
            stakeRegistryImplementation
        );

        __verifyImplementations();
        __verifyInitalizations(config_data);

        //write output
        _writeOutput(config_data, outputPath);
    }

    function xtest(
        string memory existingDeploymentInfoPath,
        string memory deployConfigPath
    ) external {
        // get info on all the already-deployed contracts
        _parseDeployedContracts(existingDeploymentInfoPath);

        // READ JSON CONFIG DATA
        string memory config_data = vm.readFile(deployConfigPath);

        // check that the chainID matches the one in the config
        uint256 currentChainId = block.chainid;
        uint256 configChainId = stdJson.readUint(
            config_data,
            ".chainInfo.chainId"
        );
        emit log_named_uint("You are deploying on ChainID", currentChainId);
        require(
            configChainId == currentChainId,
            "You are on the wrong chain for this config"
        );

        // parse the addresses of permissioned roles
        alignedLayerOwner = stdJson.readAddress(
            config_data,
            ".permissions.owner"
        );
        alignedLayerUpgrader = stdJson.readAddress(
            config_data,
            ".permissions.upgrader"
        );
        initalPausedStatus = stdJson.readUint(
            config_data,
            ".permissions.initalPausedStatus"
        );

        pauser = address(eigenLayerPauserReg);

        deployer = stdJson.readAddress(config_data, ".permissions.deployer");
        vm.startPrank(deployer);

        // deploy proxy admin for ability to upgrade proxy contracts
        alignedLayerProxyAdmin = new ProxyAdmin();

        //deploy service manager router
        serviceManagerRouter = new ServiceManagerRouter();

        /**
         * First, deploy upgradeable proxy contracts that **will point** to the implementations. Since the implementation contracts are
         * not yet deployed, we give these proxies an empty contract as the initial implementation, to act as if they have no code.
         */
        alignedLayerServiceManager = AlignedLayerServiceManager(
            payable(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        registryCoordinator = RegistryCoordinator(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        indexRegistry = IndexRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        stakeRegistry = StakeRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );
        apkRegistry = BLSApkRegistry(
            address(
                new TransparentUpgradeableProxy(
                    address(emptyContract),
                    address(alignedLayerProxyAdmin),
                    ""
                )
            )
        );

        //deploy index registry implementation
        indexRegistryImplementation = new IndexRegistry(registryCoordinator);

        //upgrade index registry proxy to implementation
        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(indexRegistry))),
            address(indexRegistryImplementation)
        );

        //deploy stake registry implementation
        stakeRegistryImplementation = new StakeRegistry(
            registryCoordinator,
            delegationManager
        );

        //upgrade stake registry proxy to implementation
        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(stakeRegistry))),
            address(stakeRegistryImplementation)
        );

        //deploy apk registry implementation
        apkRegistryImplementation = new BLSApkRegistry(registryCoordinator);

        //upgrade apk registry proxy to implementation
        alignedLayerProxyAdmin.upgrade(
            TransparentUpgradeableProxy(payable(address(apkRegistry))),
            address(apkRegistryImplementation)
        );

        //deploy the registry coordinator implementation.
        registryCoordinatorImplementation = new RegistryCoordinator(
            IServiceManager(address(alignedLayerServiceManager)),
            stakeRegistry,
            apkRegistry,
            indexRegistry
        );

        {
            // parse initalization params and permissions from config data
            (
                uint96[] memory minimumStakeForQuourm,
                IStakeRegistry.StrategyParams[][]
                    memory strategyAndWeightingMultipliers
            ) = _parseStakeRegistryParams(config_data);
            (
                IRegistryCoordinator.OperatorSetParam[]
                    memory operatorSetParams,
                address churner,
                address ejector
            ) = _parseRegistryCoordinatorParams(config_data);

            //upgrade the registry coordinator proxy to implementation
            alignedLayerProxyAdmin.upgradeAndCall(
                TransparentUpgradeableProxy(
                    payable(address(registryCoordinator))
                ),
                address(registryCoordinatorImplementation),
                abi.encodeWithSelector(
                    RegistryCoordinator.initialize.selector,
                    alignedLayerOwner,
                    churner,
                    ejector,
                    IPauserRegistry(pauser),
                    initalPausedStatus,
                    operatorSetParams,
                    minimumStakeForQuourm,
                    strategyAndWeightingMultipliers
                )
            );
        }

        //deploy the alignedLayer service manager implementation
        alignedLayerServiceManagerImplementation = new AlignedLayerServiceManager(
            avsDirectory,
            rewardsCoordinator,
            registryCoordinator,
            stakeRegistry
        );

        //upgrade the alignedLayer service manager proxy to implementation
        alignedLayerProxyAdmin.upgradeAndCall(
            TransparentUpgradeableProxy(
                payable(address(alignedLayerServiceManager))
            ),
            address(alignedLayerServiceManagerImplementation),
            abi.encodeWithSelector(
                AlignedLayerServiceManager.initialize.selector,
                deployer,
                deployer
            )
        );

        string memory metadataURI = stdJson.readString(config_data, ".uri");
        alignedLayerServiceManager.updateAVSMetadataURI(metadataURI);
        alignedLayerServiceManager.transferOwnership(alignedLayerOwner);

        //deploy the operator state retriever
        operatorStateRetriever = new OperatorStateRetriever();

        // transfer ownership of proxy admin to upgrader
        alignedLayerProxyAdmin.transferOwnership(alignedLayerUpgrader);

        vm.stopPrank();

        // sanity checks
        __verifyContractPointers(
            apkRegistry,
            alignedLayerServiceManager,
            registryCoordinator,
            indexRegistry,
            stakeRegistry
        );

        __verifyContractPointers(
            apkRegistryImplementation,
            alignedLayerServiceManagerImplementation,
            registryCoordinatorImplementation,
            indexRegistryImplementation,
            stakeRegistryImplementation
        );

        __verifyImplementations();
        __verifyInitalizations(config_data);
    }

    function __verifyContractPointers(
        BLSApkRegistry _apkRegistry,
        AlignedLayerServiceManager _alignedLayerServiceManager,
        RegistryCoordinator _registryCoordinator,
        IndexRegistry _indexRegistry,
        StakeRegistry _stakeRegistry
    ) internal view {
        require(
            address(_apkRegistry.registryCoordinator()) ==
                address(registryCoordinator),
            "blsApkRegistry.registryCoordinator() != registryCoordinator"
        );

        require(
            address(_indexRegistry.registryCoordinator()) ==
                address(registryCoordinator),
            "indexRegistry.registryCoordinator() != registryCoordinator"
        );

        require(
            address(_stakeRegistry.registryCoordinator()) ==
                address(registryCoordinator),
            "stakeRegistry.registryCoordinator() != registryCoordinator"
        );
        require(
            address(_stakeRegistry.delegation()) == address(delegationManager),
            "stakeRegistry.delegationManager() != delegation"
        );

        require(
            address(_alignedLayerServiceManager.registryCoordinator()) ==
                address(registryCoordinator),
            "alignedLayerServiceManager.registryCoordinator() != registryCoordinator"
        );
        require(
            address(_alignedLayerServiceManager.stakeRegistry()) ==
                address(stakeRegistry),
            "alignedLayerServiceManager.stakeRegistry() != stakeRegistry"
        );
        require(
            address(_alignedLayerServiceManager.avsDirectory()) ==
                address(avsDirectory),
            "alignedLayerServiceManager.avsDirectory() != avsDirectory"
        );

        require(
            address(_registryCoordinator.serviceManager()) ==
                address(alignedLayerServiceManager),
            "registryCoordinator.alignedLayerServiceManager() != alignedLayerServiceManager"
        );
        require(
            address(_registryCoordinator.stakeRegistry()) ==
                address(stakeRegistry),
            "registryCoordinator.stakeRegistry() != stakeRegistry"
        );
        require(
            address(_registryCoordinator.blsApkRegistry()) ==
                address(apkRegistry),
            "registryCoordinator.blsApkRegistry() != blsPubkeyRegistry"
        );
        require(
            address(_registryCoordinator.indexRegistry()) ==
                address(indexRegistry),
            "registryCoordinator.indexRegistry() != indexRegistry"
        );
    }

    function __verifyImplementations() internal view {
        require(
            alignedLayerProxyAdmin.getProxyImplementation(
                TransparentUpgradeableProxy(
                    payable(address(alignedLayerServiceManager))
                )
            ) == address(alignedLayerServiceManagerImplementation),
            "alignedLayerServiceManager: implementation set incorrectly"
        );
        require(
            alignedLayerProxyAdmin.getProxyImplementation(
                TransparentUpgradeableProxy(
                    payable(address(registryCoordinator))
                )
            ) == address(registryCoordinatorImplementation),
            "registryCoordinator: implementation set incorrectly"
        );
        require(
            alignedLayerProxyAdmin.getProxyImplementation(
                TransparentUpgradeableProxy(payable(address(apkRegistry)))
            ) == address(apkRegistryImplementation),
            "blsApkRegistry: implementation set incorrectly"
        );
        require(
            alignedLayerProxyAdmin.getProxyImplementation(
                TransparentUpgradeableProxy(payable(address(indexRegistry)))
            ) == address(indexRegistryImplementation),
            "indexRegistry: implementation set incorrectly"
        );
        require(
            alignedLayerProxyAdmin.getProxyImplementation(
                TransparentUpgradeableProxy(payable(address(stakeRegistry)))
            ) == address(stakeRegistryImplementation),
            "stakeRegistry: implementation set incorrectly"
        );
    }

    function __verifyInitalizations(string memory config_data) internal {
        (
            uint96[] memory minimumStakeForQuourm,
            IStakeRegistry.StrategyParams[][]
                memory strategyAndWeightingMultipliers
        ) = _parseStakeRegistryParams(config_data);
        (
            IRegistryCoordinator.OperatorSetParam[] memory operatorSetParams,
            address churner,
            address ejector
        ) = _parseRegistryCoordinatorParams(config_data);

        require(
            alignedLayerServiceManager.owner() == alignedLayerOwner,
            "alignedLayerServiceManager.owner() != alignedLayerOwner"
        );
        // require(alignedLayerServiceManager.pauserRegistry() == IPauserRegistry(pauser), "alignedLayerServiceManager: pauser registry not set correctly");
        // require(alignedLayerServiceManager.paused() == initalPausedStatus, "alignedLayerServiceManager: init paused status set incorrectly");

        require(
            registryCoordinator.owner() == alignedLayerOwner,
            "registryCoordinator.owner() != alignedLayerOwner"
        );
        require(
            registryCoordinator.churnApprover() == churner,
            "registryCoordinator.churner() != churner"
        );
        require(
            registryCoordinator.ejector() == ejector,
            "registryCoordinator.ejector() != ejector"
        );
        require(
            registryCoordinator.pauserRegistry() == IPauserRegistry(pauser),
            "registryCoordinator: pauser registry not set correctly"
        );
        require(
            registryCoordinator.paused() == initalPausedStatus,
            "registryCoordinator: init paused status set incorrectly"
        );

        for (uint8 i = 0; i < operatorSetParams.length; ++i) {
            require(
                keccak256(
                    abi.encode(registryCoordinator.getOperatorSetParams(i))
                ) == keccak256(abi.encode(operatorSetParams[i])),
                "registryCoordinator.operatorSetParams != operatorSetParams"
            );
        }

        for (uint8 i = 0; i < minimumStakeForQuourm.length; ++i) {
            require(
                stakeRegistry.minimumStakeForQuorum(i) ==
                    minimumStakeForQuourm[i],
                "stakeRegistry.minimumStakeForQuourm != minimumStakeForQuourm"
            );
        }

        for (uint8 i = 0; i < strategyAndWeightingMultipliers.length; ++i) {
            for (
                uint8 j = 0;
                j < strategyAndWeightingMultipliers[i].length;
                ++j
            ) {
                IStakeRegistry.StrategyParams
                    memory strategyParams = stakeRegistry.strategyParamsByIndex(
                        i,
                        j
                    );
                require(
                    address(strategyParams.strategy) ==
                        address(strategyAndWeightingMultipliers[i][j].strategy),
                    "stakeRegistry.strategyAndWeightingMultipliers != strategyAndWeightingMultipliers"
                );
                require(
                    strategyParams.multiplier ==
                        strategyAndWeightingMultipliers[i][j].multiplier,
                    "stakeRegistry.strategyAndWeightingMultipliers != strategyAndWeightingMultipliers"
                );
            }
        }

        require(
            operatorSetParams.length ==
                strategyAndWeightingMultipliers.length &&
                operatorSetParams.length == minimumStakeForQuourm.length,
            "operatorSetParams, strategyAndWeightingMultipliers, and minimumStakeForQuourm must be the same length"
        );
    }

    function _writeOutput(
        string memory config_data,
        string memory outputPath
    ) internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";
        vm.serializeAddress(
            deployed_addresses,
            "alignedLayerProxyAdmin",
            address(alignedLayerProxyAdmin)
        );
        vm.serializeAddress(
            deployed_addresses,
            "operatorStateRetriever",
            address(operatorStateRetriever)
        );
        vm.serializeAddress(
            deployed_addresses,
            "alignedLayerServiceManager",
            address(alignedLayerServiceManager)
        );
        vm.serializeAddress(
            deployed_addresses,
            "alignedLayerServiceManagerImplementation",
            address(alignedLayerServiceManagerImplementation)
        );
        vm.serializeAddress(
            deployed_addresses,
            "registryCoordinator",
            address(registryCoordinator)
        );
        vm.serializeAddress(
            deployed_addresses,
            "registryCoordinatorImplementation",
            address(registryCoordinatorImplementation)
        );
        vm.serializeAddress(
            deployed_addresses,
            "blsApkRegistry",
            address(apkRegistry)
        );
        vm.serializeAddress(
            deployed_addresses,
            "blsApkRegistryImplementation",
            address(apkRegistryImplementation)
        );
        vm.serializeAddress(
            deployed_addresses,
            "indexRegistry",
            address(indexRegistry)
        );
        vm.serializeAddress(
            deployed_addresses,
            "indexRegistryImplementation",
            address(indexRegistryImplementation)
        );
        vm.serializeAddress(
            deployed_addresses,
            "stakeRegistry",
            address(stakeRegistry)
        );
        vm.serializeAddress(
            deployed_addresses,
            "stakeRegistryImplementation",
            address(stakeRegistryImplementation)
        );
        vm.serializeAddress(
            deployed_addresses,
            "serviceManagerRouter",
            address(serviceManagerRouter)
        );
        string memory deployed_addresses_output = vm.serializeAddress(
            deployed_addresses,
            "stakeRegistryImplementation",
            address(stakeRegistryImplementation)
        );

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(
            chain_info,
            "chainId",
            block.chainid
        );

        address churner = stdJson.readAddress(
            config_data,
            ".permissions.churner"
        );
        address ejector = stdJson.readAddress(
            config_data,
            ".permissions.ejector"
        );
        address alignedLayerAggregator = stdJson.readAddress(
            config_data,
            ".permissions.aggregator"
        );
        string memory permissions = "permissions";
        vm.serializeAddress(
            permissions,
            "alignedLayerOwner",
            alignedLayerOwner
        );
        vm.serializeAddress(
            permissions,
            "alignedLayerUpgrader",
            alignedLayerUpgrader
        );
        vm.serializeAddress(permissions, "alignedLayerChurner", churner);

        vm.serializeAddress(permissions, "pauserRegistry", pauser);

        vm.serializeAddress(permissions, "alignedLayerAggregator", alignedLayerAggregator);

        string memory permissions_output = vm.serializeAddress(
            permissions,
            "alignedLayerEjector",
            ejector
        );

        vm.serializeString(parent_object, chain_info, chain_info_output);
        vm.serializeString(
            parent_object,
            deployed_addresses,
            deployed_addresses_output
        );
        string memory finalJson = vm.serializeString(
            parent_object,
            permissions,
            permissions_output
        );
        vm.writeJson(finalJson, outputPath);
    }

    function _parseStakeRegistryParams(
        string memory config_data
    )
        internal
        pure
        returns (
            uint96[] memory minimumStakeForQuourm,
            IStakeRegistry.StrategyParams[][]
                memory strategyAndWeightingMultipliers
        )
    {
        bytes memory stakesConfigsRaw = stdJson.parseRaw(
            config_data,
            ".minimumStakes"
        );
        minimumStakeForQuourm = abi.decode(stakesConfigsRaw, (uint96[]));

        bytes memory strategyConfigsRaw = stdJson.parseRaw(
            config_data,
            ".strategyWeights"
        );
        strategyAndWeightingMultipliers = abi.decode(
            strategyConfigsRaw,
            (IStakeRegistry.StrategyParams[][])
        );
    }

    function _parseRegistryCoordinatorParams(
        string memory config_data
    )
        internal
        returns (
            IRegistryCoordinator.OperatorSetParam[] memory operatorSetParams,
            address churner,
            address ejector
        )
    {
        bytes memory operatorConfigsRaw = stdJson.parseRaw(
            config_data,
            ".operatorSetParams"
        );
        operatorSetParams = abi.decode(
            operatorConfigsRaw,
            (IRegistryCoordinator.OperatorSetParam[])
        );

        churner = stdJson.readAddress(config_data, ".permissions.churner");
        ejector = stdJson.readAddress(config_data, ".permissions.ejector");
    }
}
