// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/forge-std/src//Script.sol";
import {StateTransition} from "../src/StateTransition.sol";

contract StateTransitionDeployer is Script {
    StateTransition public stateTransitionContract;

    function setUp() public {}

    function run(bytes32 programId, bytes32 initialStateRoot, address _alignedProofAggregationService, address owner)
        public
        returns (address)
    {
        vm.startBroadcast();

        stateTransitionContract =
            new StateTransition(programId, initialStateRoot, _alignedProofAggregationService, owner);

        vm.stopBroadcast();

        return address(stateTransitionContract);
    }
}
