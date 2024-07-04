# Integrating Aligned into your Application

Aligned can be integrated into your applications in a few simple steps to provide a way to verify ZK proofs generated inside your system.  

This example shows a sample app that generates an SP1 proof that a user knows the answers to a quiz, then submits the proof to Aligned for verification.
Finally, it includes a smart contract that verifies that a proof was verified in Aligned and mints an NFT.

You can find an example of the full flow of using Aligned on your app in the [ZKQuiz example](../../examples/zkquiz). 

## Steps

### Step 1 - Generate your ZK Proof

Generate your ZK proofs using any of the proving systems supported by Aligned.
For this example, we use the SP1 proving system. The current SP1 version used in Aligned is v1.0.8-testnet.

You can find the example of the quiz proof [program](../../examples/zkquiz/quiz/program/src/main.rs) as well as the [script](../../examples/zkquiz/quiz/script/src/main.rs) that generates it in the [ZKQuiz example](../../examples/zkquiz) directory.

### Step 2 - Write your smart contract

To check if a proof was verified in Aligned, you need to call to the Aligned ServiceManager contract inside your smart contract. 

Also, you will need a way to check that the proven program is your own.

The aligned cli provides a way for you to get the verification key commitment
without actually generating and submitting a proof.

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

You can find the example of the smart contract that checks the proof was verified in Aligned
in the [Quiz Verifier Contract](../../examples/zkquiz/contracts/src/VerifierContract.sol).

Note that the contract checks that the verification key commitment is the same as the program elf.
```solidity
require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");
```

This contract also includes a static call to the Aligned ServiceManager contract 
to check that the proof was verified in Aligned.

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

An example [python script](../../examples/zkquiz/encode_verification_data.py) can be found to encode the call data from the json output of the Aligned cli. 

```python
from argparse import ArgumentParser
from json import load
from eth_abi import encode
from Crypto.Hash import keccak


def encode_call(file):
    with open(file) as f:
        data = load(f)

        verification_data_commitment = data['verification_data_commitment']
        proof_commitment = bytearray(verification_data_commitment['proof_commitment'])
        pub_input_commitment = bytearray(verification_data_commitment['pub_input_commitment'])
        proving_system_aux_data_commitment = bytearray(
            verification_data_commitment['proving_system_aux_data_commitment'])
        proof_generator_addr = bytearray(verification_data_commitment['proof_generator_addr'])
        batch_merkle_root = bytearray(data['batch_merkle_root'])

        merkle_path_arr = data['batch_inclusion_proof']['merkle_path']
        merkle_proof = bytearray()
        for i in range(0, len(merkle_path_arr)):
            merkle_proof += bytearray(merkle_path_arr[i])

        index = data['index_in_batch']

        output = encode(['bytes32', 'bytes32', 'bytes32', 'bytes20', 'bytes32', 'bytes', 'uint256'],
                        [proof_commitment, pub_input_commitment, proving_system_aux_data_commitment,
                         proof_generator_addr, batch_merkle_root, merkle_proof, index])

        k = keccak.new(digest_bits=256)
        k.update(b'verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)')
        signature = k.hexdigest()[:8]

        return '0x' + signature + output.hex()
```

This is then used to call the smart contract using cast:

```bash
cast send \
    --rpc-url <rpc_url> \
    --private-key <private_key> \
    <contract_address> \
    <encoded_calldata>
```

This call can be done from any library that can interact with the Ethereum blockchain.

