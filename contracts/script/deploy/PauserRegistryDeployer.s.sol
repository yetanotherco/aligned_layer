// SPDX-License-Identifier: BUSL-1.1
pragma solidity =0.8.12;

import {PauserRegistry} from "eigenlayer-core/contracts/permissions/PauserRegistry.sol";

import "script/deploy/utils/ExistingDeploymentParser.sol";
import "forge-std/Test.sol";
import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

// This script is only for deploying the PauserRegistry contract
// In order to avoid redeploying previously deployed contracts, we will use the deployed contract addresses
contract PauserRegistryDeployer is ExistingDeploymentParser {
    address public pauser;
    uint256 public initalPausedStatus;
    address public deployer;

    PauserRegistry public pauserRegistry;

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

        initalPausedStatus = stdJson.readUint(
            config_data,
            ".permissions.initalPausedStatus"
        );
        pauser = stdJson.readAddress(
            config_data,
            ".permissions.pauser"
        );

        deployer = stdJson.readAddress(config_data, ".permissions.deployer");
        require(
            deployer == tx.origin,
            "Deployer address must be the same as the tx.origin"
        );
        emit log_named_address("You are deploying from", deployer);

        vm.startBroadcast();

        //deploy pauser registry
        {
            address[] memory pausers = new address[](1);
            pausers[0] = pauser;
            pauserRegistry = new PauserRegistry(pausers, pauser); // (pausers, unpauser)
        }

        vm.stopPrank();

        //write output
        _writeOutput(config_data, outputPath);
    }

    function _writeOutput(string memory config_data, string memory outputPath) internal {
        string memory parent_object = "parent object";

        string memory deployed_addresses = "addresses";

        string memory deployed_addresses_output = vm.serializeAddress(
            deployed_addresses,
            "pauserRegistry",
            address(pauserRegistry)
        );

        string memory chain_info = "chainInfo";
        vm.serializeUint(chain_info, "deploymentBlock", block.number);
        string memory chain_info_output = vm.serializeUint(
            chain_info,
            "chainId",
            block.chainid
        );

        address pauserAddress = stdJson.readAddress(
            config_data,
            ".permissions.pauser"
        );
        string memory permissions = "permissions";

        string memory permissions_output = vm.serializeAddress(
            permissions,
            "alignedLayerPauser",
            pauserAddress
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
}
