defmodule AlignedLayerServiceManager do
  require Logger

  @environment System.get_env("ENVIRONMENT")
  @aligned_config_file System.get_env("ALIGNED_CONFIG_FILE")

  case @environment do
    "devnet" -> Logger.debug("Running on devnet")
    "holesky" -> Logger.debug("Running on holesky")
    "mainnet" -> Logger.debug("Running on mainnet")
    "sepolia" -> Logger.debug("Running on sepolia")
    "hoodi" -> Logger.debug("Running on hoodi")
    _ -> Logger.debug("Invalid ENVIRONMENT var in .env")
    nil -> raise("Invalid ENVIRONMENT var in .env")
  end

  config_file_path =
    case @aligned_config_file do
      nil -> raise("ALIGNED_CONFIG_FILE not set in .env")
      file -> file
    end

  {status_aligned_config, config_json_string} = File.read(config_file_path)

  case status_aligned_config do
    :ok ->
      Logger.debug("Aligned config file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make explorer_create_env? If you did,\n make sure AlignedLayer config file is correctly stored"
      )
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

  def get_latest_block_number() do
    {:ok, num} = Ethers.current_block_number()
    Logger.info("Latest block number: #{num}")
    num
  end

  def get_new_batch_events(%{fromBlock: fromBlock, toBlock: toBlock}) do
    Logger.info("Fetching new batch events from #{fromBlock} to #{toBlock}")
    events =
      AlignedLayerServiceManager.EventFilters.new_batch_v3(nil)
        |> Ethers.get_logs(fromBlock: fromBlock, toBlock: toBlock)

    case events do
      {:ok, []} ->
        Logger.info("No new batch events found in blocks #{fromBlock}-#{toBlock}")
        []

      {:ok, list} ->
        Logger.info("Found #{length(list)} new batch events in blocks #{fromBlock}-#{toBlock}")
        Enum.map(list, &extract_new_batch_event_info/1)

      {:error, reason} ->
        Logger.error("Error fetching new batch events from #{fromBlock} to #{toBlock}: #{Map.get(reason, "message")}")
        raise("Error fetching events: #{Map.get(reason, "message")}")
    end
  end

  def extract_new_batch_event_info(event) do
    block_number = event |> Map.get(:block_number)
    tx_hash = event |> Map.get(:transaction_hash)
    Logger.info("Extracting new batch event info for block #{block_number}, tx: #{tx_hash}")

    new_batch = parse_new_batch_event(event)
    Logger.info("New batch event parsed: #{inspect(new_batch)}")

    {:ok,
     %NewBatchInfo{
       address: event |> Map.get(:address),
       block_number: block_number,
       block_timestamp: get_block_timestamp(block_number),
       transaction_hash: tx_hash,
       new_batch: new_batch
     }}
  end

  def parse_new_batch_event(%Ethers.Event{} = new_batch_event) do
    data = new_batch_event |> Map.get(:data)
    topics_raw = new_batch_event |> Map.get(:topics_raw)

    %NewBatchEvent{
      batchMerkleRoot: topics_raw |> Enum.at(1),
      senderAddress: data |> Enum.at(0),
      taskCreatedBlock: data |> Enum.at(1),
      batchDataPointer: data |> Enum.at(2),
      maxAggregatorFee: data |> Enum.at(3),
    }
  end

  def is_batch_responded(merkle_root, fromBlock) do
    event =
      Utils.string_to_bytes32(merkle_root)
      |> AlignedLayerServiceManager.EventFilters.batch_verified()
      |> Ethers.get_logs(fromBlock: fromBlock)

    case event do
      {:error, reason} ->
        Logger.error("Error checking batch response for #{merkle_root}: #{inspect(reason)}")
        {:error, reason}
      {_, []} ->
        false
      {:ok, _events} ->
        true
    end
  end

  # for new batches
  def extract_batch_response({_status, %NewBatchInfo{} = batch_creation}) do
    created_batch = batch_creation.new_batch
    was_batch_responded = is_batch_responded(created_batch.batchMerkleRoot, batch_creation.block_number)

    batch_response =
      case was_batch_responded do
        true ->
          Logger.info("Batch #{created_batch.batchMerkleRoot} was responded, fetching response details")
          fetch_batch_response(created_batch.batchMerkleRoot, batch_creation.block_number)
        # was not verified, fill with nils
        false ->
          Logger.info("Batch #{created_batch.batchMerkleRoot} was not responded yet")
          %{block_number: nil, transaction_hash: nil, block_timestamp: nil}
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
      proof_hashes: nil,
      fee_per_proof: BatcherPaymentServiceManager.get_fee_per_proof(%{merkle_root: created_batch.batchMerkleRoot, fromBlock: batch_creation.block_number}),
      sender_address: Utils.string_to_bytes32(created_batch.senderAddress),
      max_aggregator_fee: created_batch.maxAggregatorFee,
      is_valid: true # set to false later if a process determines it is invalid
    }
  end

  # for existing but unverified batches
  def extract_batch_response(%Batches{} = unverified_batch) do
    was_batch_responded = is_batch_responded(unverified_batch.merkle_root, unverified_batch.submission_block_number)

    case was_batch_responded do
      # Do nothing since unverified batch was not yet verified
      false ->
        Logger.info("Unverified batch #{unverified_batch.merkle_root} still not responded")
        nil

      true ->
        Logger.info("Unverified batch #{unverified_batch.merkle_root} now responded, updating status")
        batch_response = fetch_batch_response(unverified_batch.merkle_root, unverified_batch.submission_block_number)

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
          fee_per_proof: unverified_batch.fee_per_proof,
          proof_hashes: nil,
          sender_address: unverified_batch.sender_address,
          max_aggregator_fee: unverified_batch.max_aggregator_fee,
          is_valid: true # set to false later if a process determines it is invalid
        }
    end
  end

  def fetch_batch_response(merkle_root, fromBlock) do
    case get_batch_verified_events(%{merkle_root: merkle_root, fromBlock: fromBlock}) do
      {:ok, batch_verified_info} ->
        Logger.info("Successfully fetched batch response for #{merkle_root}")
        batch_verified_info
      {:empty, _} ->
        Logger.info("No batch verified events found for #{merkle_root}")
        nil
      {:error, error} ->
        Logger.error("Error fetching batch response for #{merkle_root}: #{error}")
        raise("Error fetching batch response: #{error}")
    end
  end

  def get_batch_verified_events(%{merkle_root: merkle_root, fromBlock: fromBlock}) do
    event =
      AlignedLayerServiceManager.EventFilters.batch_verified(Utils.string_to_bytes32(merkle_root))
      |> Ethers.get_logs(fromBlock: fromBlock)

    case event do
      {:error, reason} ->
        Logger.error("Error getting batch verified events for #{merkle_root}: #{inspect(reason)}")
        {:error, reason}
      {_, []} ->
        Logger.info("No batch verified events found for #{merkle_root}")
        {:empty, "No task found"}
      {:ok, events} ->
        Logger.info("Found #{length(events)} batch verified events for #{merkle_root}")
        extract_batch_verified_event_info(events |> List.first())
    end
  end

  defp extract_batch_verified_event_info(event) do
    block_number = event |> Map.get(:block_number)

    {:ok,
     %BatchVerifiedInfo{
       address: event |> Map.get(:address),
       block_number: block_number,
       block_timestamp: get_block_timestamp(block_number),
       transaction_hash: event |> Map.get(:transaction_hash),
       batch_merkle_root: event |> Map.get(:topics_raw) |> Enum.at(1),
       sender_address: event |> Map.get(:data) |> Enum.at(0)
     }}
  end

  def get_block_timestamp(block_number) do
    case Ethers.Utils.get_block_timestamp(block_number) do
      {:ok, timestamp} ->
        DateTime.from_unix!(timestamp)
      {:error, error} ->
        Logger.error("Error fetching block timestamp for block #{block_number}: #{error}")
        raise("Error fetching block timestamp: #{error}")
    end
  end

  def get_current_gas_price() do
    case Ethers.current_gas_price() do
      {:ok, gas_price} ->
        gas_price

      {:error, error} ->
        raise("Error fetching gas price: #{error}")
    end
  end

  def update_restakeable_strategies() do
    Logger.info("Updating restakeable strategies")
    case AlignedLayerServiceManager.get_restakeable_strategies() |> Ethers.call() do
      {:ok, restakeable_strategies} ->
        Logger.info("Successfully fetched #{length(restakeable_strategies)} restakeable strategies")
        Strategies.update(restakeable_strategies)

      {:error, error} ->
        Logger.error("Error fetching restakeable strategies: #{error}")
        raise("Error fetching restakeable strategies: #{error}")
    end
  end
end
