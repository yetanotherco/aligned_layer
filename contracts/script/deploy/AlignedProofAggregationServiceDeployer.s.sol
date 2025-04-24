pragma solidity ^0.8.12;

import {AlignedProofAggregationService} from "../../src/core/AlignedProofAggregationService.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract AlignedProofAggregationServiceDeployer is Script {
    function run(string memory configPath, string memory outputPath) external returns (address, address) {
        string memory config_data = vm.readFile(configPath);

        address alignedAggregatorAddress = stdJson.readAddress(config_data, ".address.alignedAggregatorAddress");
        address sp1VerifierAddress = stdJson.readAddress(config_data, ".address.sp1VerifierAddress");
        address risc0VerifierAddress = stdJson.readAddress(config_data, ".address.risc0VerifierAddress");

        address ownerAddress = stdJson.readAddress(config_data, ".permissions.owner");

        vm.startBroadcast();

        AlignedProofAggregationService alignedProofAggregationService = new AlignedProofAggregationService();

        ERC1967Proxy proxy = new ERC1967Proxy(
            address(alignedProofAggregationService),
            abi.encodeWithSignature(
                "initialize(address,address,address,address)",
                ownerAddress,
                alignedAggregatorAddress,
                sp1VerifierAddress,
                risc0VerifierAddress
            )
        );

        vm.stopBroadcast();

        string memory addresses = "addresses";
        vm.serializeAddress(addresses, "alignedProofAggregationService", address(proxy));
        string memory addressesStr = vm.serializeAddress(
            addresses, "alignedProofAggregationServiceImplementation", address(alignedProofAggregationService)
        );

        string memory parentObject = "parent";
        string memory finalJson = vm.serializeString(parentObject, "addresses", addressesStr);
        vm.writeJson(finalJson, outputPath);

        return (address(proxy), address(alignedProofAggregationService));
    }
}
