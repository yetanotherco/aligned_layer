# Integrating Aligned into your Application

Aligned can be integrated into your applications in a few simple steps to provide a way to verify ZK proofs generated inside your system.

You can find an example of the full flow of using Aligned in your app in the [ZKQuiz example](../../examples/zkquiz).

This example shows a sample app that generates an SP1 proof that a user knows the answers to a quiz and then submits the proof to Aligned for verification. Finally, it includes a smart contract that verifies that a proof was verified in Aligned and mints an NFT.

## 1. Generate your ZK Proof

Generate your ZK proofs using any of the proving systems supported by Aligned.
For this example, we use the SP1 proving system. The current SP1 version used in Aligned is v1.0.8-testnet.

```rust
use sp1_sdk::{ProverClient, SP1Stdin};
use std::io;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn generate_sp1_proof(user_answers: &str) -> Result<Vec<u8>, &'static str> {
    let mut stdin = SP1Stdin::new();
    stdin.write(user_answers);

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);

    match client.prove_compressed(&pk, stdin) {
        Ok(proof) => {
            client.verify_compressed(&proof, &vk).expect("verification failed");
            println!("Proof generated and verified successfully.");
            Ok(proof)
        }
        Err(_) => {
            println!("Proof generation failed. Incorrect answer");
            Err("Proof generation failed")
        }
    }
}

fn main() {
    // Example user answers
    let user_answers = "abc";
    match generate_sp1_proof(user_answers) {
        Ok(proof) => println!("Proof: {:?}", proof),
        Err(err) => println!("Error: {}", err),
    }
}
```

You can find an example of the quiz proof [program](../../examples/zkquiz/quiz/program/src/main.rs) as well as the [script](../../examples/zkquiz/quiz/script/src/main.rs) that generates it in the [ZKQuiz example](../../examples/zkquiz) directory.

## 2. Write your smart contract

To check if a proof was verified in Aligned, you need to make a call to the `AlignedServiceManager` contract inside your smart contract.

Also, you will need a way to check that the proven program is the correct one.

The Aligned CLI provides a way for you to get the verification key commitment without actually generating and submitting a proof.

You can do this by running the following command:

```bash
aligned get-vk-commitment --input <path_to_input_file>
```

The following is an example of how to call the `verifyBatchInclusionMethod` from the Aligned ServiceManager contract in your smart contract.

```solidity
contract YourContract {
    // Your contract variables ...
    address public alignedServiceManager;
    bytes32 public elfCommitment = <elf_commitment>;

    constructor(address _alignedServiceManager) {
        //... Your contract constructor ...
        alignedServiceManager = _alignedServiceManager;
    }
    
    // Your contract code ...
    
    function yourContractMethod(
        //... Your function variables, ...
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) {
        // ... Your function code
        
        require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");
        
        (bool callWasSuccessful, bytes memory proofIsIncluded) = alignedServiceManager.staticcall(
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
        
        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));
        require(proofIsIncludedBool, "proof not included in batch");
        
        // Your function code ...
    }
}
```

You can find an example of the smart contract that checks if the proof was verified in Aligned in the [Quiz Verifier Contract](../../examples/zkquiz/contracts/src/VerifierContract.sol).

Note that the contract checks that the verification key commitment is the same as the program ELF commitment.

```solidity
require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");
```

This contract also includes a static call to the Aligned ServiceManager contract to check if the proof was verified in Aligned.

```solidity
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
```

## 3. Submit and verify the proof to Aligned

The proof submission and verification can be done either with the SDK or by using the Aligned CLI.

#### Using the SDK

To submit a proof using the SDK, you can use the `submit` function, and then you can use the `verify_proof_onchain` function to check if the proof was correctly verified in Aligned.

The following code is an example of how to submit a proof using the SDK:

```rust
use aligned_sdk::sdk::submit;
use aligned_sdk::types::{ProvingSystemId, VerificationData};
use ethers::prelude::*;

const BATCHER_URL: &str = "wss://batcher.alignedlayer.com";
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

async fn submit_proof_to_aligned(
    proof: Vec<u8>,
    wallet: Wallet<SigningKey>
) -> Result<AlignedVerificationData, anyhow::Error> {
    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        proof_generator_addr: wallet.address(),
        vm_program_code: Some(ELF.to_vec()),
        verification_key: None,
        pub_input: None,
    };

    submit(BATCHER_URL, &verification_data, wallet).await
        .map_err(|e| anyhow::anyhow!("Failed to submit proof: {:?}", e))
}

#[tokio::main]
async fn main() {
    let wallet = // Initialize wallet
    let proof = // Generate or obtain proof

    match submit_proof_to_aligned(proof, wallet).await {
        Ok(aligned_verification_data) => println!("Proof submitted successfully"),
        Err(err) => println!("Error: {:?}", err),
    }
}
```

The following code is an example of how to verify the proof was correctly verified in Aligned using the SDK:

```rust
use aligned_sdk::sdk::verify_proof_onchain;
use aligned_sdk::types::{AlignedVerificationData, Chain};
use ethers::prelude::*;
use tokio::time::{sleep, Duration};

async fn wait_for_proof_verification(
    aligned_verification_data: AlignedVerificationData,
    rpc_url: String,
) -> Result<(), anyhow::Error> {
    for _ in 0..10 {
        if verify_proof_onchain(aligned_verification_data.clone(), Chain::Holesky, rpc_url.as_str()).await.is_ok_and(|r| r) {
            println!("Proof verified successfully.");
            return Ok(());
        }
        println!("Proof not verified yet. Waiting 10 seconds before checking again...");
        sleep(Duration::from_secs(10)).await;
    }
    anyhow::bail!("Proof verification failed")
}

#[tokio::main]
async fn main() {
    let aligned_verification_data = // Obtain aligned verification data
    let rpc_url = "https://ethereum-holesky-rpc.publicnode.com".to_string();

    match wait_for_proof_verification(aligned_verification_data, rpc_url).await {
        Ok(_) => println!("Proof verified"),
        Err(err) => println!("Error: {:?}", err),
    }
}
```

You can find an example of the proof submission and verification in the [Quiz Program](../../examples/zkquiz/quiz/script/src/main.rs).

The example generates a proof, instantiates a wallet to submit the proof, and then submits the proof to Aligned for verification. It then waits for the proof to be verified in Aligned.

#### Using the CLI

You can find examples of how to submit a proof using the CLI in the [submitting proofs guide](0_submitting_proofs.md).
