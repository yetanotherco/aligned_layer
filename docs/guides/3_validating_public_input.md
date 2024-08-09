# Validating public input

In some applications, it is crucial to ensure that a third party has performed a computation correctly and to make use of the result of that computation. To achieve this, the third party must first interact with Aligned and obtain the `AlignedVerificationData`, a receipt indicating that the proof of the computation was verified correctly. The application should then receive both the `AlignedVerificationData` and the result of the computation. After confirming that the proof was verified by Aligned, it must check that the posted result matches the one committed in the `AlignedVerificationData`.

This guide demonstrates how to validate a Risc0 proof using the Aligned SDK. The Risc0 program in this example is a Fibonacci sequence calculator. It generates a public input that corresponds to the last two Fibonacci numbers of the sequence, taken modulo 7919. Our goal is to validate, within a smart contract, that the public input commitments match these two numbers.

In this case, the Fibonacci number to be calculated is **500** and the last two numbers of the sequence modulo 7919 are **1268** and **1926**.

This guide assumes you are in the `examples/validating-public-input` directory.

## Generate your ZK Proof

> [!IMPORTANT]  
> To generate the proof ensure you have [docker](https://www.docker.com/get-started/) installed and the docker daemon running.
> This is necessary to ensure deterministic builds of the binary we want to generate a proof of. If not used, builds may differ depending on the system you are running on. To know more about this, check [this link](https://dev.risczero.com/terminology#deterministic-builds) from RiscZero docs.

To submit proofs to Aligned and get them verified, first you need to generate those proofs. Every proving system has its own way of generating proofs.

You can find examples on how to generate proofs in the [generating proofs guide](4_generating_proofs.md).

To generate the proof needed to try this example, run `make generate_risc_zero_fibonacci_proof`.

Once finished, you will see the program id, the two last fibonacci numbers of the sequence and the result of the verification like so:

```
Program ID: 0xf000637ed63d26fc664f16666aebf05440ddb7071931240dc49d9bbcfbac304a
a: 1268
b: 1926
Verification result: true
Fibonacci proof, pub input and image ID generated in risc_zero folder
```

## Write your smart contract

To check if a proof was verified in Aligned, you need to make a call to the `AlignedServiceManager` contract inside your smart contract.

The following is an example of how to validate the public input of the Risc0 proof in your smart contract.

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

contract FibonacciValidator {
    address public alignedServiceManager;
    bytes32 public fibonacciProgramId;

    bytes32 public fibonacciProgramIdCommitment =
        0x069ed9f3972550a2901523723f4beb5e240749dcafa30e1623d0778e17d69d70;

    event FibonacciNumbers(uint32 fibN, uint32 fibNPlusOne);

    constructor(address _alignedServiceManager) {
        alignedServiceManager = _alignedServiceManager;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 programIdCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        bytes memory pubInputBytes
    ) public returns (bool) {
        require(
            fibonacciProgramIdCommitment == programIdCommitment,
            "Program ID doesn't match"
        );

        require(
            pubInputCommitment == keccak256(abi.encodePacked(pubInputBytes)),
            "Fibonacci numbers don't match with public input"
        );

        (
            bool callWasSuccessful,
            bytes memory proofIsIncluded
        ) = alignedServiceManager.staticcall(
                abi.encodeWithSignature(
                    "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)",
                    proofCommitment,
                    pubInputCommitment,
                    programIdCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot,
                    merkleProof,
                    verificationDataBatchIndex
                )
            );

        require(callWasSuccessful, "static_call failed");

        (uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(pubInputBytes);

        emit FibonacciNumbers(fibN, fibNPlusOne);

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

1. **Program Identifier Validation:** The contract first validates if the provided commitment of the program identifier matches the expected one.

```solidity
require(
            fibonacciProgramIdCommitment == programIdCommitment,
            "Program ID doesn't match"
);
```

2. **Public Input Validation:** The contract then checks that the commitment of the public inputs matches the keccak 256 hash of the actual public inputs.

```solidity
require(
            pubInputCommitment == keccak256(abi.encodePacked(pubInputBytes)),
            "Fibonacci numbers don't match with public input"
);
```

3. **Static Call to AlignedServiceManager**: The contract makes a static call to the `AlignedServiceManager` contract to check if the proof was verified in Aligned. It then extracts the last two Fibonacci numbers from the pubInputBytes and emits an event.

```solidity
(
    bool callWasSuccessful,
    bytes memory proofIsIncluded
) = alignedServiceManager.staticcall(
    abi.encodeWithSignature(
        "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)",
        proofCommitment,
        pubInputCommitment,
        programIdCommitment,
        proofGeneratorAddr,
        batchMerkleRoot,
        merkleProof,
        verificationDataBatchIndex
        )
);

require(callWasSuccessful, "static_call failed");

(uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(pubInputBytes);

emit FibonacciNumbers(fibN, fibNPlusOne);
```

4. **Bytes to two `uint32` conversion:** A helper function to convert a byte array into two `uint32` numbers, used for extracting the last two Fibonacci numbers from the `pubInputBytes`.

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
RPC_URL=<rpc_url> #You can use publicnode RPC: https://ethereum-holesky-rpc.publicnode.com
PRIVATE_KEY=<private_key>
ALIGNED_SERVICE_MANAGER_ADDRESS=<service_manager_address> #0x58F280BeBE9B34c9939C3C39e0890C81f163B623 for Holesky
```

Then, run `make deploy_fibonacci_validator`.

To call the function in the contract with [cast](https://book.getfoundry.sh/cast/) run:

```
cast send --rpc-url https://ethereum-holesky-rpc.publicnode.com <CONTRACT_ADDRESS> "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,bytes)" <proofCommitment> <pubInputCommitment> <programIdCommitment> <proofGeneratorAddr> <batchMerkleRoot> <merkleProof> <verificationDataBatchIndex> <pubInputBytes> --private-key <PRIVATE_KEY>
```

## Submit and verify the proof to Aligned

The proof submission and verification can be done either with the SDK or by using the Aligned CLI.

To submit the proof generated in this example, run `make submit_fibonacci_proof`. This will output the `AlignedVerificationData` needed to send to the `verifyBatchInclusion` method of the contract in the `batch_inclusion_data` directory inside `aligned-integration`.

For more details on submitting proofs, refer to the [submitting proofs guide](0_submitting_proofs.md).
