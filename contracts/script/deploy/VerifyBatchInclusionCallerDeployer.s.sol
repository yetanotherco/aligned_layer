pragma solidity =0.8.12;

import {Script} from "forge-std/Script.sol";
import {VerifyBatchInclusionCaller} from "../../src/core/VerifyBatchInclusionCaller.sol";

contract VerifyBatchInclusionCallerDeployer is Script {
    function run(address _targetContract) external returns (address) {

        vm.startBroadcast();

        VerifyBatchInclusionCaller verifyBatchInclusionCaller = new VerifyBatchInclusionCaller(_targetContract);
        
        vm.stopBroadcast();

        return address(verifyBatchInclusionCaller);
    }
}

