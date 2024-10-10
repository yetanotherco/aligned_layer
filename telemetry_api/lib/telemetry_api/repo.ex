defmodule TelemetryApi.Repo do
  use Ecto.Repo,
    otp_app: :telemetry_api,
    adapter: Ecto.Adapters.Postgres
end
