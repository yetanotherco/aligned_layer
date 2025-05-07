defmodule TelemetryApi.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    TelemetryApi.MetricsExporter.setup()

    children = [
      TelemetryApi.Traces,
      TelemetryApiWeb.Telemetry,
      TelemetryApi.Repo,
      {DNSCluster, query: Application.get_env(:telemetry_api, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: TelemetryApi.PubSub},
      # Start a worker by calling: TelemetryApi.Worker.start_link(arg)
      # {TelemetryApi.Worker, arg},
      # Start to serve requests, typically the last entry
      TelemetryApiWeb.Endpoint,
      TelemetryApi.Periodically
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: TelemetryApi.Supervisor]

    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    TelemetryApiWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
