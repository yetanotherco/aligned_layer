# Aligned contract addresses

The contracts below back Aligned's Proof Aggregation Service.

- `AlignedProofAggregationService` — receives aggregated proofs and answers proof inclusion checks.
- `AggregationModePaymentService` — receives deposits that fund your proof submission quota.

## Mainnet Deployments

| Contract                       | Address                                                                                                               |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| AlignedProofAggregationService | [0xD0696d3eEebffcAB2D1b358805efAA005A9A8BC0](https://etherscan.io/address/0xD0696d3eEebffcAB2D1b358805efAA005A9A8BC0) |
| AggregationModePaymentService  | [0xc8631Bc1E60c20db40e474F791126212fA8255F4](https://etherscan.io/address/0xc8631Bc1E60c20db40e474F791126212fA8255F4) |

## Hoodi Deployments

| Contract                       | Address                                                                                                                     |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
| AlignedProofAggregationService | [0x6B34AAaE780A5EAB4c91AB8F54f2a421E9c2FB59](https://hoodi.etherscan.io/address/0x6B34AAaE780A5EAB4c91AB8F54f2a421E9c2FB59) |
| AggregationModePaymentService  | [0xe6C9D0cf87cdaA8B2093c4b3830dde7267843F64](https://hoodi.etherscan.io/address/0xe6C9D0cf87cdaA8B2093c4b3830dde7267843F64) |

## Sepolia Deployments

| Contract                       | Address                                                                                                                       |
|--------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| AlignedProofAggregationService | [0xb5D46304c30B1AeB3a8Da6ab599c336f7946C8A4](https://sepolia.etherscan.io/address/0xb5D46304c30B1AeB3a8Da6ab599c336f7946C8A4) |

{% hint style="warning" %}
The CLI and SDK do not support Sepolia. The `--network` option accepts `devnet`, `hoodi` and `mainnet` only, and there is no Aggregation Mode Gateway deployed for Sepolia.
{% endhint %}

## Gateway URLs

Proofs are submitted to the Aggregation Mode Gateway. The CLI and SDK pick the right URL from the `--network` option, so you normally do not need these directly.

| Network | Gateway URL                                |
|---------|--------------------------------------------|
| Mainnet | `https://mainnet.gateway.alignedlayer.com` |
| Hoodi   | `https://hoodi.gateway.alignedlayer.com`   |
| Devnet  | `http://127.0.0.1:8089`                    |
