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
    default_address:
      Jason.decode!(config_json_string)
      |> Map.get("addresses")
      |> Map.get("alignedLayerServiceManager")

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
    if not is_integer(task_id) do
      {:empty, "task_id must be an integer"}
    end

    events =
      AlignedLayerServiceManager.EventFilters.new_task_created(task_id)
      |> Ethers.get_logs(fromBlock: 0)

    case events do
      {:error, reason} -> {:empty, reason}
      {_, []} -> {:empty, "No task found"}
      {:ok, event} -> extract_events_info(event |> List.first())
    end
  end

  defp extract_events_info(event) do
    data = event |> Map.get(:data) |> List.first()

    da_payload = %Explorer.DAPayload{
      # int
      solution: data |> elem(1) |> elem(0),
      # bytes
      proof_associated_data: data |> elem(1) |> elem(1),
      # uint64
      index: data |> elem(1) |> elem(2)
    }

    <<quorum_number::8>> = data |> elem(5)
    <<quorumThresholdPercentages::8>> = data |> elem(6)

    task = %Explorer.AlignedTask{
      # int
      provingSystemId: data |> elem(0),
      # %DAPayload{},
      da_payload: da_payload,
      # int
      pubInput: data |> elem(2),
      # bytes
      verificationKey: data |> elem(3),
      # uint32
      taskCreatedBlock: data |> elem(4),
      # bytes
      quorumNumbers: quorum_number,
      # bytes
      quorumThresholdPercentages: quorumThresholdPercentages,
      # uint256
      fee: data |> elem(7)
    }

    {:ok,
     %AlignedTaskCreatedInfo{
       address: event |> Map.get(:address),
       block_hash: event |> Map.get(:block_hash),
       block_number: event |> Map.get(:block_number),
       taskId: event |> Map.get(:topics) |> Enum.at(1),
       transaction_hash: event |> Map.get(:transaction_hash),
       aligned_task: task
     }}
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
      Logger.debug("No task response found, id #{task_id}")
      {:empty, "No task response found"}
    end
  end

  def get_latest_task_index() do
    {status, data} =
      AlignedLayerServiceManager.latest_task_index_plus_one() |> Ethers.call()

    case status do
      :ok -> data
      :error -> raise("Error fetching latest task index: #{data}")
    end
  end

  def get_tx_hash(id) do
    AlignedLayerServiceManager.task_hashes(id)
    |> Ethers.call()
    |> (fn {x, y} when x == :ok -> y end).()
    |> Base.encode16()
    |> String.downcase()
    |> (fn x -> "0x" <> x end).()
  end

  def get_task_response(id) do
    {status, task_responses} = AlignedLayerServiceManager.task_responses(id) |> Ethers.call()

    case status do
      :ok -> Logger.debug("task_responses #{task_responses}")
      :error -> raise("Error fetching task_responses")
    end

    task_responses
  end

  def get_task_responded_events() do
    {status, data} =
      AlignedLayerServiceManager.EventFilters.task_responded(nil) |> Ethers.get_logs(fromBlock: 0)

    case {status, data} do
      {:ok, []} -> raise("Error fetching responded events, no events found")
      {:ok, list} -> list
      {:error, data} -> raise("Error fetching responded events #{data}")
    end
  end

  def get_tasks_created_events() do
    {status, data} =
      AlignedLayerServiceManager.EventFilters.new_task_created(nil)
      |> Ethers.get_logs(fromBlock: 0)

    case {status, data} do
      {:ok, []} -> raise("Error fetching events, no events found")
      {:ok, list} -> list
      {:error, _} -> raise("Error fetching events")
    end
  end

  def get_task_range(from_id, to_id) when from_id <= to_id do
    # TODO : get from config file:
    task_created_event_signature =
      "0x1210195ebf465da0c87970f5e00248cd12b410335543e3ef555a0737f584ddd6"

    task_responded_event_signature =
      "0x8093f568fedd692803418ecdd966ebda93313efa011b6af02d1e54625b17d728"

    task_created_events =
      get_logs_with_range(task_created_event_signature, from_id, to_id)
      |> encode_logs("NewTaskCreated")
      |> Enum.map(fn event -> extract_events_info(event) end)

    # task_created_events = Enum.map(task_created_events, fn event -> extract_events_info(event) end )
    # "task_created_events" |> IO.inspect()
    # task_created_events |> IO.inspect()

    task_responded_events = get_logs_with_range(task_responded_event_signature, from_id, to_id)
    # |> encode_logs("TaskResponded")
    # "task_responded_events" |> IO.inspect()
    # task_responded_events |> IO.inspect()
    # task_responded_events = ""

    [task_created_events, task_responded_events]
  end

  defp get_logs_with_range(event_signature, from_id, to_id) do
    # TODO get from config file
    rpc_url = "http://localhost:8545"

    indexes =
      for n <- from_id..to_id, do: "0x#{String.pad_leading(Integer.to_string(n, 16), 64, "0")}"

    event_filter = %{
      fromBlock: "0x1",
      # TODO get from config file
      address: "0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690",
      topics: [
        event_signature,
        indexes
      ]
    }

    # cant make a custom rpc filter and send to |> Ethers.get_logs()
    alias Ethereumex.HttpClient
    {status, events} = HttpClient.eth_get_logs(event_filter, url: rpc_url)

    events =
      case status do
        :error ->
          raise("Error fetching task_created_events")

        :ok ->
          events
      end
  end

  def encode_logs(events, event_name) do
    abi = load_abi()
    selector = get_event_selector(event_name, abi)
    decode_logs(events, selector)
  end

  defp decode_logs(events, selector) do
    Enum.map(events, fn event -> Ethers.Event.decode(event, selector) end)
  end

  defp get_event_selector(event_name, abi) do
    event =
      Enum.find(abi, fn entry -> entry["type"] == "event" and entry["name"] == event_name end)

    case event_name do
      "NewTaskCreated" ->
        %ABI.FunctionSelector{
          type: :event,
          function: "NewTaskCreated",
          input_names: ["taskIndex", "task"],
          inputs_indexed: [true, false],
          method_id: <<0x1210195EBF465DA0C87970F5E00248CD12B410335543E3EF555A0737F584DDD6::256>>,
          types: [
            {:uint, 32},
            {
              :tuple,
              [
                {:uint, 16},
                {
                  :tuple,
                  [
                    {:uint, 8},
                    :bytes,
                    {:uint, 64}
                  ]
                },
                :bytes,
                :bytes,
                {:uint, 32},
                :bytes,
                :bytes,
                {:uint, 256}
              ]
            }
          ]
        }

      "TaskResponded" ->
        %ABI.FunctionSelector{
          type: :event,
          function: "TaskResponded",
          input_names: ["taskIndex", "taskResponse"],
          inputs_indexed: [true, false],
          method_id: <<0x8093F568FEDD692803418ECDD966EBDA93313EFA011B6AF02D1E54625B17D728::256>>,
          types: [
            {:uint, 32},
            {
              :tuple,
              [
                {:uint, 32},
                {:bool}
              ]
            }
          ]
        }
    end
  end

  defp load_abi() do
    file_path = "lib/abi/AlignedLayerServiceManager.json"
    Jason.decode!(File.read!(file_path))
  end
end
