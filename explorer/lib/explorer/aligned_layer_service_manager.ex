defmodule AlignedLayerServiceManager do
  require Logger

  file_path =
    "../contracts/script/output/#{System.get_env("ENVIRONMENT")}/alignedlayer_deployment_output.json"

  {status, config_json_string} = File.read(file_path)

  case status do
    :ok -> Logger.debug("File read successfully")
    :error -> raise("Config file not read successfully, did you run make create-env ?")
  end

  use Ethers.Contract,
  abi_file: "lib/abi/AlignedLayerServiceManager.json",
  default_address: Jason.decode!(config_json_string) |> Map.get("addresses") |> Map.get("alignedLayerServiceManager")

  def get_aligned_layer_service_manager_address() do
    file_path =
      "../contracts/script/output/#{System.get_env("ENVIRONMENT")}/alignedlayer_deployment_output.json"

    {status, config_json_string} = File.read(file_path)

    case status do
      :ok -> Logger.debug("File read successfully")
      :error -> raise("Config file not read successfully, did you run make create-env ?")
    end

    Jason.decode!(config_json_string)
      |> Map.get("addresses")
      |> Map.get("alignedLayerServiceManager")
  end

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

  def get_latest_task_index() do
    {status, last_task_id} =
      AlignedLayerServiceManager.latest_task_index_plus_one() |> Ethers.call()

    case status do
      :ok -> Logger.debug("Latest task index: #{last_task_id}")
      :error -> raise("Error fetching latest task index")
    end

    last_task_id
  end

  def get_avs_directory() do
    {status, avs_directory} = AlignedLayerServiceManager.avs_directory() |> Ethers.call()

    case status do
      :ok -> Logger.debug("AVS directory #{avs_directory}")
      :error -> raise("Error fetching latest task index")
    end

    avs_directory
  end

  # def get_tx_hash(id) do
  #   {status, tx_hash} = AlignedLayerServiceManager.task_hashes(id) |> Ethers.call()
  #   case status do
  #     :ok -> Logger.debug("tx_hash #{tx_hash}")
  #     :error -> raise("Error fetching tx_hashes")
  #   end
  #   tx_hash |> Base.encode16 |> String.downcase |> (fn x -> "0x" <> x end).()
  # end

  def get_tx_hash(id) do
    AlignedLayerServiceManager.task_hashes(id)
    |> Ethers.call()
    |> (fn {x, y} when x == :ok -> y end).()
    |> Base.encode16()
    |> String.downcase()
    |> (fn x -> "0x" <> x end).()

    # {status, tx_hash} = AlignedLayerServiceManager.task_hashes(id) |> Ethers.call()
    # case status do
    #   :ok -> Logger.debug("tx_hash #{tx_hash}")
    #   :error -> raise("Error fetching tx_hashes")
    # end
    # tx_hash |> Base.encode16 |> String.downcase |> (fn x -> "0x" <> x end).()
  end

  def get_task_response(id) do
    {status, task_responses} = AlignedLayerServiceManager.task_responses(id) |> Ethers.call()

    status |> IO.inspect()
    task_responses |> IO.inspect()


    case status do
      :ok -> Logger.debug("task_responses #{task_responses}")
      :error -> raise("Error fetching task_responses")
    end

    task_responses
  end

  def get_task_responded_events() do
      AlignedLayerServiceManager.EventFilters.task_responded(nil) |> Ethers.get_logs(fromBlock: 0)
  end
end
