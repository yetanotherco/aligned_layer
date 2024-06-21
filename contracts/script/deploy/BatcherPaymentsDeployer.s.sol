pragma solidity =0.8.12;

import {Script} from "forge-std/Script.sol";
import {BatcherPayments} from "../../src/core/BatcherPayments.sol";

contract BatcherPaymentsDeployer is Script {
    function run(address _AlignedLayerServiceManager, address _BatcherWallet) external returns (address) {

        vm.startBroadcast();

        BatcherPayments batcherPayments = new BatcherPayments(_AlignedLayerServiceManager, _BatcherWallet);
        // batcherPayments.initialize(
        //     _AlignedLayerServiceManager,
        //     _BatcherWallet
        // );
        
        vm.stopBroadcast();

        return address(batcherPayments);
    }
}

