// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract StateTransition {
    event StateUpdated(bytes32);
    event ProgramIdUpdated(bytes32);

    error OnlyOwner(address);
    error AlignedVerifyProofInclusionCallFailed();
    error ProofVerificationFailed();
    error PrevStateRootDidNotMatch();

    bytes32 public PROGRAM_ID;
    bytes32 public stateRoot;
    address public alignedProofAggregator;
    address public owner;

    constructor(bytes32 programId, bytes32 initialStateRoot, address _alignedProofAggregator, address _owner) {
        alignedProofAggregator = _alignedProofAggregator;
        owner = _owner;
        PROGRAM_ID = programId;
        stateRoot = initialStateRoot;
    }

    function updateState(bytes calldata publicInputs, bytes32[] calldata merkleProof) public onlyOwner {
        bytes memory callData = abi.encodeWithSignature(
            "verifyProofInclusion(bytes32[],bytes32,bytes)", merkleProof, PROGRAM_ID, publicInputs
        );
        (bool callResult, bytes memory response) = alignedProofAggregator.staticcall(callData);
        if (!callResult) {
            revert AlignedVerifyProofInclusionCallFailed();
        }

        bool proofVerified = abi.decode(response, (bool));
        if (!proofVerified) {
            revert ProofVerificationFailed();
        }

        (bytes32 prevStateRoot, bytes32 newStateRoot) = abi.decode(publicInputs, (bytes32, bytes32));
        if (prevStateRoot != stateRoot) {
            revert PrevStateRootDidNotMatch();
        }

        stateRoot = newStateRoot;
        emit StateUpdated(stateRoot);
    }

    function setProgramId(bytes32 programId) public onlyOwner {
        PROGRAM_ID = programId;

        emit ProgramIdUpdated(PROGRAM_ID);
    }

    modifier onlyOwner() {
        if (msg.sender != owner) {
            revert OnlyOwner(msg.sender);
        }
        _;
    }
}
