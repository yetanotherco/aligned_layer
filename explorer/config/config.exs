# This file is responsible for configuring your application
# and its dependencies with the aid of the Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
import Config

config :explorer,
  generators: [timestamp_type: :utc_datetime]

# Configures the endpoint
config :explorer, ExplorerWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [html: ExplorerWeb.ErrorHTML, json: ExplorerWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: Explorer.PubSub,
  live_view: [signing_salt: "XkOXIXZ0"]

# Configures the database
config :explorer,
  ecto_repos: [Explorer.Repo],
  env: Mix.env()

# Configure esbuild (the version is required)
config :esbuild,
  version: "0.17.11",
  explorer: [
    args:
      ~w(js/app.js --bundle --target=es2017 --outdir=../priv/static/assets --external:/fonts/* --external:/images/*),
    cd: Path.expand("../assets", __DIR__),
    env: %{"NODE_PATH" => Path.expand("../deps", __DIR__)}
  ]

# Configure tailwind (the version is required)
config :tailwind,
  version: "3.4.0",
  explorer: [
    args: ~w(
      --config=tailwind.config.js
      --input=css/app.css
      --output=../priv/static/assets/app.css
    ),
    cd: Path.expand("../assets", __DIR__)
  ]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Configures Ethers, to interact with Ethereum contracts
config :ethers,
  # Defaults to: Ethereumex.HttpClient
  rpc_client: Ethereumex.HttpClient,
  # Defaults to: ExKeccak
  keccak_module: ExKeccak,
  # Defaults to: Jason
  json_module: Jason,
  # Defaults to: ExSecp256k1
  secp256k1_module: ExSecp256k1,
  # Defaults to: nil, see Ethers.Signer for more info
  default_signer: nil,
  # Defaults to: []
  default_signer_opts: []

# Using Ethereumex, you can specify a default JSON-RPC server url here for all requests.
config :ethereumex,
  url: System.get_env("RPC_URL")

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{config_env()}.exs"
