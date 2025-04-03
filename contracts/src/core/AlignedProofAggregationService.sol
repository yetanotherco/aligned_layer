// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Initializable} from "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import {OwnableUpgradeable} from "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin-upgrades/contracts/proxy/utils/UUPSUpgradeable.sol";
import {IAlignedProofAggregationService} from "./IAlignedProofAggregationService.sol";

/// Template for contract, SP1 verifiers needs to be added
contract AlignedProofAggregationService is
    IAlignedProofAggregationService,
    Initializable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    /// @notice Maps the aggregated verification merkle root with the blob transaction hash that contains the leaves
    uint64 public currentAggregatedProofNumber;
    mapping(uint64 => AggregatedProof) public aggregatedProofs;

    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://docs.succinct.xyz/onchain-verification/contract-addresses
    address public sp1VerifierAddress;

    address public alignedAggregatorAddress;

    /// @notice whether we are in dev mode or not
    /// if the sp1 verifier address is set to this address, then we skip verification
    address public constant VERIFIER_MOCK_ADDRESS = address(0xFF);

    constructor() {
        _disableInitializers();
    }

    function initialize(address newOwner, address _alignedAggregatorAddress, address _sp1VerifierAddress)
        public
        initializer
    {
        __Ownable_init();
        __UUPSUpgradeable_init();
        _transferOwnership(newOwner);
        alignedAggregatorAddress = _alignedAggregatorAddress;
        sp1VerifierAddress = _sp1VerifierAddress;
    }

    function verify(
        bytes32 blobVersionedHash,
        //bytes32 sp1ProgramVKey,
        bytes calldata sp1PublicValues
        //bytes calldata sp1ProofBytes
    ) public onlyAlignedAggregator {
        // In dev mode, poofs are mocked, so we skip the verification part
        if (sp1VerifierAddress == VERIFIER_MOCK_ADDRESS) {
            (bytes32 merkleRoot) = abi.decode(sp1PublicValues, (bytes32));
            _newAggregatedProof(merkleRoot, blobVersionedHash);
            return;
        }

    }

    function markCurrentAggregatedProofAsMissed() public onlyAlignedAggregator {
        AggregatedProof storage proof = aggregatedProofs[currentAggregatedProofNumber];
        proof.status = AggregatedProofStatus.Missed;
        emit NewAggregatedProof(currentAggregatedProofNumber, AggregatedProofStatus.Missed, 0x0, 0x0);
        currentAggregatedProofNumber += 1;
    }

    function _newAggregatedProof(bytes32 merkleRoot, bytes32 blobHash) internal {
        AggregatedProof storage proof = aggregatedProofs[currentAggregatedProofNumber];
        proof.merkleRoot = merkleRoot;
        proof.blobHash = blobHash;
        proof.status = AggregatedProofStatus.Verified;
        emit NewAggregatedProof(currentAggregatedProofNumber, AggregatedProofStatus.Verified, merkleRoot, blobHash);
        currentAggregatedProofNumber += 1;
    }

    function getAggregatedProof(uint64 proofNumber) public view returns (uint8, bytes32 blobHash, bytes32 merkleRoot) {
        AggregatedProof storage proof = aggregatedProofs[proofNumber];
        return (uint8(proof.status), proof.blobHash, proof.merkleRoot);
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
}
