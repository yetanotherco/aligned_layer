defmodule AlignedLayerServiceManager do
  require Logger

  file_path =
    "../contracts/script/output/#{System.get_env("ENVIRONMENT")}/alignedlayer_deployment_output.json"

  {status, config_json_string} = File.read(file_path)

  case status do
    :ok -> Logger.debug("File read successfully")
    :error -> raise("Config file not read successfully, did you run make create-env ?")
  end

  @aligned_layer_service_manager_address Jason.decode!(config_json_string)
                                         |> Map.get("addresses")
                                         |> Map.get("alignedLayerServiceManager")

  use Ethers.Contract,
    abi_file: "lib/abi/AlignedLayerServiceManager.json",
    default_address: @aligned_layer_service_manager_address

  def get_aligned_layer_service_manager_address() do
    @aligned_layer_service_manager_address
  end

  def get_new_batch_events() do
    events =
      AlignedLayerServiceManager.EventFilters.new_batch(nil)
      |> Ethers.get_logs(fromBlock: 0)

    case events do
      {:ok, []} -> []
      {:ok, list} -> list
      {:error, _} -> raise("Error fetching events")
    end
  end

  def get_new_batch_events(merkle_root) when is_binary(merkle_root) do
    events =
      AlignedLayerServiceManager.EventFilters.new_batch(Utils.string_to_bytes32(merkle_root))
      |> Ethers.get_logs(fromBlock: 0)

    case events do
      {:error, reason} -> {:empty, reason}
      {_, []} -> {:empty, "No task found"}
      {:ok, event} -> extract_new_batch_event_info(event |> List.first())
    end
  end

  def get_new_batch_events(amount) when is_integer(amount) do

    events =
      AlignedLayerServiceManager.EventFilters.new_batch(nil)
      |> Ethers.get_logs(fromBlock: get_latest_block_number(-100), toBlock: get_latest_block_number())

    case events do
      {:ok, []} -> raise("Error fetching events, no events found")
      {:ok, list} -> Utils.get_last_n_items(list, amount)
      {:error, _} -> raise("Error fetching events")
    end

  end

  def get_latest_block_number() do
    {:ok, num} = Ethers.current_block_number()
    num
  end

  def get_latest_block_number(less) when is_integer(less) do
    {:ok, num} = Ethers.current_block_number()
    case num - abs(less) do #this allows passing negative number as param, which makes it easier to code
      r when r > 0 -> r
      r when r <= 0 -> 1
      _ -> raise("Error fetching latest block number")
    end
  end

  defp extract_new_batch_event_info(event) do
    data = event |> Map.get(:data)
    topics_raw = event |> Map.get(:topics_raw)

    # TODO verify this
    new_batch = %NewBatchEvent{
      batchMerkleRoot: topics_raw |> Enum.at(1),
      taskCreatedBlock: data |> Enum.at(0),
      batchDataPointer: data |> Enum.at(1)
    }

    {:ok,
     %NewBatchInfo{
       address: event |> Map.get(:address),
       block_hash: event |> Map.get(:block_hash),
       block_number: event |> Map.get(:block_number),
       transaction_hash: event |> Map.get(:transaction_hash),
       new_batch: new_batch
     }}
  end

  def get_batch_verified_events() do
    events =
      AlignedLayerServiceManager.EventFilters.batch_verified(nil) |> Ethers.get_logs(fromBlock: 0)

    case events do
      {:ok, []} -> raise("Error fetching responded events, no events found")
      {:ok, list} -> {:ok, list}
      {:error, data} -> raise("Error fetching responded events #{data}")
    end
  end

  def get_batch_verified_events(merkle_root) do
    events =
      AlignedLayerServiceManager.EventFilters.batch_verified(merkle_root)
      |> Ethers.get_logs(fromBlock: 0)

    case events do
      {:error, reason} -> {:empty, reason}
      {_, []} -> {:empty, "No task found"}
      {:ok, event} -> extract_batch_verified_event_info(event |> List.first())
    end
  end

  defp extract_batch_verified_event_info(event) do
    data = event |> Map.get(:data) |> List.first()

    # TODO verify this
    batch_verified = %BatchVerifiedEvent{
      batchMerkleRoot: data |> elem(0)
    }

    {:ok,
     %BatchVerifiedInfo{
       address: event |> Map.get(:address),
       block_hash: event |> Map.get(:block_hash),
       block_number: event |> Map.get(:block_number),
       transaction_hash: event |> Map.get(:transaction_hash),
       batch_verified: batch_verified
     }}
  end

  def is_batch_responded(merkle_root) do
    case AlignedLayerServiceManager.batches_state(Utils.string_to_bytes32(merkle_root))
         |> Ethers.call() do
      {:ok, [_, true]} -> true
      _ -> false
    end
  end

  def get_latest_batches() do
    AlignedLayerServiceManager.EventFilters.new_batch(nil)
  end

  # previous version: get_latest_task_index
  # TODO
  # def get_latest_batch_merkle_root() do
  #   {status, data} =

  #   case status do
  #     :ok -> data
  #     :error -> raise("Error fetching latest task index: #{data}")
  #   end
  # end

  # TODO refactor to new arquitecture, rethink this
  # def get_tx_hash(id) do
  #   AlignedLayerServiceManager.task_hashes(id)
  #   |> Ethers.call()
  #   |> (fn {x, y} when x == :ok -> y end).()
  #   |> Base.encode16()
  #   |> String.downcase()
  #   |> (fn x -> "0x" <> x end).()
  # end

  # TODO refactor to new arquitecture, rethink this
  # maybe use new storage "batchesState"
  # def get_task_response(id) do
  #   {status, task_responses} = AlignedLayerServiceManager.task_responses(id) |> Ethers.call()

  #   case status do
  #     :ok -> Logger.debug("task_responses #{task_responses}")
  #     :error -> raise("Error fetching task_responses")
  #   end

  #   task_responses
  # end

  # TODO pagination : revise with new arquitecture
  # i will turn it off for now, to do a step-by-step refactor

  # def get_task_range(from_id, to_id) when from_id <= to_id do
  #   # TODO : get from config file:
  #   task_created_event_signature =
  #     "0x1210195ebf465da0c87970f5e00248cd12b410335543e3ef555a0737f584ddd6"

  #   task_responded_event_signature =
  #     "0x8093f568fedd692803418ecdd966ebda93313efa011b6af02d1e54625b17d728"

  #   task_created_events =
  #     get_logs_with_range(task_created_event_signature, from_id, to_id)
  #     |> encode_logs("NewTaskCreated")
  #     |> Enum.map(fn event -> extract_events_info(event) end)

  #   # task_created_events = Enum.map(task_created_events, fn event -> extract_events_info(event) end )
  #   # "task_created_events" |> IO.inspect()
  #   # task_created_events |> IO.inspect()

  #   task_responded_events = get_logs_with_range(task_responded_event_signature, from_id, to_id)
  #   # |> encode_logs("TaskResponded")
  #   # "task_responded_events" |> IO.inspect()
  #   # task_responded_events |> IO.inspect()
  #   # task_responded_events = ""

  #   [task_created_events, task_responded_events]
  # end

  # defp get_logs_with_range(event_signature, from_id, to_id) do
  #   # TODO get from config file
  #   rpc_url = "http://localhost:8545"

  #   indexes =
  #     for n <- from_id..to_id, do: "0x#{String.pad_leading(Integer.to_string(n, 16), 64, "0")}"

  #   event_filter = %{
  #     fromBlock: "0x1",
  #     # TODO get from config file
  #     address: "0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690",
  #     topics: [
  #       event_signature,
  #       indexes
  #     ]
  #   }

  #   # cant make a custom rpc filter and send to |> Ethers.get_logs()
  #   alias Ethereumex.HttpClient
  #   {status, events} = HttpClient.eth_get_logs(event_filter, url: rpc_url)

  #   events =
  #     case status do
  #       :error ->
  #         raise("Error fetching task_created_events")

  #       :ok ->
  #         events
  #     end
  # end

  # def encode_logs(events, event_name) do
  #   abi = load_abi()
  #   selector = get_event_selector(event_name, abi)
  #   decode_logs(events, selector)
  # end

  # defp decode_logs(events, selector) do
  #   Enum.map(events, fn event -> Ethers.Event.decode(event, selector) end)
  # end

  # defp get_event_selector(event_name, abi) do
  #   event =
  #     Enum.find(abi, fn entry -> entry["type"] == "event" and entry["name"] == event_name end)

  #   case event_name do
  #     "NewTaskCreated" ->
  #       %ABI.FunctionSelector{
  #         type: :event,
  #         function: "NewTaskCreated",
  #         input_names: ["taskIndex", "task"],
  #         inputs_indexed: [true, false],
  #         method_id: <<0x1210195EBF465DA0C87970F5E00248CD12B410335543E3EF555A0737F584DDD6::256>>,
  #         types: [
  #           {:uint, 32},
  #           {
  #             :tuple,
  #             [
  #               {:uint, 16},
  #               {
  #                 :tuple,
  #                 [
  #                   {:uint, 8},
  #                   :bytes,
  #                   {:uint, 64}
  #                 ]
  #               },
  #               :bytes,
  #               :bytes,
  #               {:uint, 32},
  #               :bytes,
  #               :bytes,
  #               {:uint, 256}
  #             ]
  #           }
  #         ]
  #       }

  #     "TaskResponded" ->
  #       %ABI.FunctionSelector{
  #         type: :event,
  #         function: "TaskResponded",
  #         input_names: ["taskIndex", "taskResponse"],
  #         inputs_indexed: [true, false],
  #         method_id: <<0x8093F568FEDD692803418ECDD966EBDA93313EFA011B6AF02D1E54625B17D728::256>>,
  #         types: [
  #           {:uint, 32},
  #           {
  #             :tuple,
  #             [
  #               {:uint, 32},
  #               {:bool}
  #             ]
  #           }
  #         ]
  #       }
  #   end
  # end

  # defp load_abi() do
  #   file_path = "lib/abi/AlignedLayerServiceManager.json"
  #   Jason.decode!(File.read!(file_path))
  # end
end
