defmodule Explorer.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      ExplorerWeb.Telemetry,
      {Cachex, name: :eth_price_cache},
      {DNSCluster, query: Application.get_env(:explorer, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Explorer.PubSub},
      # Start the Ecto db repository
      Explorer.Repo,
      # Start the Finch HTTP client for getting data from batch_data_pointer
      {Finch, name: Explorer.Finch},
      # Start a worker by calling: Explorer.Worker.start_link(arg)
      # {Explorer.Worker, arg},
      # Start to serve requests, typically the last entry
      ExplorerWeb.Endpoint
    ]

    # Start the periodic task, with its own supervisor and mutex
    opts = [strategy: :one_for_one, name: Explorer.Supervisor]
    Supervisor.start_link(children, opts)

    periodic_children = [
      {Explorer.Periodically, []},
      {Mutex, name: BatchMutex, meta: "Used to prevent concurrent downloads"}
    ]

    periodic_opts = [strategy: :one_for_all, name: Explorer.Periodically.Supervisor]
    Supervisor.start_link(periodic_children, periodic_opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    ExplorerWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
