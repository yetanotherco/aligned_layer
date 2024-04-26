defmodule Explorer.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      ExplorerWeb.Telemetry,
      {DNSCluster, query: Application.get_env(:explorer, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Explorer.PubSub},
      # Start the Finch HTTP client for sending emails
      {Finch, name: Explorer.Finch},
      # Start a worker by calling: Explorer.Worker.start_link(arg)
      # {Explorer.Worker, arg},
      # Start to serve requests, typically the last entry
      ExplorerWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Explorer.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    ExplorerWeb.Endpoint.config_change(changed, removed)
    :ok
  end

end

defmodule MyERC20Token do
  use Ethers.Contract,
    abi_file: "lib/abi/UriCoin.json",
    default_address: "0x206f772c702D4B249F153853a4c94b071f98AA58"

  def get_erc20_name do
    MyERC20Token.name() |> Ethers.call()
  end
end
