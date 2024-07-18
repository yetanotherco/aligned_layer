# Validating public input

You can validate that the public input of the proof sent to Aligned for verification is correct in a few simple steps.

This guide demonstrates the submission of a Risc0 proof to Aligned using the Aligned SDK. The Risc0 program to be proven is a Fibonacci sequence calculator. Risc0 generates a public input corresponding to the last two Fibonacci numbers of the sequence taken modulo 7919, and we want to validate in a smart contract that the public input commitments correspond to those two numbers.

In this case, the Fibonacci number to be calculated is **500** and the last two numbers of the sequence modulo 7919 are **1268** and **1926**.

This guide assumes you are in the `examples/validating-public-input` directory.

## Generate your ZK Proof

To submit proofs to Aligned and get them verified, first you need to generate those proofs. Every proving system has its own way of generating proofs.

You can find examples on how to generate proofs in the [generating proofs guide](4_generating_proofs.md).

To generate the proof needed to try this example, run `make generate_risc_zero_fibonacci_proof`.

## Write your smart contract

To check if a proof was verified in Aligned, you need to make a call to the `AlignedServiceManager` contract inside your smart contract.

Also, you will need a way to check that the proven program is the correct one.

The Aligned CLI provides a way for you to get the verification key commitment without actually generating and submitting a proof.

You can do this by running the following command:

```bash
aligned get-commitment --input <path_to_input_file>
```

The following is an example of how to validate the public input of the Risc0 proof in your smart contract.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

contract FibonacciValidator {
    address public alignedServiceManager;
    bytes32 public fibonacciImageId;

    bytes32 public fibonacciImageIdCommitment =
        0xbfa561e384be753bd6fd75b15db31eb511cd114ec76d619a87c2342af0ee1ed7;

    event FibonacciNumbers(uint32 fibN, uint32 fibNPlusOne);

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
        uint256 verificationDataBatchIndex,
        bytes memory journalBytes
    ) public returns (bool) {
        require(
            fibonacciImageIdCommitment == provingSystemAuxDataCommitment,
            "Image ID doesn't match"
        );

        require(
            pubInputCommitment == keccak256(abi.encodePacked(journalBytes)),
            "Fibonacci numbers don't match with public input"
        );

        (uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(journalBytes);

        emit FibonacciNumbers(fibN, fibNPlusOne);

        (
            bool callWasSuccessful,
            bytes memory proofIsIncluded
        ) = alignedServiceManager.staticcall(
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

        require(callWasSuccessful, "static_call failed");

        return abi.decode(proofIsIncluded, (bool));
    }

    function bytesToTwoUint32(
        bytes memory data
    ) public pure returns (uint32, uint32) {
        require(data.length >= 8, "Input bytes must be at least 8 bytes long");

        uint32 first = uint32(uint8(data[0])) |
            (uint32(uint8(data[1])) << 8) |
            (uint32(uint8(data[2])) << 16) |
            (uint32(uint8(data[3])) << 24);

        uint32 second = uint32(uint8(data[4])) |
            (uint32(uint8(data[5])) << 8) |
            (uint32(uint8(data[6])) << 16) |
            (uint32(uint8(data[7])) << 24);

        return (first, second);
    }
}
```

### Explanation

1. **Verification Key Check:** The contract first checks if the verification key commitment matches the Fibonacci Image ID commitment.

```solidity
require(
            fibonacciImageIdCommitment == provingSystemAuxDataCommitment,
            "Image ID doesn't match"
        );
```

2. **Public Input Commitment Validation:** The contract validates that the public input commitment matches the keccak256 hash of the journalBytes. It then extracts the last two Fibonacci numbers from the journalBytes and emits an event.

```solidity
require(
    pubInputCommitment == keccak256(abi.encodePacked(journalBytes)),
    "Fibonacci numbers don't match with public input"
);

(uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(journalBytes);

emit FibonacciNumbers(fibN, fibNPlusOne);
```

3. **Static Call to AlignedServiceManager**: The contract makes a static call to the `AlignedServiceManager` contract to check if the proof was verified in Aligned.

```solidity
(
    bool callWasSuccessful,
    bytes memory proofIsIncluded
) = alignedServiceManager.staticcall(
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

require(callWasSuccessful, "static_call failed");
```

4. **Bytes to two `uint32` conversion:** A helper function to convert a byte array into two `uint32` numbers, used for extracting the last two Fibonacci numbers from the `journalBytes`.

```solidity
function bytesToTwoUint32(
    bytes memory data
) public pure returns (uint32, uint32) {
    require(data.length >= 8, "Input bytes must be at least 8 bytes long");

    uint32 first = uint32(uint8(data[0])) |
        (uint32(uint8(data[1])) << 8) |
        (uint32(uint8(data[2])) << 16) |
        (uint32(uint8(data[3])) << 24);

    uint32 second = uint32(uint8(data[4])) |
        (uint32(uint8(data[5])) << 8) |
        (uint32(uint8(data[6])) << 16) |
        (uint32(uint8(data[7])) << 24);

    return (first, second);
}
```

To deploy the contract, first you will need to set up the `.env` file in the contracts folder with the following variables:

```
RPC_URL= #You can use publicnode RPC: https://ethereum-holesky-rpc.publicnode.com
PRIVATE_KEY=
ALIGNED_SERVICE_MANAGER_ADDRESS= #0x58F280BeBE9B34c9939C3C39e0890C81f163B623 for Holesky
```

Then, run `make deploy_fibonacci_validator`.

## Submit and verify the proof to Aligned

The proof submission and verification can be done either with the SDK or by using the Aligned CLI.

To submit the proof generated in this example, run `make submit_fibonacci_proof`. This will output the `AlignedVerificationData` needed to send to the `verifyBatchInclusion` method of the contract in the `batch_inclusion_data` directory inside `aligned-integration`.

For more details on submitting proofs, refer to the [submitting proofs guide](0_submitting_proofs.md).
