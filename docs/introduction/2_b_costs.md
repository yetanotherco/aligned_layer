# Costs

## Current on chain verification costs

The verification of zero-knowledge proofs in Ethereum depends on the proof system used, proof size and additional data needed to verify such proofs. Some popular choices are Groth16, Plonk and STARKs. The cost to verify a proof (expressed in USD) depends both on the ether (ETH) to dollar conversion and the gas cost at a given time. The table below summarizes the gas cost and the cost in USD, assuming that the gas cost is 25 gwei/gas and ETH is worth 3,000 USD. These are just estimates, and the cost can fluctuate significantly, depending on network congestion and variations in the value of ETH.

| Proof system | Gas cost    | Cost in USD |
| --------     | --------    | --------    |
| Groth16      | 250,000     | 18.75       |
| Plonk/KZG    | 450,000     | 30.00       |
| STARKs       | >1,000,000  | >75         |

Note that these costs can vary depending on variants in the proof system (such as supporting lookup arguments or customized gates), the implementation of the smart contract in Ethereum, as well as the factors mentioned above. The annualized costs depend on the number of proofs the application needs to verify in one year. These costs just refer to the verification and do not take into account other costs the project incurs in Ethereum.

## Costs in Aligned

The costs depend on task creation, aggregated signature or proof verification, and reading the results. The cost C per proof by batching N proofs is roughly:
    
$$
  C =\frac{C_{task} + C_{verification}}{N} + C_{read}
$$

The cost in USD will be obtained by multiplying this cost $C$ by the gas cost at the time of the verification, and the conversion factor from Ether to USD.

It is important to note that this cost is independent of the proof system used and of the proof size.

## Savings