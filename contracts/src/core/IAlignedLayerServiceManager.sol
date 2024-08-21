// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.8.12;

import {IBLSSignatureChecker} from "eigenlayer-middleware/interfaces/IBLSSignatureChecker.sol";

interface IAlignedLayerServiceManager {
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer
    ) external payable;

    function respondToTask(
        bytes32 batchMerkleRoot,
        IBLSSignatureChecker.NonSignerStakesAndSignature
            memory nonSignerStakesAndSignature
    ) external;

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) external view returns (bool);

    function balanceOf(address account) external view returns (uint256);
}
