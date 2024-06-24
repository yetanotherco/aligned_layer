// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

contract VerifierContract is ERC721 {
    uint256 private _nextTokenId;

    address public alignedServiceManager;

    bytes32 public elfCommitment = 0x875acd4ee99557f7a29004fc8b08ca6eb0cee2cef8f151061c71fe1d6418fe50;

    // map to check if proof has already been submitted
    mapping(bytes32 => bool) public mintedProofs;

    constructor(address _alignedServiceManager) ERC721("Aligned Quiz Solved", "AZKQ") {
        alignedServiceManager = _alignedServiceManager;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) external returns (uint256) {
        require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");
        require(address(proofGeneratorAddr) == msg.sender, "proofGeneratorAddr does not match");

        bytes32 fullHash = keccak256(abi.encodePacked(proofCommitment,
            pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr));
        require(!mintedProofs[fullHash], "proof already minted");

        (bool callWasSuccessfull, bytes memory proofIsIncluded) = alignedServiceManager.staticcall(
            abi.encodeWithSignature(
                "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)",
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr,
                batchMerkleRoot,
                merkleProof,
                verificationDataBatchIndex
            )
        );

        require(callWasSuccessfull, "static_call failed");

        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));
        require(proofIsIncludedBool, "proof not included in batch");

        mintedProofs[fullHash] = true;

        uint256 tokenId = _nextTokenId++;
        _mint(msg.sender, tokenId);

        return tokenId;
    }
}
