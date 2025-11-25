pragma solidity ^0.8.12;

import {AlignedProofAggregationService} from "../../src/core/AlignedProofAggregationService.sol";
import {IAlignedProofAggregationService} from "../../src/core/IAlignedProofAggregationService.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract AlignedProofAggregationServiceDeployer is Script {
    function run(string memory configPath, string memory outputPath) external returns (address, address) {
        string memory config_data = vm.readFile(configPath);

        address alignedAggregatorAddress = stdJson.readAddress(config_data, ".address.alignedAggregatorAddress");
        address sp1VerifierAddress = stdJson.readAddress(config_data, ".address.sp1VerifierAddress");
        bytes32 sp1AggregationProgramVKHash =
            stdJson.readBytes32(config_data, ".programs_id.sp1AggregationProgramVKHash");
        address risc0VerifierAddress = stdJson.readAddress(config_data, ".address.risc0VerifierAddress");
        bytes32 risc0AggregationProgramImageId =
            stdJson.readBytes32(config_data, ".programs_id.risc0AggregationProgramImageId");

        address ownerAddress = stdJson.readAddress(config_data, ".permissions.owner");

        vm.startBroadcast();

        AlignedProofAggregationService alignedProofAggregationService = new AlignedProofAggregationService();

        bytes32[] memory programIds = new bytes32[](2);
        programIds[0] = risc0AggregationProgramImageId;
        programIds[1] = sp1AggregationProgramVKHash;

        uint8[] memory verifierTypes = new uint8[](2);
        verifierTypes[0] = uint8(IAlignedProofAggregationService.VerifierType.RISC0);
        verifierTypes[1] = uint8(IAlignedProofAggregationService.VerifierType.SP1);

        ERC1967Proxy proxy = new ERC1967Proxy(
            address(alignedProofAggregationService),
            abi.encodeWithSignature(
                "initialize(address,address,address,address,bytes32[],uint8[])",
                ownerAddress,
                alignedAggregatorAddress,
                sp1VerifierAddress,
                risc0VerifierAddress,
                programIds,
                verifierTypes
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
