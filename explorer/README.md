# Explorer

## Requirements

- Erlang 26
- Elixir 1.16.2, compiled with OTP 26

## Local development

To start your Phoenix server:

```makefile
$ make
Usage:
    run   : Starts the Elixir backend server.
    deps  : Installs Elixir code dependencies.
```

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.
You can access to a tasks information by visiting `localhost:4000/tasks/:id`.

Once the frontend is running, you must run a local devnet with the deployed contracts:
```bash
make anvil-start
```

Then, you must send a Task to view it in the explorer:
```bash
make send-plonk-proof
```

You can also respond to a task as an operator, saying if the proof was true or false:
```bash
cast send 0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690 --rpc-url "http://localhost:8545" "respondToTask(uint64, bool)()" <num_task_id> <boolean_is_proof_correct> --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

## Contributing

We appreciate your interest in contributing to the Aligned Explorer! Your contributions can help make this project even better.

PRs are more than welcome if you want to collaborate to the project. If you don't know how to implement a feature, you are still welcome to create an issue and don't forget to add the `frontend` label!

### Get in Touch

If you have any questions, suggestions, or if you'd like to contribute in any way, please feel free to reach out to us:

- **Telegram**: [Get Aligned](https://t.me/alignedlayer)
- **GitHub Issues**: [Open an Issue](https://github.com/yetanotherco/aligned_layer/labels/frontend)
