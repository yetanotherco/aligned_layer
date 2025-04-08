pragma solidity ^0.8.12;

interface IAlignedProofAggregationService {

    /// @notice Method to verify an aggregated proof from aligned
    /// @dev This function is called by the aligned proof aggregator after collecting the proofs and aggregating them
    /// to be verified on-chain. We expect the blobTransactionHash to be called before
    /// @param blobVersionedHash the versioned hash of the blob transaction that contains the leaves that compose the merkle root.
    /// @param sp1ProgramVKey Public verifying key
    /// @param sp1PublicValues Values used to perform the execution
    /// @param sp1ProofBytes Groth16 proof
    function verify(
        bytes32 blobVersionedHash,
        bytes32 sp1ProgramVKey,
        bytes calldata sp1PublicValues,
        bytes calldata sp1ProofBytes
    ) external;

    /// @notice event that gets emitted after a successful aggregated proof verification
    event AggregatedProofVerified(
        bytes32 indexed merkleRoot, bytes32 blobVersionedHash
    );

    error OnlyAlignedAggregator(address sender);
}
