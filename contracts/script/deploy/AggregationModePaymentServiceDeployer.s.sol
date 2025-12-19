pragma solidity ^0.8.12;

import {AggregationModePaymentService} from "../../src/core/AggregationModePaymentService.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract AggregationModePaymentServiceDeployer is Script {
    function run(string memory configPath, string memory outputPath) external returns (address, address) {
        string memory configData = vm.readFile(configPath);

        address owner = stdJson.readAddress(configData, ".permissions.paymentServiceOwner");
        address admin = stdJson.readAddress(configData, ".permissions.paymentServiceAdmin");
        address recipient = stdJson.readAddress(configData, ".permissions.recipient");
        uint256 amountToPay = stdJson.readUint(configData, ".amounts.amountToPayInWei");
        uint256 paymentExpirationTimeSeconds = stdJson.readUint(configData, ".amounts.paymentExpirationTimeSeconds");
        uint256 subscriptionLimit = stdJson.readUint(configData, ".amounts.subscriptionLimit");
        uint256 maxSubscriptionTimeAhead = stdJson.readUint(configData, ".amounts.maxSubscriptionTimeAhead");

        vm.startBroadcast();

        AggregationModePaymentService implementation = new AggregationModePaymentService();
        ERC1967Proxy proxy = new ERC1967Proxy(
            address(implementation),
            abi.encodeWithSignature(
                "initialize(address,address,address,uint256,uint256,uint256,uint256)",
                owner,
                admin,
                recipient,
                amountToPay,
                paymentExpirationTimeSeconds,
                subscriptionLimit,
                maxSubscriptionTimeAhead
            )
        );

        vm.stopBroadcast();

        string memory addresses = "addresses";
        vm.serializeAddress(addresses, "aggregationModePaymentService", address(proxy));
        string memory addressesStr =
            vm.serializeAddress(addresses, "aggregationModePaymentServiceImplementation", address(implementation));

        string memory parentObject = "parent";
        string memory finalJson = vm.serializeString(parentObject, "addresses", addressesStr);
        vm.writeJson(finalJson, outputPath);

        return (address(proxy), address(implementation));
    }
}
