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

# called AlignedTask since Task is a reserved word in Elixir
defmodule AlignedTask do
  @enforce_keys [
    :verificationSystemId,
    # :proof,
    :pubInput,
    :taskCreatedBlock
  ]
  defstruct [
    :verificationSystemId,
    # :proof,
    :pubInput,
    :taskCreatedBlock
  ]
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
  file_path =
    "../contracts/script/output/#{System.get_env("ENVIRONMENT")}/alignedlayer_deployment_output.json"

  Logger.debug(file_path)

  {status, config_json_string} = File.read(file_path)

  case status do
    :ok -> Logger.debug("File read successfully")
    :error -> raise("Config file not read successfully, did you run make create-env ?")
  end

  alignedLayerServiceManagerAddress =
    Jason.decode!(config_json_string)
    |> Map.get("addresses")
    |> Map.get("alignedLayerServiceManager")

  use Ethers.Contract,
    abi_file: "lib/abi/AlignedLayerServiceManager.json",
    # devnet
    default_address: alignedLayerServiceManagerAddress

  def get_task_created_event(task_id) do
    # check if task_id is a valid integer
    if not is_integer(task_id) do
      {:empty, "task_id must be an integer"}
    end

    events =
      AlignedLayerServiceManager.EventFilters.new_task_created(task_id)
      |> Ethers.get_logs(fromBlock: 0)

    # extract relevant info from RPC response
    if not (events |> elem(1) |> Enum.empty?()) do
      first_event = events |> elem(1) |> List.first()
      Logger.debug("get_task_created_event -> event #{task_id}: #{inspect(first_event)}")
      address = first_event |> Map.get(:address)
      block_hash = first_event |> Map.get(:block_hash)
      block_number = first_event |> Map.get(:block_number)
      taskId = first_event |> Map.get(:topics) |> Enum.at(1)
      transaction_hash = first_event |> Map.get(:transaction_hash)

      data = first_event |> Map.get(:data) |> List.first()
      verificationSystemId = data |> elem(0)
      # proof = data |> elem(1)
      taskCreatedBlock = data |> elem(4)
      pubInput = data |> elem(6)

      task = %AlignedTask{
        verificationSystemId: verificationSystemId,
        # proof: proof,
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
      Logger.debug("No task found")
      {:empty, "No task found"}
    end
  end

  def get_task_responded_event(task_id) do
    events =
      AlignedLayerServiceManager.EventFilters.task_responded(task_id)
      |> Ethers.get_logs(fromBlock: 0)

    # extract relevant info from RPC response
    if not (events |> elem(1) |> Enum.empty?()) do
      first_event = events |> elem(1) |> List.first()
      address = first_event |> Map.get(:address)
      block_hash = first_event |> Map.get(:block_hash)
      block_number = first_event |> Map.get(:block_number)
      transaction_hash = first_event |> Map.get(:transaction_hash)

      {taskIndex, proofIsCorrect} = first_event |> Map.get(:data) |> List.first()

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
      Logger.debug("No task response found")
      {:empty, "No task response found"}
    end
  end
end
