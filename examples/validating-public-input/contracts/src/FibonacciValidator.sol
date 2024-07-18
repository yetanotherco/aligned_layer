// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

contract FibonacciValidator {
    address public alignedServiceManager;
    bytes32 public fibonacciImageId;

    bytes32 public fibonacciImageIdCommitment =
        0xbfa561e384be753bd6fd75b15db31eb511cd114ec76d619a87c2342af0ee1ed7;

    event FibonacciNumbers(uint32 fibN, uint32 fibNPlusOne);

    constructor(address _alignedServiceManager) {
        alignedServiceManager = _alignedServiceManager;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        bytes memory journalBytes
    ) public returns (bool) {
        require(
            fibonacciImageIdCommitment == provingSystemAuxDataCommitment,
            "Image ID doesn't match"
        );

        require(
            pubInputCommitment == keccak256(abi.encodePacked(journalBytes)),
            "Fibonacci numbers don't match with public input"
        );

        (uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(journalBytes);

        emit FibonacciNumbers(fibN, fibNPlusOne);

        (
            bool callWasSuccessful,
            bytes memory proofIsIncluded
        ) = alignedServiceManager.staticcall(
                abi.encodeWithSignature(
                    "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)",
                    proofCommitment,
                    pubInputCommitment,
                    provingSystemAuxDataCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot,
                    merkleProof,
                    verificationDataBatchIndex
                )
            );

        require(callWasSuccessful, "static_call failed");

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
