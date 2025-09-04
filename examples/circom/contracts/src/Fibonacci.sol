// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract FibonacciValidator {
    address public alignedServiceManager;
    address public paymentServiceAddr;

    bytes32 public fibonacciProgramVk;

    error ProofVerificationFailed();

    uint256 fibonacciNumber = 0;

    constructor(address _alignedServiceManager, address _paymentServiceAddr, bytes32 programVk) {
        alignedServiceManager = _alignedServiceManager;
        paymentServiceAddr = _paymentServiceAddr;
        fibonacciProgramVk = programVk;
    }

    function setNewNumber(
        bytes32 proofCommitment,
        bytes memory pubInputBytes,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) public {
        bytes32 pubInputCommitment = keccak256(pubInputBytes);
        (bool callWasSuccessful, bytes memory response) = alignedServiceManager.staticcall(
            abi.encodeWithSignature(
                "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)",
                proofCommitment,
                pubInputCommitment,
                fibonacciProgramVk,
                proofGeneratorAddr,
                batchMerkleRoot,
                merkleProof,
                verificationDataBatchIndex,
                paymentServiceAddr
            )
        );

        bool proofVerified = abi.decode(response, (bool));
        if (!proofVerified) {
            revert ProofVerificationFailed();
        }

        require(callWasSuccessful, "static_call failed");

        uint256 number = abi.decode(pubInputBytes, (uint256));
        fibonacciNumber = number;
    }
}
