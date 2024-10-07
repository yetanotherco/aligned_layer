## Aggregation mode in a nutshell

General Prover/Verifier: Every several days, takes the proofs from the DA layer and generates a proof of the verification of all the proofs. The general prover can be based on the SP1, Risc0 or Nexus virtual machine, which is a virtual machine able to prove general Rust code. The proof of the verification of the proofs is done using the corresponding verifier codes in Rust. The verification can be done using a tree structure.

To aggregate all the proofs, in the first step, all proofs are transformed into proofs of execution of the virtual machine, achieving proof uniformity (see Figure 1). We can then shrink proof size by recursively proving the verification of proofs, as shown in the tree diagram (see Figure 2).

![Figure 1: Prover](../images/prover.png)

![Figure 2: Recursion tree](../images/recursion.png)
