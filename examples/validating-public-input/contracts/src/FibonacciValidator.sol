// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract FibonacciValidator {
    address public alignedServiceManager;
    bytes32 public fibonacciImageId;

    bytes32 public fibonacciImageIdCommitment =
        0xbfa561e384be753bd6fd75b15db31eb511cd114ec76d619a87c2342af0ee1ed7;

    event FibonacciNumbersVerified(uint32 fibN, uint32 fibNPlusOne);

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
        uint32 fibN,
        uint32 fibNPlusOne
    ) public returns (bool) {
        require(
            fibonacciImageIdCommitment == provingSystemAuxDataCommitment,
            "Image ID doesn't match"
        );

        bytes32 calculatedCommitment = calculateCommitment(fibN, fibNPlusOne);

        require(
            pubInputCommitment == calculatedCommitment,
            "Fibonacci numbers don't match with public input"
        );

        // Emit the event after the require statements
        emit FibonacciNumbersVerified(fibN, fibNPlusOne);

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

    function calculateCommitment(
        uint32 fibN,
        uint32 fibNPlusOne
    ) public pure returns (bytes32) {
        bytes memory encoded = abi.encodePacked(fibN, fibNPlusOne);
        return keccak256(encoded);
    }
}
