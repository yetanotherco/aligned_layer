pragma solidity =0.8.12;

import {BatcherPayments} from "../../src/core/BatcherPayments.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

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

        uint256 this_tx_base_gas_cost = stdJson.readUint(
            config_data,
            ".amounts.this_tx_base_gas_cost"
        );

        uint256 create_task_gas_price = stdJson.readUint(
            config_data,
            ".amounts.create_task_gas_price"
        );

        uint256 extra_user_tx_gas_cost = stdJson.readUint(
            config_data,
            ".amounts.extra_user_tx_gas_cost"
        );

        vm.startBroadcast();

        BatcherPayments batcherPayments = new BatcherPayments();
        ERC1967Proxy proxy = new ERC1967Proxy(address(batcherPayments), "");
        BatcherPayments(payable(address(proxy))).initialize(
            alignedLayerServiceManager,
            batcherWallet,
            this_tx_base_gas_cost,
            create_task_gas_price,
            extra_user_tx_gas_cost
        );
        
        vm.stopBroadcast();

        return address(proxy);
    }
}

