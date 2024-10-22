# Validating public input

In some applications, it is crucial to ensure that a third party has performed a computation correctly and to make use of the result of that computation. To achieve this, the third party must interact with Aligned, using the Aligned SDK, to obtain the `AlignedVerificationData`, a receipt indicating that the proof of the computation was verified correctly. The application should then receive both the `AlignedVerificationData` and the result of the computation. After confirming that the proof was verified by Aligned, it must check that the posted result matches the one committed in the `AlignedVerificationData`.

This guide demonstrates how to validate Risc0 and SP1 proofs using the Aligned SDK. The program in this example is a Fibonacci sequence calculator. It generates a public input that corresponds to the number of fibonacci being calculated and the last two Fibonacci numbers of the sequence, taken modulo 7919. Our goal is to validate, within a smart contract, that the public input commitments match these numbers.

In this case, the Fibonacci number to be calculated is **500** and the last two numbers of the sequence modulo 7919 are **1268** and **1926**.

## Requirements
- [SP1](https://docs.succinct.xyz/getting-started/install.html)
- [Risc0](https://dev.risczero.com/api/zkvm/install)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

## The program

The Fibonacci program to be proven is essentially a Rust program with a few additional functions from the `risc0` and `sp1` libraries. These extra functions allow for the submission of public inputs and enable the generation of a proof.

### Risc0

For `risc0`, the Fibonacci program can be found in `examples/validating-public-input/risc_zero/fibonacci_proof_generator/methods/guest/src/main.rs`, and it's known as the guest code. This code compiles into a binary file that is later used to correctly generate the proof. The host code, located in `examples/validating-public-input/risc_zero/fibonacci_proof_generator/host/src/main.rs`, is responsible for executing the program with the given input and generating a receipt that contains both the proof and all output data from the process.

For more details about `risc0` and the interaction between guest and host code, as well as how the various parts work, you can refer to the official documentation [here](https://dev.risczero.com/api/zkvm/).

### SP1

For `SP1`, the Fibonacci program is located in `examples/validating-public-input/sp1/fibonacci/program/src/main.rs`, and it functions similarly to the one written for `risc0`. It follows a similar structure and process to generate a proof. The code responsible for executing and proving the program can be found in `examples/validating-public-input/sp1/fibonacci/script/src/main.rs`. Both components work in tandem, much like in the `risc0` framework.

## Generate your ZK Proof

> [!IMPORTANT]
> To generate the proof ensure you have [docker](https://www.docker.com/get-started/) installed and the docker daemon running.
> This is necessary to ensure deterministic builds of the binary we want to generate a proof of. If not used, builds may differ depending on the system you are running on. To know more about this, check [this link](https://dev.risczero.com/terminology#deterministic-builds) from RiscZero docs or [this](https://docs.succinct.xyz/writing-programs/compiling.html#advanced-build-options-1) from SP1.

To submit proofs to **Aligned** and get them verified, you first need to generate those proofs. Every proving system has its own method for generating proofs.

Examples on how to generate proofs can be found in the [generating proofs guide](4_generating_proofs.md).

To generate the proof required for this example, run the following commands:

- For **Risc0**: `make generate_risc0_fibonacci_proof`
- For **SP1**: `make generate_sp1_fibonacci_proof`

Once completed, you will see output that includes the program ID, the public inputs (which are the initial number of steps in the sequence and the last two Fibonacci numbers of the sequence), and the verification result, like so:

```
Program ID: 0xf000637ed63d26fc664f16666aebf05440ddb7071931240dc49d9bbcfbac304a
n: 500
a: 1268
b: 1926
Verification result: true
Fibonacci proof, pub input, and image ID generated in <verifier> folder
```

The command generates three different files, which will be used for later validation:
- An `.elf` file containing the compiled program.
- A `.proof` file containing the proof bytes for the program.
- A `.pub` file containing the serialized public input bytes committed by the program.

## Submit and verify the proof to Aligned

> For more details on submitting proofs and setting up a local wallet keystore, refer to the [submitting proofs guide](0_submitting_proofs.md).

The proof submission and verification process can be done either using the SDK or the Aligned CLI. In this case, we’ll use the **Aligned SDK** to better illustrate how the entire process works.

To submit the **Risc0** proof generated in this example, run:

```sh
make submit_fibonacci_risc0_proof KEYSTORE_PATH=<KEYSTORE_PATH>
```

Alternatively, you can submit the one generated with **SP1** by running:

```sh
make submit_fibonacci_sp1_proof KEYSTORE_PATH=<KEYSTORE_PATH>
```

This command will execute the Rust code that handles the proof submission with the appropriate verifier. You can find this code in the file `examples/validating-public-input/aligned-integration/src/main.rs`. It acts as the integration layer between the proof-generating program and the proof submission process to **Aligned**.

The data necessary to send to aligned follows this structure:

```rust
VerificationData {
    // The proving system ID.
    proving_system
    // The proof bytes previously serialized.
    proof,
    // The public input bytes.
    pub_input,
    // The bytes of the verification key if necessary.
    verification_key: None,
    // The bytes corresponding to the compiled program.
    vm_program_code: elf,
    // The address of the wallet that generated the proof.
    proof_generator_addr,
}
```

It could take some time but once this proof is submitted and executed within Aligned, you should see an output similar to:

```
INFO  aligned_integration: Saved batch inclusion data to ".../aligned_layer/examples/validating-public-input/aligned-integration/batch_inclusion_data/<JSON_FILE_NAME>"
```

The file logged in `<JSON_FILE_NAME>` will contain the `AlignedVerificationData`. This data is essential for sending the transaction to the `verifyBatchInclusion` method of the smart contract, which verifies the inclusion of your proof in Aligned and checks the correctness of the compiled program and public inputs.

Each generated proof gets its own file name, so ensure to save the filename or remember it for future steps, as it will be required later. You can check the generated data files in `aligned-layer/examples/validating-public-input/aligned-integration/batch_inclusion_data`

## Validating the public inputs

To check if a proof was verified in Aligned, you need to make a call to the `AlignedServiceManager` contract from within your smart contract.

We previously reviewed the structure of a Verifier contract when building our first application; you can find that information [here](./2_build_your_first_aligned_application.md#verifier-contract) if you'd like to revisit it.
Now, we need to implement a check to ensure that the public inputs match the expected values. To accomplish this, we have added a new parameter to our `verifyBatchInclusion` function in the smart contract, which will receive the bytes of the public inputs directly from the `.pub` file generated during compilation.

Now the function should look like this for both sp1 and risc0 proofs.

```solidity
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
        pubInputCommitment == keccak256(abi.encodePacked(pubInputBytes)),
        "Fibonacci numbers don't match with public input"
    );
```

Since the format of the generated byts is the same for both of the verifiers, we can later decode the inputs if we want to do something with them, in this case we emit an event:

```solidity
(uint32 n, uint32 fibN, uint32 fibNPlusOne) = bytesToTwoUint32(pubInputBytes);

emit FibonacciNumbers(n, fibN, fibNPlusOne);

function bytesToTwoUint32(
    bytes memory data
) public pure returns (uint32, uint32, uint32) {
    require(data.length >= 8, "Input bytes must be at least 8 bytes long");

    uint32 first = uint32(uint8(data[0])) |
        (uint32(uint8(data[1])) << 8) |
        (uint32(uint8(data[2])) << 16) |
        (uint32(uint8(data[3])) << 24);

    uint32 second = uint32(uint8(data[4])) |
        (uint32(uint8(data[5])) << 8) |
        (uint32(uint8(data[6])) << 16) |
        (uint32(uint8(data[7])) << 24);

    uint32 third = uint32(uint8(data[8])) |
        (uint32(uint8(data[9])) << 8) |
        (uint32(uint8(data[10])) << 16) |
        (uint32(uint8(data[11])) << 24);

    return (first, second, third);
}
```
We have already implemented the contract with these features; you can check it in `contracts/src/FibonacciValidator.sol`.

To test it, you'll need to deploy the contract. First, create a new `.env` file following the format of `.env.example`, ensuring to add the `private_key` you wish to use for deployment. Make sure you have a sufficient balance on the Holesky testnet. For all other values, you can use the default settings provided in the comments.

Once your `.env` file is set up, you can deploy the contract using the following command:

```bash
make deploy_fibonacci_validator
```

This command will log the address of the deployed contract like so: 

```
== Return ==
0: address 0x5081a39b8A5f0E35a8D959395a630b68B74Dd30f
```

Make sure to save this address, as you'll need it for the next step.

Now, to call our verifier contract and check the inclusion of the proof along with the validation of the public inputs, use the following command based on the verifier you used:

- For Risc0:
  ```bash
  make verify_risc0_batch_inclusion FIBONACCI_VALIDATOR_ADDRESS=<FIBONACCI_VALIDATOR_ADDRESS> DATA_FILE_NAME=<DATA_FILE_NAME>
  ```

- For SP1:
  ```bash
  make verify_sp1_batch_inclusion FIBONACCI_VALIDATOR_ADDRESS=<FIBONACCI_VALIDATOR_ADDRESS> DATA_FILE_NAME=<DATA_FILE_NAME>
  ```

In these commands:
- `<FIBONACCI_VALIDATOR_ADDRESS>` is the address of the validator you deployed in the previous step.
- `<DATA_FILE_NAME>` is the name of the file where the aligned data for this proof was saved (including the `.json` extension)

When you run this command, it will gather all necessary information from the file containing the aligned data and send a transaction to the Fibonacci validator using the `cast send` tool, like so:

```bash
cast send --rpc-url $RPC_URL $FIBONACCI_VALIDATOR_ADDRESS \
	"verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256, bytes, string)" \
    $PROOF_COMMITMENT \
    $PUB_INPUT_COMMITMENT \
    $PROGRAM_ID_COMMITMENT \
    $PROOF_GENERATOR_ADDR \
    $BATCH_MERKLE_ROOT \
    $MERKLE_PROOF \
    $VERIFICATION_DATA_BATCH_INDEX \
    $PUB_INPUT \
    $VERIFIER_ID \
    --private-key $PRIVATE_KEY
```

If the output of this transaction indicates `success`, then we can confirm that our proof has been successfully included in Aligned. Additionally, this outcome verifies that the public inputs we generated match the expected values from the proof generation process.
