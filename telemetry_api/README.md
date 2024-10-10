# TelemetryApi

## Database Setup

To create a Postgres container and run it:

```shell
make telemetry_run_db
```

This will create and run the container using the credentials set in `Dockerfile`

> [!CAUTION]
> Do not use default credentials in Production environments.

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

First you need to manually set the `telemetry_api/.env` file.

To use `telemetry_api/.env.dev`, run:

```shell
make telemetry_create_env
```

To start your Phoenix server:

```shell
make telemetry_start
```

On startup, it will run ecto migrations.

  * Run `mix setup` to install and setup dependencies
  * Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

## Database Migrations

This API uses Ecto for migrations. To apply migrations, run:

```shell
make telemetry_ecto_migrate
```


