// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script, console} from "forge-std/Script.sol";
import {FibonacciValidator} from "../src/FibonacciValidator.sol";

contract CounterScript is Script {
    function setUp() public {}

    function run(address _targetContract) external returns (address) {
        vm.startBroadcast();

        FibonacciValidator fibonacciContract = new FibonacciValidator(
            _targetContract
        );

        vm.stopBroadcast();

        return address(fibonacciContract);
    }
}
