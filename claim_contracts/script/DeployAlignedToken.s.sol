// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "../src/AlignedToken.sol";
import "../src/ClaimableAirdrop.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "forge-std/Script.sol";
import {Vm} from "forge-std/Vm.sol";
import {Utils} from "./Utils.sol";

contract DeployAlignedToken is Script {
    function run(string memory config) public {
        string memory root = vm.projectRoot();
        string memory path = string.concat(
            root,
            "/script-config/config.",
            config,
            ".json"
        );
        string memory config_json = vm.readFile(path);

        address _foundation = stdJson.readAddress(config_json, ".foundation");
        address _tokenDistributor = stdJson.readAddress(
            config_json,
            ".tokenDistributor"
        );

        vm.broadcast();
        AlignedToken _token = new AlignedToken();

        console.log("Aligned Token deployed at address:", address(_token));

        vm.broadcast();
        TransparentUpgradeableProxy _tokenProxy = new TransparentUpgradeableProxy(
                address(_token),
                _foundation,
                Utils.alignedTokenInitData(_foundation, _tokenDistributor)
            );

        bytes memory _alignedTokenProxyConstructorData = Utils
            .alignedTokenProxyConstructorData(
                address(_token),
                _foundation,
                _tokenDistributor
            );

        console.log(
            string.concat(
                "Aligned Token Proxy deployed at address: ",
                vm.toString(address(_tokenProxy)),
                " with proxy admin: ",
                vm.toString(Utils.getAdminAddress(address(_tokenProxy))),
                " and owner: ",
                vm.toString(_foundation),
                " with constructor args: ",
                vm.toString(_alignedTokenProxyConstructorData)
            )
        );
    }
}
