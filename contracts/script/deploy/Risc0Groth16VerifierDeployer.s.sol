// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {IRiscZeroVerifier} from "@risc0-contracts/IRiscZeroVerifier.sol";
import {ControlID, RiscZeroGroth16Verifier} from "@risc0-contracts/groth16/RiscZeroGroth16Verifier.sol";

contract Risc0VerifierRouterDeployer is Script {
    function run() external {
        uint256 deployerKey = uint256(vm.envBytes32("DEPLOYER_PRIVATE_KEY"));

        vm.startBroadcast(deployerKey);

        IRiscZeroVerifier verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
        console2.log("Deployed RiscZeroGroth16Verifier to", address(verifier));

        vm.stopBroadcast();
    }
}
