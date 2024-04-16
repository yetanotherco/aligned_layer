# Aligned Layer

> [!CAUTION]
> To be used in testnet only.

Basic repo demoing a Stark/Snark verifier AVS middleware with full EigenLayer integration. 

## The Project 

Aligned Layer works with EigenLayer to leverage ethereum consensus mechanism for ZK proof verification. Working outside the EVM, this allows for cheap verification of any proving system. This enables the usage of cutting edge algorithms, that may use new techniques to prove even faster. Even more, proving systems that reduces the proving overhead and adds verifier overhead, now become economically feasable to verify thanks to Aligned Layer. 

Full documentation and examples will be added soon

## Dependencies

You will need [foundry](https://book.getfoundry.sh/getting-started/installation) and [zap-pretty](https://github.com/maoueh/zap-pretty) to run the examples below.
```
curl -L https://foundry.paradigm.xyz | bash
foundryup
go install github.com/maoueh/zap-pretty@latest
```

## Running via make

Start anvil in a separate terminal:

```bash
make start-anvil-chain-with-el-and-avs-deployed
```

The above command starts a local anvil chain from a [saved state](./tests/integration/eigenlayer-and-shared-avs-contracts-deployed-anvil-state.json) with EigenLayer and Aligned Layer contracts already deployed (but no operator registered).

Start the aggregator:

```bash
make start-aggregator
```

Register the operator with EigenLayer and Aligned Layer, then start the process. In MacOs, run:

```bash
make cli-setup-operator-macos
make start-operator
```

for Linux, use 

```bash
make cli-setup-operator-linux
make start-operator
```


Start the task generator, which will be sending periodic tasks to the Aligned Layer task manager:

```bash
make start-task-generator
```

To send custom tasks with proofs to be verified, in another terminal you can run:
```bash
go run task_sender/cmd/main.go --proof <proof_path> --verifier-id <verifier-string-variant>
```

where `proof_path` is the path of the file containing the serialized proof you want to be verified and `verifier-string-variant` is either `cairo` or `plonk`.

A shortcut for sending a CAIRO proof of a fibonacci program can be used:

```bash
make send-cairo-proof
```

Likewise, for sending a PLONK proof of a cubic circuit:

```bash
make send-plonk-proof
```

## Workflow

To re compile contracts in case of changes use:

```bash
make deploy-all-to-anvil-and-save-state
```

To re generate the bindings in go:

```bash
make bindings
```

## FAQ

**What is the objective of Aligned Layer?**

Aligned Layer’s mission is to extend Ethereum’s zero-knowledge capabilities. We are certain the zero-knowledge proofs will have a key role in the future of blockchains and computation. We don’t know what that future will look like, but we are certain it will be in Ethereum. The question we want to share is: If we are certain zero-knowledge proofs are the future of Ethereum but we are not certain which of the many possible zero-knowledge futures will win. How can we build an infrastructure for Ethereum to be compatible with any future zero-knowledge proving system?

**Why do we need a ZK verification layer?**

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and help it increase its capabilities. 

**What are the use cases of Aligned Layer?**

**Why build on top of Ethereum?**

Ethereum is the most decentralized and biggest source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned Layer is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.

**Why not do this directly on top of Ethereum?**

In order to do this we would have to aggregate all the proofs into a single proof. This is not a good solution considering that we would need some way to wrap proofs (for example, by means of recursion), which involves complex operations such as field emulation, bitwise, and/or elliptic curve operations. 

**Why not make Aligned Layer a ZK L1?**

An L1 would not have the security properties of Ethereum consensus, and bootstrapping a new decentralized network is not only expensive but might be an impossible task. Zero-knowledge proofs are a nascent technology, and change is a constant. The best solution for today may not be the best for tomorrow; modifying L1s is extremely costly, especially as time progresses. 

**Why not a ZK L2?**

An L2 needs to use the EVM to settle in Ethereum. This means that the proofs need to be efficiently verified in the EVM, and their data made available there.

The EVM is not designed for ZK Verification, so most verifications are expensive.

To solve this, for pairing-based cryptography, Ethereum has added a precompile for verifications using the curve BN254.

But technology changes fast. BN254 security was demonstrated to be around 100 bits instead of the expected 128. Fast Starks need efficient hashing for fields. Which is the best field? Mersenne’s? Goldilocks? Binary fields? What about the sumcheck protocol? Is Jolt the endgame? Or is GKR going to be faster?

The amount of progress in the field is big, and nobody can predict the endgame.

Even more, it would be naive to think that only one optimized prover will exist in the future. In the world of ZK, as in many others, there are trade-offs and systems that solve different problems.

Maybe we want faster proving and don't care about proof size. Maybe we want the fastest proof verification and smallest size and can do more work on the prover. The system may be optimized to prove Keccak really fast. Or we can skip the traditional hashes altogether and just optimize for Poseidon, Rescue, or one hash not created yet.

Aligned Layer solves all of this. No matter how or what you want to prove, it can be verified efficiently here while still inheriting the security of Ethereum as other L2s.

**Why EigenLayer?**

We believe Ethereum is the best settlement layer, and zero-knowledge will play a key role in helping it be THE settlement layer of the internet. We want to build a verification layer that helps Ethereum achieve this goal. This layer needs to have a decentralized group of validators that will just re-execute the verification of different proofs, but how can we build such a decentralized network that will help Ethereum? Creating a new L1 doesn’t benefit Ethereum because using it will add new trust assumptions to the Ethereum protocols relying on it. So, if we must have:

1. A decentralized network of verifiers
2. A similar economic security level that can be easily measured in Ethereum
3. Part of the Ethereum ecosystem
4. Flexible enough to support many current and future proving systems

**Will you aggregate proofs?**

Proof aggregation can also be supported by proving the verification of many of these different verifications. This will likely not be an urgent feature, but it will be needed in the future with more demand.

**How does it compare to the Polygon aggregation layer?**

Aligned Layer is just a network of decentralized verifiers renting security from Ethereum. On the other hand, the Polygon aggregation layer, in essence, is a rollup verifying multiple proofs. That is not the case for Aligned Layer, which just executes a rust binary from different verifiers directly in multiple Ethereum validators.

## Acknowledgments

We want to thank Layr-Labs for creating [Incredible Squaring AVS](https://github.com/Layr-Labs/incredible-squaring-avs), which was used to bootstrap the initial version of Aligned Layer, SuccintLabs for its [SP1](https://github.com/succinctlabs/sp1) zkVM and Consensys for its [gnark](https://github.com/Consensys/gnark) library, which have been crucial for showcasing the ease of embedding validity proof verification systems in Aligned Layer, and [RISC Zero](https://github.com/risc0/risc0), [Valida](https://github.com/valida-xyz/valida) and Polygon Zero's [Plonky3](https://github.com/Plonky3/Plonky3) for being foundational building blocks for these technologies.
