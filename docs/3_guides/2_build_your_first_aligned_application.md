# Build your first Application

In this guide you will learn how to build applications on top of Aligned. It provides a few simple steps to help you verify ZK proofs generated within your system.

First we will show you an example of a trivia application, called ZkQuiz. We'll show you the different components and how they interact with each other to be able to submit the proof to aligned and verify that was correctly included in a batch.

## ZkQuiz

ZkQuiz is an application that leverages Aligned's ZK verification infrastructure to run a small trivia. The proof allows any party to check that the quiz was answered right or wrong. If answered correctly, the user receives an NFT.

{% hint style="warning" %}
Received NFTs from ZkQuiz do not have any value. It is just a test application
{% endhint %}

The process is as follows:

1. The user runs ZKQuiz and answers the questions.
2. ZKQuiz generates a ZK Proof of correct answers.
3. The proof is posted on Aligned.
4. Upon verification, ZKQuiz mints an NFT via a Smart Contract.

The NFT is only granted if the user's answers correctly.
Incorrect answers or tampering with the ZKQuiz code will result in proof generation failure or mismatched checksums,
preventing NFT minting.

Next, we will see how to execute ZKQuiz to get your own ZKQuiz NFT!

### Requirements

1. [Rust v1.80.0](https://www.rust-lang.org/tools/install)
2. [Foundry](https://getfoundry.sh)

{% hint style="info" %}
ELF commitment was generated with Rust v1.80.1 and SP1 v3.0.0.
{% endhint %}

### Usage

#### 1. Clone the repository

```bash
git clone https://github.com/yetanotherco/aligned_layer.git && cd aligned_layer
```

#### 2. Create a Keystore

You need a keystore to pay for the proof verification, you can use cast to create a local keystore.
If you already have one, you can skip this step.

```bash
cast wallet new-mnemonic
```

Then you can import your created keystore using:

```bash
cast wallet import --interactive <keystore_name>
```

The keystores are saved in `~/.foundry/keystores`. You can find more information about keystores in the [cast documentation](https://book.getfoundry.sh/reference/cast/wallet-commands).

Then you need to get some funds to pay for gas and proof verification.
You can do this by using one of the following faucets:

- [Google Faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)
- [Stakely Faucet](https://stakely.io/faucet/ethereum-holesky-testnet-eth)
- [Quicknode Faucet](https://faucet.quicknode.com/ethereum/holesky)

#### 3. Answer Quiz

To answer quiz questions run:

```bash
cd examples/zkquiz
make answer_quiz KEYSTORE_PATH=<path_to_keystore>
```

This will:

1. Ask quiz questions
2. Generate ZK proof
3. Pay & submit proof to aligned for verification
4. Wait for proof to be verified in aligned
5. Claim NFT if proof is verified

## Deep dive

The ZkQuiz source code is available [here](../../examples/zkquiz).

ZkQuiz has three main components:

- App/script
- Program
- Verifier contract

The user interacts with ZkQuiz App to solve a trivia challenge answering questions. Then, the App generates a Zk Proof with the Program generated using SP1.

{% hint style="info" %}
The ZkQuiz Program is built using SP1 following the [quickstart guide](https://docs.succinct.xyz/getting-started/quickstart.html#project-overview). For your projects, you can user any of the [prooving systems supported by Aligned](../2_architecture/0_supported_verifiers.md).
{% endhint %}

Once the proof is generated, the App sends the proof to Aligned, and once it is verified, the App calls to the ZkQuiz Verifier Contract to check the proof verification and send an NFT to the user is the proof was verified in Aligned.

![ZkQuiz](../images/zkquiz.png)

Now, lets build ZkQuiz from scratch.

### Program

First you need to write the code you want to prove; in this case it looks like this:

```rust
// program/src/main.rs

#![no_main]

use tiny_keccak::{Hasher, Sha3};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let answers = sp1_zkvm::io::read::<String>();
    let mut sha3 = Sha3::v256();
    let mut output = [0u8; 32];

    sha3.update(answers.as_bytes());

    sha3.finalize(&mut output);

    if output
        != [
            232, 202, 155, 157, 82, 242, 126, 73, 75, 22, 197, 34, 41, 170, 163, 190, 22, 29, 192,
            5, 99, 134, 186, 25, 77, 128, 188, 154, 238, 70, 245, 229,
        ]
    {
        panic!("Answers do not match");
    }
}
```

The program takes the user answers as inputs and checks that the hash of the inputs matches with the expected output. This is the program that will be compiled generating a binary file that will be ran by the zkVm and used later in the application side. In our case this file is already generated and is located on `/quiz/program/elf/riscv32im-succinct-zkvm-elf`.

### Verifier Contract

To check if a proof was verified in Aligned, you can create your own smart contract in order to make a call to the `AlignedServiceManager` contract.

ZkQuiz uses a Smart Contract to check if aligned verified the proof and gives an NFT to the user.

{% hint style="info" %}
It is not mandatory to create an Smart Contract. You can make off-chain apps that interact with the Aligned contract directly.
{% endhint %}

**Program Identifier Validation**

The contract first checks that the commitment of the program matches with the one that we expect.
In our zkquiz example, we get the following elf commitment:

```solidity
// contracts/src/VerifierContract.sol
bytes32 public elfCommitment = 0x3f99615fdf3b67a01e41b38eee75a32c778ee2fa631bd74e01c89afc2f70f5de;
```

You can generate the expected commitment without actually generating and submitting a proof using the Aligned CLI tool running:

```bash
aligned get-vk-commitment --verification_key_file <path_to_input_file> --proving_system <proving_system_id>
```

where the `path_to_input_file` is the path to the `elf` file generated with the program compilation and the `proving_system_id` the name of the proving system used for compilation, in this case `SP1`.

Then, the contract validates if the provided commitment of the program identifier matches the expected one.

```solidity
// contracts/src/VerifierContract.sol
if (elfCommitment != provingSystemAuxDataCommitment) {
    revert InvalidElf(provingSystemAuxDataCommitment);
}
```

The contract makes a call to the `AlignedServiceManager` contract to check if the proof was verified in Aligned.

```solidity
// contracts/src/VerifierContract.sol
(
   bool callWasSuccessfull,
   bytes memory proofIsIncluded
) = alignedServiceManager.staticcall(
                abi.encodeWithSignature(
                    "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)",
                    proofCommitment,
                    pubInputCommitment,
                    provingSystemAuxDataCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot,
                    merkleProof,
                    verificationDataBatchIndex,
                    paymentServiceAddr
                )
            );

require(callWasSuccessfull, "static_call failed");

bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));

require(proofIsIncludedBool, "proof not included in batch");
```

Finally, if the proof was verified, the contract sends a NFT to the user

```solidity
// contracts/src/VerifierContract.sol

    _mint(msg.sender, tokenId);
    _setTokenURI(
        tokenId,
        "ipfs://QmUKviny9x2oQUegyJFFBAUU2q5rvu5CsPzrUaBSDukpHQ"
    );
```

### App

The first part of the app takes the answers of the user via CLI.
Once the user answer the questions, we prepare them and initiate the prover, as follows:

```rust
// script/src/main.rs

// Include the bytes of the compiled program.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

// Generate proof.
let mut stdin = SP1Stdin::new();

stdin.write(&user_awnsers);

println!("Generating Proof ");

let client = ProverClient::new();
let (pk, vk) = client.setup(ELF);

let Ok(proof) = client.prove(&pk, stdin).run() else {
    println!("Incorrect answers!");
    return;
};

println!("Proof generated successfully. Verifying proof...");
client.verify(&proof, &vk).expect("verification failed");
println!("Proof verified successfully.");
```

Now we can send the generated proof to Aligned using the SDK.

```rust
// script/src/main.rs

// Serialize the proof to later save in a file.
let proof = bincode::serialize(&proof).expect("Failed to serialize proof");

// Preparing the data needed for verification in Aligned
let verification_data = VerificationData {
    proving_system: ProvingSystemId::SP1,
    proof,
    proof_generator_addr: wallet.address(),
    vm_program_code: Some(ELF.to_vec()),
    verification_key: None,
    pub_input: None,
};

let max_fee = estimate_fee(&rpc_url, PriceEstimate::Default)
    .await
    .expect("failed to fetch gas price from the blockchain");

let max_fee_string = ethers::utils::format_units(max_fee, 18).unwrap();

let nonce = get_nonce_from_ethereum(&rpc_url, wallet.address(), NETWORK)
    .await
    .expect("Failed to get next nonce");

// Submit to Aligned.
let aligned_verification_data = submit_and_wait_verification(
    BATCHER_URL,
    &rpc_url,
    NETWORK,
    &verification_data,
    max_fee,
    wallet.clone(),
    nonce,
    )
.await
.unwrap();
```

Finally, if the proof was sent to Aligned correctly, we can interact with our verifier Smart Contract to verify that the proof was correctly posted in aligned and claim the NFT.

```rust
// script/src/main.rs

// Sends a transaction to the verifier contract with the
// verification data provided by aligned
claim_nft_with_verified_proof(
    &aligned_verification_data,
    signer,
    &args.verifier_contract_address,
)
.await
.expect("Claiming of NFT failed ...");
```

You can find the full code of the proof submission and verification in the [ZKQuiz App](../../examples/zkquiz/quiz/script/src/main.rs).
