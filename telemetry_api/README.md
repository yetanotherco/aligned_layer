# TelemetryApi

## Database Setup

To create a Postgres container and run it:

```shell
make telemetry_run_db
```

This will create and run the container using the credentials set in `Dockerfile`

### Delete database

If you want to delete the container:

```shell
make telemetry_remove_db_container
```

This will remove the container but will keep the storage

If you also want to delete the storage run:

```shell
make telemetry_clean_db
```

## Run Server

To start your Phoenix server:

```shell
make telemetry_start
```

  * Run `mix setup` to install and setup dependencies
  * Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.
