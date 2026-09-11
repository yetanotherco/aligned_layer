# Aggregator

> **Deprecated.** This page documents the Aligned Verification Layer, which is no
> longer active. It is kept for historical reference only. Aligned's active
> product is the [Proof Aggregation Service](../../../2_proof_aggregation_layer/architecture/1_overview.md).

The Aggregator collects [Operator](./4_operator.md)'s BLS Signatures. 

When the quorum of responses is reached, the Aggregator will submit a Task Response with the aggregated signatures back to the [Aligned Service Manager](./3_service_manager_contract.md).

