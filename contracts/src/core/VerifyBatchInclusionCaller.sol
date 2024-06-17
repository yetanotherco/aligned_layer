// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

contract VerifyBatchInclusionCaller {
    address public targetContract;

    constructor(address _targetContract) {
        targetContract = _targetContract;
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
        (bool callWasSuccessfull, bytes memory proofIsIncluded) = targetContract.staticcall(
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

        return abi.decode(proofIsIncluded, (bool));
    }
}
