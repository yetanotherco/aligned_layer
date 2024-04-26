// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import "eigenlayer-core/contracts/permissions/PauserRegistry.sol";
import "eigenlayer-core/contracts/strategies/StrategyBase.sol";
import "eigenlayer-core/contracts/interfaces/IStrategyManager.sol";

import "src/core/ERC20Mock.sol";
import "script/deploy/utils/ExistingDeploymentParser.sol";

contract MockStrategyDeployer is ExistingDeploymentParser {
    string public existingDeploymentInfoPath =
        string(
            bytes("./script/output/devnet/eigenlayer_deployment_output.json")
        );

    string public outputPath =
        string(
            bytes("./script/output/devnet/strategy_deployment_output.json")
        );

    // ERC20 and Strategy: we need to deploy this erc20, create a strategy for it, and whitelist this strategy in the StrategyManager
    ERC20Mock public erc20Mock;
    StrategyBase public erc20MockStrategy;

    function run() external {
        _parseDeployedContracts(existingDeploymentInfoPath);

        vm.startBroadcast();

        erc20Mock = new ERC20Mock();
        erc20MockStrategy = StrategyBase(
            address(
                new TransparentUpgradeableProxy(
                    address(baseStrategyImplementation),
                    address(eigenLayerProxyAdmin),
                    abi.encodeWithSelector(
                        StrategyBaseTVLLimits.initialize.selector,
                        1 ether, // maxPerDeposit
                        100 ether, // maxDeposits
                        IERC20(erc20Mock),
                        eigenLayerPauserReg
                    )
                )
            )
        );

        IStrategy[] memory strats = new IStrategy[](1);
        strats[0] = erc20MockStrategy;
        bool[] memory reject = new bool[](1);
        reject[0] = false;

        strategyManager.addStrategiesToDepositWhitelist(strats, reject);

        vm.stopBroadcast();

        _writeDeploymentInfo();
    }

    function _writeDeploymentInfo() internal {
        string memory parent_object = "parent object";
        vm.serializeAddress(parent_object, "erc20Mock", address(erc20Mock));
        string memory finalJson = vm.serializeAddress(parent_object, "erc20MockStrategy", address(erc20MockStrategy));
        vm.writeJson(finalJson, outputPath);
    }
}
