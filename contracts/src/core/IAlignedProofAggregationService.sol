pragma solidity ^0.8.12;

interface IAlignedProofAggregationService {
    /// @notice Method to verify an aggregated proof from aligned
    /// @dev This function is called by the aligned proof aggregator after collecting the proofs and aggregating them
    /// to be verified on-chain. We expect the blobTransactionHash to be called before
    /// @param blobVersionedHash the versioned hash of the blob transaction that contains the leaves that compose the merkle root.
    /// @param sp1PublicValues Values used to perform the execution
    /// @param sp1ProofBytes Groth16 proof
    function verifySP1(bytes32 blobVersionedHash, bytes calldata sp1PublicValues, bytes calldata sp1ProofBytes)
        external;

    function verifyRisc0(bytes32 blobVersionedHash, bytes calldata risc0ReceiptSeal, bytes calldata risc0JournalBytes)
        external;

    /// @notice event that gets emitted after a successful aggregated proof verification
    event AggregatedProofVerified(bytes32 indexed merkleRoot, bytes32 blobVersionedHash);

    error OnlyAlignedAggregator(address sender);
}
