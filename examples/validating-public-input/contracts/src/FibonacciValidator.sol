// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

contract FibonacciValidator {
    address public alignedServiceManager;
    bytes32 public fibonacciProgramId;

    bytes32 public fibonacciProgramIdCommitment =
        0x069ed9f3972550a2901523723f4beb5e240749dcafa30e1623d0778e17d69d70;

    event FibonacciNumbers(uint32 fibN, uint32 fibNPlusOne);

    constructor(address _alignedServiceManager) {
        alignedServiceManager = _alignedServiceManager;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 programIdCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        bytes memory pubInputBytes
    ) public returns (bool) {
        require(
            fibonacciProgramIdCommitment == programIdCommitment,
            "Program ID doesn't match"
        );

        require(
            pubInputCommitment == keccak256(abi.encodePacked(pubInputBytes)),
            "Fibonacci numbers don't match with public input"
        );

        (
            bool callWasSuccessful,
            bytes memory proofIsIncluded
        ) = alignedServiceManager.staticcall(
                abi.encodeWithSignature(
                    "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)",
                    proofCommitment,
                    pubInputCommitment,
                    programIdCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot,
                    merkleProof,
                    verificationDataBatchIndex
                )
            );

        require(callWasSuccessful, "static_call failed");

        (uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(pubInputBytes);

        emit FibonacciNumbers(fibN, fibNPlusOne);

        return abi.decode(proofIsIncluded, (bool));
    }

    function bytesToTwoUint32(
        bytes memory data
    ) public pure returns (uint32, uint32) {
        require(data.length >= 8, "Input bytes must be at least 8 bytes long");

        uint32 first = uint32(uint8(data[0])) |
            (uint32(uint8(data[1])) << 8) |
            (uint32(uint8(data[2])) << 16) |
            (uint32(uint8(data[3])) << 24);

        uint32 second = uint32(uint8(data[4])) |
            (uint32(uint8(data[5])) << 8) |
            (uint32(uint8(data[6])) << 16) |
            (uint32(uint8(data[7])) << 24);

        return (first, second);
    }
}
