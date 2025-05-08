// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {StateTransition} from "../src/StateTransition.sol";

contract StateTransitionDeployer is Script {
    StateTransition public stateTransitionContract;

    function setUp() public {}

    function run(address _alignedProofAggregationService) public returns (address) {
        vm.startBroadcast();

        stateTransitionContract = new StateTransition(_alignedProofAggregationService);

        vm.stopBroadcast();

        return address(stateTransitionContract);
    }
}
