defmodule AlignedLayerServiceManager do
  require Logger

  @environment System.get_env("ENVIRONMENT")
  @aligned_config_file System.get_env("ALIGNED_CONFIG_FILE")

  case @environment do
    "devnet" -> Logger.debug("Running on devnet")
    "holesky" -> Logger.debug("Running on holesky")
    "mainnet" -> Logger.debug("Running on mainnet")
    _ -> Logger.debug("Invalid ENVIRONMENT var in .env")
    nil -> raise("Invalid ENVIRONMENT var in .env")
  end

  config_file_path = case @aligned_config_file do
    nil -> raise("ALIGNED_CONFIG_FILE not set in .env")
    file -> file
  end

  {status, config_json_string} = File.read(config_file_path)

  case status do
    :ok -> Logger.debug("File read successfully")
    :error -> raise("Config file not read successfully, did you run make create-env? If you did,\n make sure Alignedlayer config file is correctly stored")
  end

  @aligned_layer_service_manager_address Jason.decode!(config_json_string)
                                         |> Map.get("addresses")
                                         |> Map.get("alignedLayerServiceManager")

  @first_block (
    case @environment do
      "devnet" -> 0
      "holesky" -> 1728056
      "mainnet" -> 20020000
      _ -> raise("Invalid environment")
    end
  )

  use Ethers.Contract,
    abi_file: "lib/abi/AlignedLayerServiceManager.json",
    default_address: @aligned_layer_service_manager_address

  def get_aligned_layer_service_manager_address() do
    @aligned_layer_service_manager_address
  end

  def get_latest_block_number() do
    {:ok, num} = Ethers.current_block_number()
    num
  end

  def get_new_batch_events(%{fromBlock: fromBlock, toBlock: toBlock}) do
    events =
      AlignedLayerServiceManager.EventFilters.new_batch(nil)
      |> Ethers.get_logs(fromBlock: fromBlock, toBlock: toBlock)

    case events do
      {:ok, []} -> []
      {:ok, list} -> Enum.map(list, &extract_new_batch_event_info/1)
      {:error, reason } -> raise("Error fetching events: #{Map.get(reason, "message")}")
    end
  end

  def extract_new_batch_event_info(event) do
    new_batch = parse_new_batch_event(event)
    {:ok,
     %NewBatchInfo{
       address: event |> Map.get(:address),
       block_number: event |> Map.get(:block_number),
       block_timestamp: get_block_timestamp(event |> Map.get(:block_number)),
       transaction_hash: event |> Map.get(:transaction_hash),
       new_batch: new_batch
     }}
  end

  def parse_new_batch_event(%Ethers.Event{} = new_batch_event) do
    data = new_batch_event |> Map.get(:data)
    topics_raw = new_batch_event |> Map.get(:topics_raw)

    %NewBatchEvent{
      batchMerkleRoot: topics_raw |> Enum.at(1),
      taskCreatedBlock: data |> Enum.at(0),
      batchDataPointer: data |> Enum.at(1)
    }
  end

  def is_batch_responded(merkle_root) do
    case AlignedLayerServiceManager.batches_state(Utils.string_to_bytes32(merkle_root))
         |> Ethers.call() do
      {:ok, [_, true]} -> true
      _ -> false
    end
  end

  # for new batches
  def extract_batch_response({_status, %NewBatchInfo{} = batch_creation}) do
    created_batch = batch_creation.new_batch
    was_batch_responded = is_batch_responded(created_batch.batchMerkleRoot)
    batch_response = case was_batch_responded do
      true -> fetch_batch_response(created_batch.batchMerkleRoot)
      false -> %{block_number: nil, transaction_hash: nil, block_timestamp: nil} #was not verified, fill with nils
    end
    %BatchDB{
      merkle_root: created_batch.batchMerkleRoot,
      data_pointer: created_batch.batchDataPointer,
      is_verified: was_batch_responded,
      submission_block_number: batch_creation.block_number,
      submission_transaction_hash: batch_creation.transaction_hash,
      submission_timestamp: batch_creation.block_timestamp,
      response_block_number: batch_response.block_number,
      response_transaction_hash: batch_response.transaction_hash,
      response_timestamp: batch_response.block_timestamp,
      amount_of_proofs: nil,
      proof_hashes: nil
    }
  end

  #for existing but unverified batches
  def extract_batch_response(%Batches{} = unverified_batch) do
    was_batch_responded = is_batch_responded(unverified_batch.merkle_root)
    case was_batch_responded do
      false -> nil # Do nothing since unverified batch was not yet verified
      true ->
        batch_response = fetch_batch_response(unverified_batch.merkle_root)
        %BatchDB{
          merkle_root: unverified_batch.merkle_root,
          data_pointer: unverified_batch.data_pointer,
          is_verified: was_batch_responded,
          submission_block_number: unverified_batch.submission_block_number,
          submission_transaction_hash: unverified_batch.submission_transaction_hash,
          submission_timestamp: unverified_batch.submission_timestamp,
          response_block_number: batch_response.block_number,
          response_transaction_hash: batch_response.transaction_hash,
          response_timestamp: batch_response.block_timestamp,
          amount_of_proofs: unverified_batch.amount_of_proofs,
          proof_hashes: unverified_batch.proof_hashes
        }
    end
  end

  def fetch_batch_response(merkle_root) do
    case get_batch_verified_events(%{merkle_root: merkle_root}) do
      {:ok, batch_verified_info} -> batch_verified_info
      {:empty, _} -> nil
      {:error, error} -> raise("Error fetching batch response: #{error}")
    end
  end

  def get_batch_verified_events(%{merkle_root: merkle_root}) do
    event =
      AlignedLayerServiceManager.EventFilters.batch_verified(Utils.string_to_bytes32(merkle_root))
        |> Ethers.get_logs(fromBlock: @first_block)

    case event do
      {:error, reason} -> {:error, reason}
      {_, []} -> {:empty, "No task found"}
      {:ok, event} -> extract_batch_verified_event_info(event |> List.first())
    end
  end

  defp extract_batch_verified_event_info(event) do
    batch_verified = event |> Map.get(:topics_raw) |> Enum.at(1)

    {:ok,
     %BatchVerifiedInfo{
       address: event |> Map.get(:address),
       block_number: event |> Map.get(:block_number),
       block_timestamp: get_block_timestamp(event |> Map.get(:block_number)),
       transaction_hash: event |> Map.get(:transaction_hash),
       batch_verified: batch_verified
     }}
  end

  def get_block_timestamp(block_number) do
    case Ethers.Utils.get_block_timestamp(block_number) do
      {:ok, timestamp} -> DateTime.from_unix!(timestamp)
      {:error, error} -> raise("Error fetching block timestamp: #{error}")
    end
  end

end
