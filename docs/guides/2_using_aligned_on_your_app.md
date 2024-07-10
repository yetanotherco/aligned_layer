# Using Aligned on your app

You can find an example of the full flow of using Aligned on your app 
in the [ZKQuiz example](../../examples/zkquiz). 

This example shows a sample app that generates a SP1 proof 
that a user knows the answers to a quiz, then submits the proof 
to Aligned for verification.
Finally, it includes a smart contract that verifies that a proof 
was verified in Aligned and mints an NFT.

## Steps

### Step 1 - Write your ZK Proof

Write your ZK proof using any of the proving systems supported by Aligned.
For this example, we use the SP1 proving system. The current SP1 version used in Aligned is v1.0.8-testnet.

You can find the example of the quiz proof [program](../../examples/zkquiz/quiz/program/src/main.rs) 
as well as the [script](../../examples/zkquiz/quiz/script/src/main.rs) 
that generates it in the [ZKQuiz example](../../examples/zkquiz) folder.

### Step 2 - Write your smart contract

Write your smart contract that verifies the proof was verified in Aligned.
For this, you will need a way to check that the proven program is your own.

The aligned cli provides a way for you to get the verification key commitment 
without actually generating and submitting a proof.

You can do this by running the following command:

```bash
aligned get-vk-commitment --input <path_to_input_file>
```

For SP1 you would use the elf of the program as the input file.

You can find the example of the smart contract that verifies the proof was verified in Aligned
in the [Quiz Verifier Contract](../../examples/zkquiz/contracts/src/VerifierContract.sol).

Note that the contract checks that the verification key commitment is the same as the program elf.
```solidity
require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");
```

This contracts also includes a static call to the Aligned ServiceManager contract 
to check that the proof was verified in Aligned. For a full version of this, you can view, use as an example guide, or inherit the [Verify Batch Inclusion Caller](../../examples/verify/src/VerifyBatchInclusionCaller.sol) smart contract.

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

### Step 3 - Have your app generate the proof and submit it to Aligned

First, generate the proof. 
For SP1 this means having the [script](../../examples/zkquiz/quiz/script/src/main.rs)
generate the proof.

Then, submit the proof to Aligned for verification. 
This can be done either with the SDK or by using the Aligned CLI.
You can find examples of how to submit a proof using the cli 
in the [submitting proofs guide](0_submitting_proofs.md).

The call ZK Quiz uses is:
```bash
aligned submit \
    --proving_system SP1 \
    --proof quiz/script/proof-with-io.json \
    --vm_program quiz/program/elf/riscv32im-succinct-zkvm-elf \
    --proof_generator_addr <user_address> \
    --conn wss://batcher.alignedlayer.com
```

### Step 4 - Verify the proof was verified in Aligned

Once the proof is verified in Aligned, 
you can verify that it was verified from your smart contract.

The full example of this flow can be found on the [ZKQuiz Verifier Contract](../../examples/zkquiz/contracts/src/VerifierContract.sol).

An example [python script](../../examples/verify/encode_verification_data.py) can be found 
to encode the call data from the json output of the Aligned cli. 
This is then used to call the smart contract using cast:

```bash
cast send \
    --rpc-url <rpc_url> \
    --private-key <private_key> \
    <contract_address> \
    <encoded_calldata>
```

This call can be done from any library that can interact with the Ethereum blockchain.

