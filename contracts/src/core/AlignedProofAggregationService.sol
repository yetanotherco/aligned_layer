// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IAlignedProofAggregationService} from "./IAlignedProofAggregationService.sol";
import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";
import {IRiscZeroVerifier} from "@risc0-contracts/IRiscZeroVerifier.sol";
import {IZiskVerifier} from "../zisk/IZiskVerifier.sol";
import {MerkleProof} from "../../lib/openzeppelin-contracts/contracts/utils/cryptography/MerkleProof.sol";

contract AlignedProofAggregationService is
    IAlignedProofAggregationService,
    Initializable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    /// @notice true if merkle root is verified
    mapping(bytes32 => bool) public isMerkleRootVerified;

    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://docs.succinct.xyz/docs/sp1/verification/contract-addresses
    address public sp1VerifierAddress;

    /// @notice The address of the Wallet that is allowed to call the verify function.
    address public alignedAggregatorAddress;

    /// @notice The address of the Risc0 verifier contract
    /// @dev See supported verifier here:
    /// https://dev.risczero.com/api/blockchain-integration/contracts/verifier#contract-addresses
    address public risc0VerifierAddress;

    /// @notice The address of the Zisk verifier contract
    address public ziskVerifierAddress;

    /// @notice Proving system ID for SP1
    uint8 public constant SP1_ID = 1;

    /// @notice Proving system ID for RISC0
    uint8 public constant RISC0_ID = 2;

    /// @notice Proving system ID for ZISK
    uint8 public constant ZISK_ID = 3;

    /// @notice Maps allowed verifiers commitments to their proving system. If the verifier is not a valid one, it returns 0 and is considered invalid
    mapping(bytes32 => uint8) public allowedVerifiersProvingSystem;

    constructor() {
        _disableInitializers();
    }

    function initialize(
        address newOwner,
        address _alignedAggregatorAddress,
        address _sp1VerifierAddress,
        address _risc0VerifierAddress,
        address _ziskVerifierAddress,
        bytes32 _risc0AggregatorProgramImageId,
        bytes32 _sp1AggregatorProgramVKHash,
        bytes32 _ziskAggregatorProgramVKHash
    ) public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        _transferOwnership(newOwner);
        alignedAggregatorAddress = _alignedAggregatorAddress;
        sp1VerifierAddress = _sp1VerifierAddress;
        risc0VerifierAddress = _risc0VerifierAddress;
        ziskVerifierAddress = _ziskVerifierAddress;
        allowedVerifiersProvingSystem[_risc0AggregatorProgramImageId] = RISC0_ID;
        allowedVerifiersProvingSystem[_sp1AggregatorProgramVKHash] = SP1_ID;
        allowedVerifiersProvingSystem[_ziskAggregatorProgramVKHash] = ZISK_ID;
    }

    function verifyAggregationSP1(bytes32 blobVersionedHash, bytes calldata sp1PublicValues, bytes calldata sp1ProofBytes, bytes32 verifierProgramCommitment)
        public
        onlyAlignedAggregator
    {
        (bytes32 merkleRoot) = abi.decode(sp1PublicValues, (bytes32));

        if (allowedVerifiersProvingSystem[verifierProgramCommitment] != SP1_ID) {
            revert InvalidVerifyingProgram(verifierProgramCommitment, SP1_ID, allowedVerifiersProvingSystem[verifierProgramCommitment]);
        }

        ISP1Verifier(sp1VerifierAddress).verifyProof(verifierProgramCommitment, sp1PublicValues, sp1ProofBytes);

        isMerkleRootVerified[merkleRoot] = true;
        emit AggregatedProofVerified(merkleRoot, blobVersionedHash);
    }

    function verifyAggregationRisc0(bytes32 blobVersionedHash, bytes calldata risc0ReceiptSeal, bytes calldata risc0JournalBytes, bytes32 verifierProgramCommitment)
        public
        onlyAlignedAggregator
    {
        (bytes32 merkleRoot) = abi.decode(risc0JournalBytes, (bytes32));

        if (allowedVerifiersProvingSystem[verifierProgramCommitment] != RISC0_ID) {
            revert InvalidVerifyingProgram(verifierProgramCommitment, RISC0_ID, allowedVerifiersProvingSystem[verifierProgramCommitment]);
        }

        bytes32 risc0JournalDigest = sha256(risc0JournalBytes);
        IRiscZeroVerifier(risc0VerifierAddress).verify(
            risc0ReceiptSeal, verifierProgramCommitment, risc0JournalDigest
        );

        isMerkleRootVerified[merkleRoot] = true;
        emit AggregatedProofVerified(merkleRoot, blobVersionedHash);
    }

    function verifyAggregationZisk(bytes32 blobVersionedHash, bytes calldata publicValues, bytes calldata proofBytes, bytes32 verifierProgramCommitment)
        public
        onlyAlignedAggregator
    {
        (bytes32 merkleRoot) = abi.decode(publicValues, (bytes32));

        if (allowedVerifiersProvingSystem[verifierProgramCommitment] != ZISK_ID) {
            revert InvalidVerifyingProgram(verifierProgramCommitment, ZISK_ID, allowedVerifiersProvingSystem[verifierProgramCommitment]);
        }

        // Cast verifierProgramCommitment (bytes32) to uint64[4]
        uint256 commitment = uint256(verifierProgramCommitment);
        uint64[4] memory programVK = [
            uint64(commitment >> 192),
            uint64(commitment >> 128),
            uint64(commitment >> 64),
            uint64(commitment)
        ];

        IZiskVerifier(ziskVerifierAddress).verifySnarkProof(programVK, publicValues, proofBytes);

        isMerkleRootVerified[merkleRoot] = true;
        emit AggregatedProofVerified(merkleRoot, blobVersionedHash);
    }

    /// @notice Verifies the inclusion of proof in an aggregated proof via Merkle tree proof.
    ///
    /// @dev
    /// - The `programCommitment` parameter represents the unique identifier for the vm program:
    ///   - In RISC Zero, this corresponds to the `image_id`.
    ///   - In SP1, this corresponds to the `vk` (verification key) hash.
    /// - The proof commitment is derived by hashing together the `programCommitment` and the `publicInputs`.
    /// - The `merklePath` is then used to compute the Merkle root from this commitment.
    /// - The function returns `true` if this Merkle root is known to correspond to a valid aggregated proof.
    ///
    /// @param merklePath The Merkle proof (sibling hashes) needed to reconstruct the Merkle root.
    /// @param provingSystemId The id of the proving system (1 for SP1, 2 for RISC0).
    /// @param programCommitment The commitment of the program sent to Aligned (image_id in RISC0 or vk hash in SP1).
    /// @param publicInputs The public inputs bytes of the proof sent to Aligned.
    ///
    /// @return bool Returns true if the computed Merkle root is a recognized valid aggregated proof.
    function isProofVerified(
        bytes32[] calldata merklePath,
        uint16 provingSystemId,
        bytes32 programCommitment,
        bytes calldata publicInputs
    ) public view returns (bool) {
        bytes32 proofCommitment = keccak256(abi.encodePacked(provingSystemId, programCommitment, publicInputs));
        bytes32 merkleRoot = MerkleProof.processProofCalldata(merklePath, proofCommitment);
        return isMerkleRootVerified[merkleRoot];
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyOwner // solhint-disable-next-line no-empty-blocks
    {}

    modifier onlyAlignedAggregator() {
        if (msg.sender != alignedAggregatorAddress) {
            revert OnlyAlignedAggregator(msg.sender);
        }
        _;
    }

    /// @notice Modifier to ensure the provided proving system ID is one of the valid values.
    modifier onValidProvingSystemId(uint8 provingSystemId) {
        if (provingSystemId != SP1_ID &&
            provingSystemId != RISC0_ID &&
            provingSystemId != ZISK_ID){
                revert IAlignedProofAggregationService.InvalidProvingSystemId(provingSystemId);
            }

        _;
    }

    /// @notice Sets the address of the Risc0 verifier contract
    /// @param _risc0VerifierAddress The new address for the Risc0 verifier contract
    function setRisc0VerifierAddress(address _risc0VerifierAddress) external onlyOwner {
        risc0VerifierAddress = _risc0VerifierAddress;
        emit Risc0VerifierAddressUpdated(_risc0VerifierAddress);
    }

    /// @notice Sets the address of the SP1 verifier contract
    /// @param _sp1VerifierAddress The new address for the SP1 verifier contract
    function setSP1VerifierAddress(address _sp1VerifierAddress) external onlyOwner {
        sp1VerifierAddress = _sp1VerifierAddress;
        emit SP1VerifierAddressUpdated(_sp1VerifierAddress);
    }

    /// @notice Sets the address of the Zisk verifier contract
    /// @param _ziskVerifierAddress The new address for the Zisk verifier contract
    function setZiskVerifierAddress(address _ziskVerifierAddress) external onlyOwner {
        ziskVerifierAddress = _ziskVerifierAddress;
        emit ZiskVerifierAddressUpdated(_ziskVerifierAddress);
    }

    /// @notice Allows a new verifying program commitment to the list of valid verifying programs.
    /// @param verifierProgramCommitment The verifying program commitment to allow (image ID for RISC0 or vk hash for SP1).
    /// @param provingSystemId The proving system ID associated with the verifying program.
    function allowVerifyingProgram(bytes32 verifierProgramCommitment, uint8 provingSystemId)
        external
        onlyOwner
        onValidProvingSystemId(provingSystemId)
    {
        allowedVerifiersProvingSystem[verifierProgramCommitment] = provingSystemId;
        emit VerifierProgramAllowed(verifierProgramCommitment, provingSystemId);
    }

    /// @notice Disallows a verifying program commitment from the list of valid verifying programs.
    /// @param verifierProgramCommitment The verifying program commitment to disallow (image ID for RISC0 or vk hash for SP1).
    function disallowVerifyingProgram(bytes32 verifierProgramCommitment, uint8 provingSystemId) external onlyOwner onValidProvingSystemId(provingSystemId) {
        // Preserve the proving system ID so we can emit it with the event
        uint8 provingSystemIdRaw = allowedVerifiersProvingSystem[verifierProgramCommitment];

        // Check if the obtained proving system ID matches the one received by param
        if (provingSystemIdRaw != provingSystemId) {
            revert IAlignedProofAggregationService.ProvingSystemIdMismatch(provingSystemIdRaw, provingSystemId);
        }

        delete allowedVerifiersProvingSystem[verifierProgramCommitment];
        emit VerifierProgramDisallowed(verifierProgramCommitment, provingSystemIdRaw);
    }
}
