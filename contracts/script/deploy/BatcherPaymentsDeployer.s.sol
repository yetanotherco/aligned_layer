pragma solidity =0.8.12;

import {BatcherPayments} from "../../src/core/BatcherPayments.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract BatcherPaymentsDeployer is Script {
    function run(
        string memory batcherConfigPath
    ) external returns (address) {

        // READ JSON CONFIG DATA
        string memory config_data = vm.readFile(batcherConfigPath);

        address batcherWallet = stdJson.readAddress(
            config_data,
            ".address.batcherWallet"
        );

        address alignedLayerServiceManager = stdJson.readAddress(
            config_data,
            ".address.alignedLayerServiceManager"
        );

        vm.startBroadcast();

        BatcherPayments batcherPayments = new BatcherPayments(alignedLayerServiceManager, batcherWallet);
        // batcherPayments.initialize(
        //     _AlignedLayerServiceManager,
        //     _BatcherWallet
        // );
        
        vm.stopBroadcast();

        return address(batcherPayments);
    }
}

