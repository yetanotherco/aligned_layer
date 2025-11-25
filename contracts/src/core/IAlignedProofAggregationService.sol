pragma solidity ^0.8.12;

interface IAlignedProofAggregationService {

    /// @notice event that gets emitted after a successful aggregated proof verification
    event AggregatedProofVerified(bytes32 indexed merkleRoot, bytes32 blobVersionedHash);

    /// @notice Event emitted when the Risc0 verifier address is updated
    event Risc0VerifierAddressUpdated(address indexed newAddress);
    
    /// @notice Event emitted when the SP1 verifier address is updated
    event SP1VerifierAddressUpdated(address indexed newAddress);
    
    /// @notice Event emitted when a new program ID is added
    event ProgramIdAdded(bytes32 indexed programId, VerifierType verifierType);

    /// @notice Event emitted when a program ID is deleted
    event ProgramIdDeleted(bytes32 indexed programId, VerifierType verifierType);

    /// @notice Method to verify an aggregated proof from aligned
    /// @dev This function is called by the aligned proof aggregator after collecting the proofs and aggregating them
    /// to be verified on-chain. We expect the blobTransactionHash to be called before
    /// @param blobVersionedHash the versioned hash of the blob transaction that contains the leaves that compose the merkle root.
    /// @param sp1PublicValues Values used to perform the execution
    /// @param sp1ProofBytes Groth16 proof
    /// @param programId The chunk aggregator program ID against which the proof should be verified
    function verifySP1(bytes32 blobVersionedHash, bytes calldata sp1PublicValues, bytes calldata sp1ProofBytes, bytes32 programId)
        external;

    function verifyRisc0(bytes32 blobVersionedHash, bytes calldata risc0ReceiptSeal, bytes calldata risc0JournalBytes, bytes32 programId)
        external;

    function verifyProofInclusion(bytes32[] calldata merklePath, bytes32 programId, bytes calldata publicInputs)
        external
        view
        returns (bool);

    /// @notice Sets the address of the Risc0 verifier contract
    /// @param _risc0VerifierAddress The new address for the Risc0 verifier contract
    function setRisc0VerifierAddress(address _risc0VerifierAddress) external;

    /// @notice Sets the address of the SP1 verifier contract
    /// @param _sp1VerifierAddress The new address for the SP1 verifier contract
    function setSP1VerifierAddress(address _sp1VerifierAddress) external;

    /// @notice Adds a new program ID with its verifier type
    /// @param programId The program ID to add
    /// @param verifierType The type of verifier (SP1 or RISC0)
    function addProgramId(bytes32 programId, VerifierType verifierType) external;

    /// @notice Deletes an existing program ID
    /// @param programId The program ID to delete
    function deleteProgramId(bytes32 programId) external;

    error OnlyAlignedAggregator(address sender);

    error InvalidProgramId(bytes32 programId, VerifierType expected, uint8 actual);

    enum VerifierType {
        INVALID,  // If a given program does not exist in the `programId` map, it defaults to 0. This prevents non-existing keys to be considered valid in case SP1 or RISC0 were in this position
        SP1,
        RISC0
    }
}
