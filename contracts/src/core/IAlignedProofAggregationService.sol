pragma solidity ^0.8.12;

interface IAlignedProofAggregationService {

    /// @notice event that gets emitted after a successful aggregated proof verification
    event AggregatedProofVerified(bytes32 indexed merkleRoot, bytes32 blobVersionedHash);

    /// @notice Event emitted when the Risc0 verifier address is updated
    event Risc0VerifierAddressUpdated(address indexed newAddress);
    
    /// @notice Event emitted when the SP1 verifier address is updated
    event SP1VerifierAddressUpdated(address indexed newAddress);
    
    /// @notice Event emitted when the Risc0 aggregator program image ID is updated
    event Risc0AggregatorProgramImageIdUpdated(bytes32 indexed newImageId);
    
    /// @notice Event emitted when the SP1 aggregator program VK hash is updated
    event SP1AggregatorProgramVKHashUpdated(bytes32 indexed newVKHash);

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

    /// @notice Sets the address of the Risc0 verifier contract
    /// @param _risc0VerifierAddress The new address for the Risc0 verifier contract
    function setRisc0VerifierAddress(address _risc0VerifierAddress) external;

    /// @notice Sets the image id of the Risc0 program
    /// @param _risc0AggregatorProgramImageId The new imageid for the Risc0 aggregator program
    function setRisc0AggregatorProgramImageId(bytes32 _risc0AggregatorProgramImageId) external;

    /// @notice Sets the address of the SP1 verifier contract
    /// @param _sp1VerifierAddress The new address for the SP1 verifier contract
    function setSP1VerifierAddress(address _sp1VerifierAddress) external;

    /// @notice Sets the vk hash of the sp1 program
    /// @param _sp1AggregatorProgramVKHash The new vk hash for the sp1 aggregator program
    function setSP1AggregatorProgramVKHash(bytes32 _sp1AggregatorProgramVKHash) external;

    error OnlyAlignedAggregator(address sender);
}
