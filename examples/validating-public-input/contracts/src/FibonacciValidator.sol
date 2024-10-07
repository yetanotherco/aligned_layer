// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

contract FibonacciValidator {
    address public alignedServiceManager;
    address public paymentServiceAddr;
    bytes32 public fibonacciProgramId;

    bytes32 public fibonacciProgramIdCommitmentSp1 =
        0xc2633817b79f9a9bf340981ecc6d693ce8b0b0494114277bd5a4b4083675fa32;
    
    bytes32 public fibonacciProgramIdCommitmentRisc0 =
        0x0d5a7c5ae964f52be4c8d288b41fece9e6fec521ad813b323719bc86e859a5d2;

    event FibonacciNumbers(uint32 n, uint32 fibN, uint32 fibNPlusOne);

    constructor(address _alignedServiceManager, address _paymentServiceAddr) {
        alignedServiceManager = _alignedServiceManager;
        paymentServiceAddr = _paymentServiceAddr;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 programIdCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        bytes memory pubInputBytes,
        string memory verifierId
    ) public returns (bool) {
        if (keccak256(abi.encodePacked(verifierId)) == keccak256(abi.encodePacked("SP1"))) {
            require(
                fibonacciProgramIdCommitmentSp1 == programIdCommitment,
                "Program ID doesn't match"
            );
        } else if (keccak256(abi.encodePacked(verifierId)) == keccak256(abi.encodePacked("Risc0"))) {
            require(
                fibonacciProgramIdCommitmentRisc0 == programIdCommitment,
                "Program ID doesn't match"
            );
        } else {
            revert("Verifier ID not recognized, use Risc0 or SP1");
        }

        require(
            pubInputCommitment == keccak256(abi.encodePacked(pubInputBytes)),
            "Fibonacci numbers don't match with public input"
        );

        (
            bool callWasSuccessful,
            bytes memory proofIsIncluded
        ) = alignedServiceManager.staticcall(
                abi.encodeWithSignature(
                    "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)",
                    proofCommitment,
                    pubInputCommitment,
                    programIdCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot,
                    merkleProof,
                    verificationDataBatchIndex,
                    paymentServiceAddr
                )
            );

        require(callWasSuccessful, "static_call failed");

        (uint32 n ,uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(pubInputBytes);

        emit FibonacciNumbers(n, fibN, fibNPlusOne);

        return abi.decode(proofIsIncluded, (bool));
    }

    function bytesToTwoUint32(
        bytes memory data
    ) public pure returns (uint32, uint32, uint32) {
        require(data.length >= 8, "Input bytes must be at least 8 bytes long");

        uint32 first = uint32(uint8(data[0])) |
            (uint32(uint8(data[1])) << 8) |
            (uint32(uint8(data[2])) << 16) |
            (uint32(uint8(data[3])) << 24);

        uint32 second = uint32(uint8(data[4])) |
            (uint32(uint8(data[5])) << 8) |
            (uint32(uint8(data[6])) << 16) |
            (uint32(uint8(data[7])) << 24);
        
        uint32 third = uint32(uint8(data[8])) |
            (uint32(uint8(data[9])) << 8) |
            (uint32(uint8(data[10])) << 16) |
            (uint32(uint8(data[11])) << 24);

        return (first, second, third);
    }
}
