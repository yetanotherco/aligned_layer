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

// cast call 0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8 "verifyBatchInclusion(bytes32, bytes32, bytes32, bytes20, bytes32, bytes, uint256)(bool)" d699e2ba4634b238b15f1a9bde4f8c32109bf7fd94d47fcc8948386825f1413e 001efe45a8ea06db02146bbff933ac3dd1f43224f43a3ca92865af2b54fd5bb2 bb7e14296ccb07ebb5f26679676ef1b2de55ae8e65e094d91cdb3e16ce6fa85f f39fd6e51aad88f6f4ce6ab8827279cfffb92266 19f04bbb143af72105e2287935c320cc2aa9eeda0fe1f3ffabbe4e59cdbab691 7223eca0c5fc28a3c1c89be2d634ca4ef5340db6966a7c952e354f4ea66999c8681fcf9f64e7451a7f250e7945cef644ed52f6efccaec1a3294595405042009c 0