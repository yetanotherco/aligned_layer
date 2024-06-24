// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

contract VerifierContract {
    address public alignedServiceManager;

    bytes32 public elfCommitment = 0x4b9d9da7c31481ab20cc689580306796871409002bb2c21b56cc4a56ca0cb01b;

    constructor(address _alignedServiceManager) {
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
    ) external view returns (bool) {
        require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");

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

        // TODO: mint NFT if proofIsIncludedBool is true

        return proofIsIncludedBool;
    }
}
