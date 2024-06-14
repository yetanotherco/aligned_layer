// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

interface IAlignedLayerServiceManager {
    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint verificationDataBatchIndex
    ) external returns (bool);
}

contract VerifyBatchInclusionCaller {
    address public targetContract;

    constructor(address _targetContract) {
        targetContract = _targetContract;
    }

    function staticCallGetValue(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint verificationDataBatchIndex
    ) external view returns (bool) {
        (bool success, bytes memory returnData) = targetContract.staticcall(
            abi.encodeWithSignature(
                "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint)",
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr,
                batchMerkleRoot,
                merkleProof,
                verificationDataBatchIndex
            )
        );

        require(success, "static_call failed");

        return abi.decode(returnData, (bool));
    }
}
