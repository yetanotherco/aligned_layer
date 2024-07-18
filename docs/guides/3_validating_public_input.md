# Validating public input

You can validate that the public input of the proof sent to Aligned for verification is correct in a few simple steps.

This guide demonstrates the submission of a Risc0 proof to Aligned using the Aligned SDK. The Risc0 program to be proven is a Fibonacci sequence calculator. Risc0 generates a public input corresponding to the last two Fibonacci numbers of the sequence taken modulo 7919, and we want to validate in a smart contract that the public input commitments correspond to those two numbers.

## Generate your ZK Proof

To submit proofs to Aligned and get them verified, first you need to generate those proofs. Every proving system has its own way of generating proofs.

You can find examples on how to generate proofs in the [generating proofs guide](3_generating_proofs.md).

Additionally, you can find an example of the Fibonacci program proof and the script that generates it in the Risc0 example directory.

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

contract Fibonacci {
    address public alignedServiceManager;
    bytes32 public fibonacciImageId;

    bytes32 public fibonacciImageIdCommitment =
        0xbfa561e384be753bd6fd75b15db31eb511cd114ec76d619a87c2342af0ee1ed7;

    event FibonacciNumbersVerified(uint32 fibN, uint32 fibNPlusOne);

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
        uint32 fibN,
        uint32 fibNPlusOne
    ) public returns (bool) {
        require(
            fibonacciImageIdCommitment == provingSystemAuxDataCommitment,
            "Image ID doesn't match"
        );

        bytes32 calculatedCommitment = calculateCommitment(fibN, fibNPlusOne);

        require(
            pubInputCommitment == calculatedCommitment,
            "Fibonacci numbers don't match with public input"
        );

        // Emit the event after the require statements
        emit FibonacciNumbersVerified(fibN, fibNPlusOne);

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

    function calculateCommitment(
        uint32 fibN,
        uint32 fibNPlusOne
    ) public pure returns (bytes32) {
        bytes memory encoded = abi.encodePacked(fibN, fibNPlusOne);
        return keccak256(encoded);
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

1. **Commitment Calculation and Validation:** It calculates the commitment of the last two Fibonacci numbers modulo 7919, validates it against the submitted public input commitment, and emits an event.

```solidity
bytes32 calculatedCommitment = calculateCommitment(fibN, fibNPlusOne);

require(
    pubInputCommitment == calculatedCommitment,
    "Fibonacci numbers don't match with public input"
);
emit FibonacciNumbersVerified(fibN, fibNPlusOne);
```

```solidity
function calculateCommitment(
        uint32 fibN,
        uint32 fibNPlusOne
    ) public pure returns (bytes32) {
        bytes memory encoded = abi.encodePacked(fibN, fibNPlusOne);
        return keccak256(encoded);
    }
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


## Submit and verify the proof to Aligned

The proof submission and verification can be done either with the SDK or by using the Aligned CLI.

#### Using the SDK

To submit and check if the proof was correctly verified in Aligned using the SDK, you can use the `submit_and_wait` function.

The following code is an example of how to submit and wait for the verification of a proof using the SDK and then store the AlignedVerificationData in a `.json` file

```rust
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use aligned_sdk::core::errors::SubmitError;
use aligned_sdk::core::types::Chain::Holesky;
use aligned_sdk::core::types::{AlignedVerificationData, ProvingSystemId, VerificationData};
use aligned_sdk::sdk::submit_and_wait;
use ethers::signers::LocalWallet;
use ethers::types::Address;
use ethers::utils::hex;

#[tokio::main]
async fn main() -> Result<(), SubmitError> {
    let proof = read_file(PathBuf::from(
        "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof",
    ))
    .unwrap_or_default();
    let pub_input = read_file(PathBuf::from(
        "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub",
    ));
    let image_id = read_file(PathBuf::from(
        "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_id.bin",
    ));

    // Set to a dummy address
    let proof_generator_addr =
        Address::from_str("0x66f9664f97F2b50F62D13eA064982f936dE76657").unwrap();

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::Risc0,
        proof,
        pub_input,
        verification_key: None,
        vm_program_code: image_id,
        proof_generator_addr,
    };

    // Set to the 9th address of anvil that doesn't pay for the proof submission
    let wallet =
        LocalWallet::from_str("2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6")
            .expect("Failed to create wallet");

    let aligned_verification_data = submit_and_wait(
        "wss://batcher.alignedlayer.com",
        "https://ethereum-holesky-rpc.publicnode.com",
        Holesky,
        &verification_data,
        wallet,
    )
    .await?;

    // Directory were the AlignedVerificationData will be stored.
    let batch_inclusion_data_directory_path = PathBuf::from("./batch_inclusion_data");

    if let Some(aligned_verification_data) = aligned_verification_data {
        save_response(
            batch_inclusion_data_directory_path,
            &aligned_verification_data,
        )?;
    } else {
        return Err(SubmitError::EmptyVerificationDataList);
    }

    Ok(())
}

fn read_file(file_name: PathBuf) -> Option<Vec<u8>> {
    std::fs::read(file_name).ok()
}

fn save_response(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
) -> Result<(), SubmitError> {
    let batch_merkle_root = &hex::encode(aligned_verification_data.batch_merkle_root)[..8];
    let batch_inclusion_data_file_name = batch_merkle_root.to_owned()
        + "_"
        + &aligned_verification_data.index_in_batch.to_string()
        + ".json";

    let batch_inclusion_data_path =
        batch_inclusion_data_directory_path.join(batch_inclusion_data_file_name);

    let data = serde_json::to_vec(&aligned_verification_data)?;

    let mut file = File::create(&batch_inclusion_data_path)
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;

    file.write_all(data.as_slice())
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;

    Ok(())
}
```

This example generates a proof, instantiates a wallet to submit the proof, and then submits the proof to Aligned for verification. It then waits for the proof to be verified in Aligned and stores the verification data.

#### Using the CLI

You can find examples of how to submit a proof using the CLI in the [submitting proofs guide](0_submitting_proofs.md).
