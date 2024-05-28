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
      |> Ethers.get_logs(fromBlock: get_latest_block_number(-1000), toBlock: get_latest_block_number())

    case events do
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

  def extract_new_batch_event_info(event) do
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

  # def cross_event_with_response({status, %NewBatchInfo{new_batch: %NewBatchEvent{batchMerkleRoot: merkle_root}}}) do
  def cross_event_with_response({status, new_batch_info}) do
    new_batch = new_batch_info.new_batch
    # "new_batch_info" |> IO.inspect()
    # new_batch_info |> IO.inspect()
    batch_page_item = %BatchPageItem{
      batch_merkle_root: new_batch.batchMerkleRoot,
      task_created_block_number: new_batch.taskCreatedBlock,
      task_created_tx_hash: new_batch.batchDataPointer,
      task_responded_block_number: nil,
      task_responded_tx_hash: nil,
      batch_data_pointer: new_batch.batchDataPointer,
      responded: is_batch_responded(new_batch.batchMerkleRoot)
    }
  end
  
end
