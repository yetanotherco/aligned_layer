// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract StateTransition {
    event StateUpdated(bytes32);

    bytes32 public PROGRAM_ID = 0x00;
    bytes32 public stateRoot;
    address public alignedProofAggregator;

    constructor(address _alignedProofAggregator) {
        alignedProofAggregator = _alignedProofAggregator;
    }

    function updateState(bytes calldata publicInputs, bytes32[] calldata merkleProof) public {
        bytes memory callData = abi.encodeWithSignature(
            "verifyProofInclusion(bytes32[],bytes32,bytes)", merkleProof, PROGRAM_ID, publicInputs
        );
        (bool callResult, bytes memory response) = alignedProofAggregator.staticcall(callData);
        require(callResult, "static_call failed");

        bool proofVerified = abi.decode(response, (bool));
        require(proofVerified, "proof not verified in aligned");

        (bytes32 prevStateRoot, bytes32 newStateRoot) = abi.decode(publicInputs, (bytes32, bytes32));
        require(prevStateRoot == stateRoot);

        stateRoot = newStateRoot;

        emit StateUpdated(stateRoot);
    }
}
