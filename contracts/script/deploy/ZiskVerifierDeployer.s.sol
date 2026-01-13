// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {ZiskVerifier} from "../../src/zisk/ZiskVerifier.sol";

contract ZiskVerifierDeployer is Script {
    function run() external {
        uint256 deployerKey = uint256(vm.envBytes32("DEPLOYER_PRIVATE_KEY"));
        bytes32 salt = vm.envOr("CREATE2_SALT", bytes32(uint256(0)));

        vm.startBroadcast(deployerKey);

        ZiskVerifier verifier;
        if (salt != bytes32(0)) {
            // Deploy with CREATE2 for deterministic address
            verifier = new ZiskVerifier{salt: salt}();
        } else {
            // Deploy without CREATE2
            verifier = new ZiskVerifier();
        }

        console2.log("ZiskVerifier deployed at:", address(verifier));
        vm.stopBroadcast();
    }
}
