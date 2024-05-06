# How does Aligned Layer work?

The architecture of the [AVS]( https://docs.eigenlayer.xyz/eigenlayer/overview/key-terms) is as follows:

![Figure 1: Architecture of the AVS. Source: EigenLayer Incredible Squaring AVS repo](../images/diagram.png)

There we have:

- [EigenLayer core](https://github.com/Layr-Labs/eigenlayer-contracts/tree/master) contracts.
- AVS contracts:

  - ServiceManager (which must contain slashing logic).
  - TaskManager (which contains task creation and task response logic).
  - [Registry contracts](https://github.com/Layr-Labs/eigenlayer-middleware) (to manage operators opted in to this AVS).
  
- Task Generator.
- Aggregator.
- Operators (EigenLayer operators opted-in to this AVS).

Here you have a more detailed diagram of the aggregator and operator processes:

![Figure 2: Aggregator and operator processes. Source: EigenLayer Incredible Squaring AVS repo](../images/diagram_2.png)

## Aligned Layer architecture:

The core elements are:

- Aligned Layer: It receives proofs from different proof systems, verifies them, sends the final result to Ethereum, and posts the data into a DA layer.
  
- Data Availability Layer (DA): provides storage for the different proofs.
  
- General Prover/Verifier: Every several days, takes the proofs from the DA layer and generates a proof of the verification of all the proofs. The general prover can be based on the SP1, Risc0 or Nexus virtual machine, which is a virtual machine able to prove general Rust code. The proof of the verification of the proofs is done using the corresponding verifier codes in Rust. The verification can be done using a tree structure.
  
- Ethereum: The source of trust and liquility.

### Interaction between the main components:

![Figure 3: Core components](../images/diagram_3.png)

1. Aligned Layer receives several proofs from different sources, which could be generated using different proof systems. These have varying proof sizes, verification times, and different verification logic. However, all proofs share a common characteristic: their verification is fast.

2. Aligned Layer has dedicated verifiers, which can be used to check the validity of each of them and post the results to Ethereum.
The proofs are stored in a Data Availability Layer, which offers a cost-efficient storage strategy. The operators in Aligned fetch the proof’s data from this layer.

3. Ethereum receives the verification results from Aligned Layer. However, Ethereum itself does not have access to verify the proofs since this would be too costly. Instead, a general Rust prover generates proof of verification for all the proofs included in a given period (see Figure 4).

4. To aggregate all the proofs, in the first step, all proofs are transformed into proofs of execution of the virtual machine, achieving proof uniformity (see Figure 4). We can then shrink proof size by recursively proving the verification of proofs, as shown in the tree diagram (see Figure 5).

![Figure 4: Prover](../images/prover.png)

![Figure 5: Recursion tree](../images/recursion.png)
