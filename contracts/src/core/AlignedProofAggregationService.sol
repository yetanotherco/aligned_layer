// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IAlignedProofAggregationService} from "./IAlignedProofAggregationService.sol";
import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";
import {IRiscZeroVerifier} from "@risc0-contracts/IRiscZeroVerifier.sol";
import {MerkleProof} from "../../lib/openzeppelin-contracts/contracts/utils/cryptography/MerkleProof.sol";

contract AlignedProofAggregationService is
    IAlignedProofAggregationService,
    Initializable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    /// @notice Map the merkle root to a boolean to indicate it was verified
    mapping(bytes32 => bool) public aggregatedProofs;

    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://docs.succinct.xyz/onchain-verification/contract-addresses
    address public sp1VerifierAddress;

    /// @notice The address of the Wallet that is allowed to call the verify function.
    address public alignedAggregatorAddress;

    /// @notice The address of the Risc0 verifier contract
    /// @dev See supported verifier here:
    /// https://dev.risczero.com/api/blockchain-integration/contracts/verifier#contract-addresses
    address public risc0VerifierAddress;

    /// @notice whether we are in dev mode or not
    /// if the sp1 verifier address is set to this address, then we skip verification
    address public constant VERIFIER_MOCK_ADDRESS = address(0xFF);

    constructor() {
        _disableInitializers();
    }

    function initialize(
        address newOwner,
        address _alignedAggregatorAddress,
        address _sp1VerifierAddress,
        address _risc0VerifierAddress
    ) public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        _transferOwnership(newOwner);
        alignedAggregatorAddress = _alignedAggregatorAddress;
        sp1VerifierAddress = _sp1VerifierAddress;
        risc0VerifierAddress = _risc0VerifierAddress;
    }

    function verifySP1(
        bytes32 blobVersionedHash,
        bytes32 sp1ProgramVKey,
        bytes calldata sp1PublicValues,
        bytes calldata sp1ProofBytes
    ) public onlyAlignedAggregator {
        (bytes32 merkleRoot) = abi.decode(sp1PublicValues, (bytes32));

        // In dev mode, poofs are mocked, so we skip the verification part
        if (_isSP1VerificationEnabled()) {
            ISP1Verifier(sp1VerifierAddress).verifyProof(sp1ProgramVKey, sp1PublicValues, sp1ProofBytes);
        }

        aggregatedProofs[merkleRoot] = true;
        emit AggregatedProofVerified(merkleRoot, blobVersionedHash);
    }

    function verifyRisc0(
        bytes32 blobVersionedHash,
        bytes calldata risc0ReceiptSeal,
        bytes32 risc0ImageId,
        bytes calldata risc0JournalBytes
    ) public onlyAlignedAggregator {
        (bytes32 merkleRoot) = abi.decode(risc0JournalBytes, (bytes32));

        // In dev mode, poofs are mocked, so we skip the verification part
        if (_isRisc0VerificationEnabled()) {
            bytes32 risc0JournalDigest = sha256(risc0JournalBytes);
            IRiscZeroVerifier(risc0VerifierAddress).verify(risc0ReceiptSeal, risc0ImageId, risc0JournalDigest);
        }

        aggregatedProofs[merkleRoot] = true;
        emit AggregatedProofVerified(merkleRoot, blobVersionedHash);
    }

    /// @notice Verifies the inclusion of proof in an aggregated proof via Merkle tree proof.
    ///
    /// @dev
    /// - The `programId` parameter represents the unique identifier for the vm program:
    ///   - In RISC Zero, this corresponds to the `image_id`.
    ///   - In SP1, this corresponds to the `vk` (verification key) hash.
    /// - The proof commitment is derived by hashing together the `programId` and the `publicInputs`.
    /// - The `merklePath` is then used to compute the Merkle root from this commitment.
    /// - The function returns `true` if this Merkle root is known to correspond to a valid aggregated proof.
    ///
    /// @param merklePath The Merkle proof (sibling hashes) needed to reconstruct the Merkle root.
    /// @param programId The identifier for the ZK program (image_id in RISC0 or vk hash in SP1).
    /// @param publicInputs The public inputs bytes of the proof.
    ///
    /// @return bool Returns true if the computed Merkle root is a recognized valid aggregated proof.
    function verifyProofInclusion(bytes32[] calldata merklePath, bytes32 programId, bytes calldata publicInputs)
        public
        view
        returns (bool)
    {
        bytes32 proofCommitment = keccak256(abi.encodePacked(programId, publicInputs));
        bytes32 merkleRoot = MerkleProof.processProofCalldata(merklePath, proofCommitment);
        return aggregatedProofs[merkleRoot];
    }

    function _isSP1VerificationEnabled() internal view returns (bool) {
        return sp1VerifierAddress != VERIFIER_MOCK_ADDRESS;
    }

    function _isRisc0VerificationEnabled() internal view returns (bool) {
        return risc0VerifierAddress != VERIFIER_MOCK_ADDRESS;
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

    /// @notice Sets the address of the Risc0 verifier contract
    /// @param _risc0VerifierAddress The new address for the Risc0 verifier contract
    function setRisc0VerifierAddress(address _risc0VerifierAddress) external onlyOwner {
        risc0VerifierAddress = _risc0VerifierAddress;
    }
}
