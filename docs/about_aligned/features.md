# Features

## Expanding capacities

With Aligned, we are expanding Ethereum's ZK capabilities to include an interesting variety of proof systems in the Ethereum ecosystem. We want to offer the infrastructure for the future of trustless applications using verifiable computation and Ethereum's security.

## Neutrality

We are not in favor of any specific type of SNARK, and we support anyone involved in zero-knowledge technology development or research. Our focus is on supporting the development and research community involved in various proof systems.

## Innovation

We are helping foster open innovation in Ethereum by avoiding the constraints of the EVM, which enables us to introduce a new infrastructure to accelerate the roadmap for Ethereum and facilitate the integration of new technologies without modifying the underlying base protocol.

## Reduced costs

The costs for proof verification consist of the following:
- Proof and public input data storage.
- Running the proof verification algorithm.
- Reading and using the result from the verification.
The cost on Ethereum depends on the amount of data and computation required, the network congestion, and the valuation of ETH. The computational effort is measured in gas. For reference, the cost of a transaction in Ethereum is 21,000 gas.

To transform to USD, we need to multiply the gas cost by the gas cost expressed in gwei ($10^{-9}$ ETH) and the conversion rate between ETH and USD:
$$C_{USD} = C_{gas} V_{gas} V_{ETH}$$

For example, if the gas cost is 8 gwei/gas and ETH is worth 3000 USD/ETH, a transaction costs:
$$C_{USD} = 21,000 \times 8 \times 10^{-9} \times 3000 = 0.504\ \mathrm{USD}$$

Aligned reduces cost by splitting the cost of task creation and verification among several proofs. The gas cost per proof for a batch containing N proofs is:
$$C_{gas} = \frac{C_{task} + C_{verification}}{N} + C_{read}$$

$C_{read}$ in this case is the cost the user has to pay to use the proof in a contract.

The cost of verification depends on the mode chosen when using Aligned. For fast mode only, the cost of verification is the cost of a BLS signature check. This cost is based on the calculation of elliptic curve pairings, which costs 113,000 gas. It is important to note that while batching can reduce the costs of task creation and verification, the reading cost is fixed.

## High throughput 

Aligned provides an alternative to significantly reduce costs and increase throughput. This is achieved by two different modes: the fast mode and the aggregation/slow mode. The fast mode works using a subset of Ethereum’s validators via restaking. Validators (also called Operators) receive proofs, verify them using the verification code written in Rust or another higher-level language, and sign messages with BLS signatures. If a two-thirds majority agrees, the results are posted to Ethereum. Since Aligned’s operators only need to run the verification code on bare metal, we have several advantages compared to running it on top of the EVM:
The code can be optimized for speed, not gas consumption.
We can leverage parallelization to increase throughput.
Since the gas limit does not constrain us, we can verify proof systems that are too expensive for Ethereum, such as Kimchi or [Binius](https://eprint.iacr.org/2023/1784).

Preliminary numbers show that Aligned can verify more than 1000 proofs per second using our fast mode architecture, over two orders of magnitude greater than the EVM at nominal capacity. Using effective batching techniques, we can split the task creation and verification cost between thousands of proofs, greatly reducing costs.
