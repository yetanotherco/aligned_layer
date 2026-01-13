// Taken from https://github.com/0xPolygonHermez/zisk/blob/feature/bn128/zisk-contracts/ZiskVerifier.sol

// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.20;

import {IZiskVerifier} from "./IZiskVerifier.sol";
import {PlonkVerifier} from "./PlonkVerifier.sol";

/// @title Zisk Verifier
/// @author SilentSig
/// @notice This contracts implements a solidity verifier for Zisk.
contract ZiskVerifier is PlonkVerifier, IZiskVerifier {
    /// @notice Thrown when the verifier selector from this proof does not match the one in this
    /// verifier. This indicates that this proof was sent to the wrong verifier.
    /// @notice Thrown when the proof is invalid.
    error InvalidProof();

    function VERSION() external pure returns (string memory) {
        return "v0.15.0";
    }

    // Modulus zkSNARK
    uint256 internal constant _RFIELD = 21888242871839275222246405745257275088548364400416034343698204186575808495617;

    /// @notice Hashes the public values to a field elements inside Bn254.
    /// @param publicValues The public values.
    function hashPublicValues(uint64[4] calldata programVK, bytes calldata publicValues) public pure returns (uint256) {
        return uint256(
            sha256(
            abi.encodePacked(
            bytes8(programVK[0]), bytes8(programVK[1]), bytes8(programVK[2]), bytes8(programVK[3]), publicValues
        )
        )
        ) % _RFIELD;
    }

    /// @notice Verifies a proof with given public values and vkey.
    /// @param programVK The verification key for the RISC-V program.
    /// @param publicValues The public values encoded as bytes.
    /// @param proofBytes The proof of the program execution the Zisk zkVM encoded as bytes.
    function verifySnarkProof(uint64[4] calldata programVK, bytes calldata publicValues, bytes calldata proofBytes)
        external
        view
    {
        uint256 publicValuesDigest = hashPublicValues(programVK, publicValues);

        uint256[24] memory proofDecoded = abi.decode(proofBytes, (uint256[24]));

        bool success = this.verifyProof(proofDecoded, [publicValuesDigest]);

        if (!success) {
            revert InvalidProof();
        }
    }
}
