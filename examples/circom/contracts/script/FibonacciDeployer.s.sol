// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {FibonacciValidator} from "../src/Fibonacci.sol";

contract FibonacciDeployer is Script {
    function run(address _alignedServiceManager, address _paymentServiceAddr) public returns (address) {
        vm.startBroadcast();

        FibonacciValidator contractAddress = new FibonacciValidator(_alignedServiceManager, _paymentServiceAddr);

        vm.stopBroadcast();
        return address(contractAddress);
    }
}
