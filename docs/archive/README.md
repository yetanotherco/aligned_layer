# Archive — Aligned Verification Layer

> **The Aligned Verification Layer is deprecated.**
> The pages in this folder are kept for historical reference only. They are no
> longer maintained, and the services, contracts and CLI they describe should not
> be used for new integrations.

Aligned's active ZK verification product is the
[Proof Aggregation Service](../2_proof_aggregation_layer/architecture/1_overview.md). To submit
proofs, see the [Proof Aggregation Service guide](../2_proof_aggregation_layer/guides/5_integration_example.md),
the [Aggregation Mode CLI](../2_proof_aggregation_layer/guides/3_cli.md) and the
[Aggregation Mode SDK](../2_proof_aggregation_layer/guides/4_sdk.md).

These pages are not listed in `SUMMARY.md`, so they do not appear in the
documentation navigation.

## Contents

### Architecture

- [Proof Verification Layer](2_architecture/1_proof_verification_layer.md) — overall architecture of the deprecated AVS
- [Batcher](2_architecture/components/1_batcher.md) — batched proofs into verification tasks
- [Payment Service Contract](2_architecture/components/2_payment_service_contract.md) — `BatcherPaymentService`
- [Service Manager Contract](2_architecture/components/3_service_manager_contract.md) — `AlignedLayerServiceManager`
- [Operator](2_architecture/components/4_operator.md) — EigenLayer restaker that re-executed proof verification
- [Aggregator](2_architecture/components/5_aggregator.md) — BLS signature aggregator (not proof aggregation)
- [Explorer](2_architecture/components/6_explorer.md) — the batches, operators and restaking views

### Guides

- [Submitting proofs](3_guides/0_submitting_proofs.md)
- [SDK intro](3_guides/1_SDK_how_to.md) and [Verification Layer SDK reference](3_guides/1.2_SDK_api_reference.md)
- [Verification Layer CLI](3_guides/10_aligned_cli.md)
- [Build your first Aligned application](3_guides/2_build_your_first_aligned_application.md) and [Modify ZkQuiz questions](3_guides/2.2_modify_zkquiz_questions.md)
- [Validating public input](3_guides/3_validating_public_input.md)
- [Generating a Gnark proof](3_guides/3.2_generate_gnark_proof.md) — Gnark proofs cannot be submitted to the Proof Aggregation Service
- [Using zkRust](3_guides/5_using_zkrust.md) — `--submit-to-aligned` targets the Verification Layer
- [Setup Aligned infrastructure locally](3_guides/6_setup_aligned.md)
- [Submitting a batch without the Batcher](3_guides/9_submitting_batch_without_batcher.md)

### Operators

The Verification Layer's operator set is no longer active and is not accepting
new operators.

- [Running an operator](operator_guides/0_running_an_operator.md)
- [Operator FAQ](operator_guides/1_operator_FAQ.md)
- [Troubleshooting](operator_guides/2_troubleshooting.md)
- Upgrading guides: [v0.14.0](operator_guides/upgrading_guides/v0_14_0.md), [v0.10.2](operator_guides/upgrading_guides/v0_10_2.md), [v0.9.2](operator_guides/upgrading_guides/v0_9_2.md)

### Introduction

- [ZkQuiz example app](1_introduction/2_example_quiz_app.md)

### Superseded pages

The `about_aligned/` folder holds an earlier version of the introductory
documentation, written when the Verification Layer was called "fast mode". It was
superseded by `1_introduction/` and was already unlisted before being archived.
