// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {BaseScript} from "../../lib/sp1-contracts/contracts/script/utils/Base.s.sol";
import {SP1Verifier} from "../../lib/sp1-contracts/contracts/src/v4.0.0-rc.3/SP1VerifierGroth16.sol";
import {SP1VerifierGateway} from "../../lib/sp1-contracts/contracts/src/SP1VerifierGateway.sol";
import {ISP1VerifierWithHash} from "../../lib/sp1-contracts/contracts/src/ISP1Verifier.sol";

contract SP1VerifierScript is BaseScript {
    string internal constant KEY = "V4_0_0_RC3_SP1_VERIFIER_GROTH16";

    function run() external multichain(KEY) broadcaster {
        // Read config
        bytes32 CREATE2_SALT = readBytes32("CREATE2_SALT");
        address SP1_VERIFIER_GATEWAY = readAddress("SP1_VERIFIER_GATEWAY_GROTH16");

        // Deploy contract
        address verifier = address(new SP1Verifier{salt: CREATE2_SALT}());

        // Add the verifier to the gateway
        SP1VerifierGateway gateway = SP1VerifierGateway(SP1_VERIFIER_GATEWAY);
        gateway.addRoute(verifier);

        // Write address
        writeAddress(KEY, verifier);
    }

    function freeze() external multichain(KEY) broadcaster {
        // Read config
        address SP1_VERIFIER_GATEWAY = readAddress("SP1_VERIFIER_GATEWAY_GROTH16");
        address SP1_VERIFIER = readAddress(KEY);

        // Freeze the verifier on the gateway
        SP1VerifierGateway gateway = SP1VerifierGateway(SP1_VERIFIER_GATEWAY);
        bytes4 selector = bytes4(ISP1VerifierWithHash(SP1_VERIFIER).VERIFIER_HASH());
        gateway.freezeRoute(selector);
    }
}
