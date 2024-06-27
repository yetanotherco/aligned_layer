pragma solidity =0.8.12;

import {BatcherPaymentService} from "../../src/core/BatcherPaymentService.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract BatcherPaymentServiceDeployer is Script {
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

        uint256 thisTxBaseGasCost = stdJson.readUint(
            config_data,
            ".amounts.thisTxBaseGasCost"
        );

        uint256 createTaskGasCost = stdJson.readUint(
            config_data,
            ".amounts.createTaskGasCost"
        );

        uint256 extraUserTxGasCost = stdJson.readUint(
            config_data,
            ".amounts.extraUserTxGasCost"
        );

        vm.startBroadcast();

        BatcherPaymentService batcherPaymentService = new BatcherPaymentService();
        ERC1967Proxy proxy = new ERC1967Proxy(address(batcherPaymentService), "");
        BatcherPaymentService(payable(address(proxy))).initialize(
            alignedLayerServiceManager,
            batcherWallet,
            thisTxBaseGasCost,
            createTaskGasCost,
            extraUserTxGasCost
        );
        
        vm.stopBroadcast();

        return address(proxy);
    }
}

