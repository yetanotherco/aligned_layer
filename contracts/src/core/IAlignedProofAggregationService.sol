pragma solidity ^0.8.12;

interface IAlignedProofAggregationService {
    /// @notice aggregated proof status
    /// Verified -> Verification was successful
    /// Failed -> Verification failed
    /// Missed -> Internal error in the aligned service could not send the aggregated proof to verify
    enum AggregatedProofStatus {
        Verified,
        Failed,
        Missed
    }

    /// @notice aggregated proof
    /// Status -> proof status
    /// blobHash -> the hash of the blob transaction containing the hashes of all the proofs that have been aggregated
    /// merkleRoot -> the committed merkle root in the aggregated
    struct AggregatedProof {
        AggregatedProofStatus status;
        bytes32 blobHash;
        bytes32 merkleRoot;
    }

    /// @notice Method to verify an aggregated proof from aligned
    /// @dev This function is called by the aligned proof aggregator after collecting the proofs and aggregating them
    /// to be verified on-chain. We expect the blobTransactionHash to be called before
    /// @param blobVersionedHash the versioned hash of the blob transaction that contains the leaves that compose the merkle root.
    /// param sp1ProgramVKey Public verifying key
    /// @param sp1PublicValues Values used to perform the execution
    /// param sp1ProofBytes Groth16 proof
    function verify(
        bytes32 blobVersionedHash,
        //bytes32 sp1ProgramVKey,
        bytes calldata sp1PublicValues
        //bytes calldata sp1ProofBytes
    ) external;

    function getAggregatedProof(uint64 proofNumber)
        external
        view
        returns (uint8 status, bytes32 blobHash, bytes32 merkleRoot);

    function markCurrentAggregatedProofAsMissed() external;

    /// @notice event that gets emitted after a successful aggregated proof verification
    event NewAggregatedProof(
        uint64 indexed proofNumber, AggregatedProofStatus status, bytes32 merkleRoot, bytes32 blobVersionedHash
    );

    error OnlyAlignedAggregator(address sender);
}
