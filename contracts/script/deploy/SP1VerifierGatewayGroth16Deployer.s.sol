// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {BaseScript} from "../../lib/sp1-contracts/contracts/script/utils/Base.s.sol";
import {SP1VerifierGateway} from "../../lib/sp1-contracts/contracts/src/SP1VerifierGateway.sol";

contract SP1VerifierGatewayScript is BaseScript {
    string internal constant KEY = "SP1_VERIFIER_GATEWAY_GROTH16";

    function run() external multichain(KEY) broadcaster {
        // Read config
        bytes32 CREATE2_SALT = readBytes32("CREATE2_SALT");
        address OWNER = readAddress("OWNER");

        // Deploy contract
        address gateway = address(new SP1VerifierGateway{salt: CREATE2_SALT}(OWNER));

        // Write addresss
        writeAddress(KEY, gateway);
    }
}
