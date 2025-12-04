// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract StateTransition {
    event StateUpdated(bytes32);
    event VerifierProgramUpdated(bytes32);

    error OnlyOwner(address);
    error AlignedVerifyProofInclusionCallFailed();
    error ProofVerificationFailed();
    error PrevStateRootDidNotMatch();

    bytes32 public VERIFIER_PROGRAM_COMMITMENT;
    bytes32 public stateRoot;
    address public alignedProofAggregator;
    address public owner;

    constructor(bytes32 verifierProgramCommitment, bytes32 initialStateRoot, address _alignedProofAggregator, address _owner) {
        alignedProofAggregator = _alignedProofAggregator;
        owner = _owner;
        VERIFIER_PROGRAM_COMMITMENT = verifierProgramCommitment;
        stateRoot = initialStateRoot;
    }

    function updateState(uint16 provingSystemId, bytes calldata publicInputs, bytes32[] calldata merkleProof)
        public
        onlyOwner
    {
        bytes memory callData = abi.encodeWithSignature(
            "isProofVerified(bytes32[],uint16,bytes32,bytes)",
            merkleProof,
            provingSystemId,
            VERIFIER_PROGRAM_COMMITMENT,
            publicInputs
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

    function setVerifierProgramCommitment(bytes32 verifierProgramCommitment) public onlyOwner {
        VERIFIER_PROGRAM_COMMITMENT = verifierProgramCommitment;

        emit VerifierProgramUpdated(VERIFIER_PROGRAM_COMMITMENT);
    }

    modifier onlyOwner() {
        if (msg.sender != owner) {
            revert OnlyOwner(msg.sender);
        }
        _;
    }
}
