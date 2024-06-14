import Config

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :explorer, ExplorerWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4002],
  secret_key_base: "Aa/QmDjOvy3JD36Wdg0KT+9eIsWkephInTvV7sjKv4eOPXJ7z0+WT2cIVXT12Y/e",
  server: false

# Print only warnings and errors during test
config :logger, level: :warning

# Initialize plugs at runtime for faster test compilation
config :phoenix, :plug_init_mode, :runtime

config :phoenix_live_view,
  # Enable helpful, but potentially expensive runtime checks
  enable_expensive_runtime_checks: true
