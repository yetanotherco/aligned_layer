// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script, console} from "forge-std/Script.sol";
import {FibonacciValidator} from "../src/FibonacciValidator.sol";

contract FibonacciDeployer is Script {
    function setUp() public {}

    function run(
        address _alignedServiceManager,
        address _paymentServiceAddr
    ) external returns (address) {
        vm.startBroadcast();

        FibonacciValidator fibonacciContract = new FibonacciValidator(
            _alignedServiceManager,
            _paymentServiceAddr
        );

        vm.stopBroadcast();

        return address(fibonacciContract);
    }
}
