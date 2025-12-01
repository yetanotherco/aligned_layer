pragma solidity ^0.8.12;

interface IAlignedProofAggregationService {
    /// @notice event that gets emitted after a successful aggregated proof verification
    event AggregatedProofVerified(bytes32 indexed merkleRoot, bytes32 blobVersionedHash);

    /// @notice Event emitted when the Risc0 verifier address is updated
    event Risc0VerifierAddressUpdated(address indexed newAddress);

    /// @notice Event emitted when the SP1 verifier address is updated
    event SP1VerifierAddressUpdated(address indexed newAddress);
    
    /// @notice Event emitted when a verifier program is allowed
    event VerifierProgramAllowed(bytes32 indexed verifierProgramCommitment, uint8 provingSystemId);

    /// @notice Event emitted when a verifier program is disallowed
    event VerifierProgramDisallowed(bytes32 indexed verifierProgramCommitment, uint8 provingSystemId);

    /// @notice Method to verify an aggregated proof from aligned
    /// @dev This function is called by the aligned proof aggregator after collecting the proofs and aggregating them
    /// to be verified on-chain. We expect the blobTransactionHash to be called before
    /// @param blobVersionedHash the versioned hash of the blob transaction that contains the leaves that compose the merkle root.
    /// @param sp1PublicValues Values used to perform the execution
    /// @param sp1ProofBytes Groth16 proof
    /// @param verifierProgramCommitment The chunk aggregator verifier program commitment against which the proof should be verified
    function verifyAggregationSP1(bytes32 blobVersionedHash, bytes calldata sp1PublicValues, bytes calldata sp1ProofBytes, bytes32 verifierProgramCommitment)
        external;

    function verifyAggregationRisc0(bytes32 blobVersionedHash, bytes calldata risc0ReceiptSeal, bytes calldata risc0JournalBytes, bytes32 verifierProgramCommitment)
        external;

    function isProofVerified(
        bytes32[] calldata merklePath,
        uint16 provingSystemId,
        bytes32 programCommitment,
        bytes calldata publicInputs
    ) external view returns (bool);

    /// @notice Sets the address of the Risc0 verifier contract
    /// @param _risc0VerifierAddress The new address for the Risc0 verifier contract
    function setRisc0VerifierAddress(address _risc0VerifierAddress) external;

    /// @notice Sets the address of the SP1 verifier contract
    /// @param _sp1VerifierAddress The new address for the SP1 verifier contract
    function setSP1VerifierAddress(address _sp1VerifierAddress) external;

    /// @notice Allows a new verifier program commitment with its proving system ID
    /// @param verifierProgramCommitment The verifier program commitment to allow
    /// @param provingSystemId The proving system ID (1 for SP1, 2 for RISC0)
    function allowVerifyingProgram(bytes32 verifierProgramCommitment, uint8 provingSystemId) external;

    /// @notice Disallows an existing verifier program commitment
    /// @param verifierProgramCommitment The verifier program commitment to disallow
    /// @param provingSystemId The proving system ID (1 for SP1, 2 for RISC0)
    function disallowVerifyingProgram(bytes32 verifierProgramCommitment, uint8 provingSystemId) external;

    error OnlyAlignedAggregator(address sender);

    error InvalidVerifyingProgram(bytes32 verifierProgramCommitment, uint8 expected, uint8 actual);

    error InvalidProvingSystemId(uint8 actual);

    error ProvingSystemIdMismatch(uint8 expected, uint8 received);
}
