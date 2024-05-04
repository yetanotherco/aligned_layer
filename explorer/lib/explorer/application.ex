defmodule Explorer.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application
  require Logger

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

# called AlignedTask since Task is a reserved word in Elixir
defmodule AlignedTask do
  @enforce_keys [:verificationSystemId, :proof, :pubInput, :taskCreatedBlock]
  defstruct [:verificationSystemId, :proof, :pubInput, :taskCreatedBlock]
end

defmodule AlignedTaskCreatedInfo do
  @enforce_keys [:address, :block_hash, :block_number, :taskId, :transaction_hash, :aligned_task]
  defstruct [:address, :block_hash, :block_number, :taskId, :transaction_hash, :aligned_task]
end

defmodule AlignedTaskRespondedInfo do
  @enforce_keys [
    :address,
    :block_hash,
    :block_number,
    :taskId,
    :transaction_hash,
    :proofIsCorrect
  ]
  defstruct [:address, :block_hash, :block_number, :taskId, :transaction_hash, :proofIsCorrect]
end

defmodule AlignedLayerServiceManager do
  require Logger
  # read alignedLayerServiceManagerAddress from config file
  {:ok, json_string} =
    File.read("../contracts/script/output/holesky/alignedlayer_deployment_output.json")

  alignedLayerServiceManagerAddress =
    Jason.decode!(json_string) |> Map.get("addresses") |> Map.get("alignedLayerServiceManager")

  use Ethers.Contract,
    abi_file: "lib/abi/AlignedLayerServiceManager.json",
    # devnet
    default_address: alignedLayerServiceManagerAddress

  # default_address: "0x2fcE68A46aF645A00D0b94C2db48f627040766A7" #holesky

  def get_task_created_event(task_id) do
    # check if task_id is a valid integer
    if not is_integer(task_id) do
      {:empty, "task_id must be an integer"}
    end

    events =
      AlignedLayerServiceManager.EventFilters.new_task_created(task_id)
      |> Ethers.get_logs(fromBlock: 0)

    #  struct Task {
    #     uint16 provingSystemId;
    #     bytes proof;
    #     bytes pubInput;
    #     bytes verificationKey;
    #     uint32 taskCreatedBlock;
    #     bytes quorumNumbers;
    #     bytes quorumThresholdPercentages;
    #     uint256 fee;
    # }

    Logger.debug("Events from #{task_id}: #{inspect(events)}")

    # extract relevant info from RPC response
    if not (events |> elem(1) |> Enum.empty?()) do
      address = events |> elem(1) |> List.first() |> Map.get(:address)
      block_hash = events |> elem(1) |> List.first() |> Map.get(:block_hash)
      block_number = events |> elem(1) |> List.first() |> Map.get(:block_number)
      taskId = events |> elem(1) |> List.first() |> Map.get(:topics) |> Enum.at(1)
      transaction_hash = events |> elem(1) |> List.first() |> Map.get(:transaction_hash)

      {verificationSystemId, proof, pubInput, taskCreatedBlock} =
        events |> elem(1) |> List.first() |> Map.get(:data) |> List.first()

      # struct TaskResponse {
      #   uint32 taskIndex;
      #   bool proofIsCorrect;
      # }

      task = %AlignedTask{
        verificationSystemId: verificationSystemId,
        proof: proof,
        pubInput: pubInput,
        taskCreatedBlock: taskCreatedBlock
      }

      {:ok,
       %AlignedTaskCreatedInfo{
         address: address,
         block_hash: block_hash,
         block_number: block_number,
         taskId: taskId,
         transaction_hash: transaction_hash,
         aligned_task: task
       }}
    else
      {:empty, "No task found"}
    end
  end

  def get_task_responded_event(task_id) do
    events =
      AlignedLayerServiceManager.EventFilters.task_responded(task_id)
      |> Ethers.get_logs(fromBlock: 0)

    # extract relevant info from RPC response
    if not (events |> elem(1) |> Enum.empty?()) do
      address = events |> elem(1) |> List.first() |> Map.get(:address)
      block_hash = events |> elem(1) |> List.first() |> Map.get(:block_hash)
      block_number = events |> elem(1) |> List.first() |> Map.get(:block_number)
      transaction_hash = events |> elem(1) |> List.first() |> Map.get(:transaction_hash)

      {taskIndex, proofIsCorrect} =
        events |> elem(1) |> List.first() |> Map.get(:data) |> List.first()

      {:ok,
       %AlignedTaskRespondedInfo{
         address: address,
         block_hash: block_hash,
         block_number: block_number,
         taskId: taskIndex,
         transaction_hash: transaction_hash,
         proofIsCorrect: proofIsCorrect
       }}
    else
      {:empty, "No task response found"}
    end
  end
end
