# Docker for all components

To build the necessary images, first run:

```shell
make docker_build
```

Beware that this takes quite a bit of storage, so if you're using Docker for Mac, you're advised to increase to at least 100GB the "Virtual disk limit" size.

To run the whole stack, including initialization, run:

```shell
make docker_up
```

This will bring app the resources and run the common initialization steps.

If you want to bring it down, run:

```shell
make docker_down
```

If you want to rebuild any of the components, you can run either of these:

```shell
make docker_build_aggregator
```

```shell
make docker_build_operator
```

```shell
make docker_build_batcher
```

If you want to rebuild and then restart any of these components without bringing down the docker environment, just run either of these, after rebuilding:

```shell
make docker_restart_aggregator
```

```shell
make docker_restart_operator
```

```shell
make docker_restart_batcher
```

Alternatively, you can run `make docker_down`, then rebuild, and then `make docker_up` to start over with a fresh environment.

Additionally, you can run any of these to send proofs (burst of 2 each):

```shell
make docker_batcher_send_sp1_burst
```

```shell
make docker_batcher_send_risc0_burst
```

```shell
make docker_batcher_send_plonk_bn254_burst
```

```shell
make docker_batcher_send_plonk_bls12_381_burst
```

```shell
make docker_batcher_send_infinite_groth16
```

Or you can send all of them together with:

```shell
make docker_batcher_send_all_proofs_burst
```

To verify all sent proofs:

```shell
make docker_verify_proofs_onchain
```

And you can run this to attach to the anvil/foundry container and run `cast` with custom flags:

```shell
make docker_attach_foundry
```

## Logs

You can watch logs for the components with the following commands:

```shell
make docker_logs_anvil
```

```shell
make docker_logs_aggregator
```

```shell
make docker_logs_operator
```

```shell
make docker_logs_batcher
```
