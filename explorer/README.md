# Explorer

## Requirements

- [Erlang 26](https://github.com/asdf-vm/asdf-erlang)
- [Elixir 1.16.2](https://elixir-ko.github.io/install.html), compiled with OTP 26
- [Phoenix 1.7.12](https://hexdocs.pm/phoenix/installation.html)
- [Ecto 3.11.2](https://hexdocs.pm/ecto/getting-started.html)

## Local development

### Set up environment variables

Create a `.env` file in the `/explorer` directory of the project. The `.env` file needs to contain the following variables:

| Variable | Description |
| -------- | ----------- |
| `RPC_URL` | The RPC URL of the network you want to connect to. |
| `ENVIRONMENT` | The environment you want to run the application in. It can be `devnet`, `holesky` or `mainnet`. |

### Running the database

To run the database, you will need to have [docker](https://docs.docker.com/get-docker/).

Running Postgres via Docker is simple; just need run the following command:

```sh
docker run --name explorer-postgres -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5500:5432 -d postgres
```

### Run devnet environment (optional)

If you want to run the devnet environment, you can run the following command in another terminal:

```sh
make run_local
```

<details>
<summary>
    Or alternatively you can manually run the following commands:
</summary>

```sh
cd ..
make anvil_start
```

Then, in another terminal, run the following command to start the aggregator:

```sh
make aggregator_start
```

Again, in another terminal, you can run the following command to run the operator:

```sh
make operator_full_registration
make operator_start
```

Now we need to start the batcher by running the following command:

```sh
make batcher_start
```

Finally, to have a batch of tasks running in the devnet, you can run the following commands:

```sh
make batcher_send_sp1_task
make batcher_send_sp1_task
make batcher_send_sp1_task
make batcher_send_sp1_task
make batcher_send_sp1_task
make batcher_send_groth16_task
make batcher_send_groth16_task
make batcher_send_groth16_task
make batcher_send_groth16_task
make batcher_send_groth16_task
make batcher_send_sp1_task
make batcher_send_groth16_task
make batcher_send_sp1_task
make batcher_send_groth16_task
make batcher_send_sp1_task
```

This will send 10 SP1 tasks, 5 Groth16 tasks, and 5 SP1 tasks to the devnet.

In order to stop the devnet environment, you'll need to stop each of the services started in the previous steps.

</details>

### Run the frontend

To start your Phoenix server:

```makefile
cd explorer
make <run | deps | help>
```

| Command | Description |
| --- | --- |
| `make run` | Starts the Elixir backend server. |
| `make deps` | Installs Elixir code dependencies. |
| `make help` | Shows the help message. |

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.
You can access to a tasks information by visiting `localhost:4000/tasks/:id`.

### Upgrade Aligned ABI

Keep in mind when the contracts are updated, the ABI of the contracts must be updated in the frontend.

If you want to update the ABI of the contracts, you can run the following commands:

```bash
cd ..
make build_aligned_contracts
cd contracts/out/
```

This will generate new contracts in the `/contracts/out/AlignedLayerServiceManager.sol` folder in the root of the project.

Once in there copy the contents of the file after `{"abi":` and before `,"bytecode":{"object":`.

Once copied, either create or replace the `AlignedLayerServiceManager.json` file, located in the `/explorer/lib/abi` directory, by pasting the content from your clipboard.

Lastly, repeat the same process for the AVSDirectory contract.

Last updated ABI versions:

- AlignedLayerServiceManager: v0.8.24
- AVSDirectory: v0.8.24

## Contributing

We appreciate your interest in contributing to the Aligned Explorer! Your contributions can help make this project even better.

PRs are more than welcome if you want to collaborate to the project. If you don't know how to implement a feature, you are still welcome to create an issue and don't forget to add the `frontend` label!

### Get in Touch

If you have any questions, suggestions, or if you'd like to contribute in any way, please feel free to reach out to us:

- **Discord**: [Aligned](https://discord.gg/alignedlayer)
- **GitHub Issues**: [Open an Issue](https://github.com/yetanotherco/aligned_layer/labels/frontend)
