defmodule Explorer.Periodically do
  require Logger
  alias Phoenix.PubSub
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(_) do
    send_work()
    {:ok, %{batches_count: 0, restakings_last_read_block: 0}}
  end

  def send_work() do
    one_second = 1000
    seconds_in_an_hour = 60 * 60

    # every minute
    :timer.send_interval(one_second * 60, :next_batch_progress)
    # every 12 seconds, once per block
    :timer.send_interval(one_second * 12, :batches)
    # every 1 hour
    :timer.send_interval(one_second * seconds_in_an_hour, :restakings)

    # Fetch new aggregated proofs every 1 minute
    :timer.send_interval(one_second * 60, :aggregated_proofs)
  end

  # Reads and process last blocks for operators and restaking changes
  def handle_info(:restakings, state) do
    last_read_block = Map.get(state, :restakings_last_read_block)
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()

    process_quorum_strategy_changes()
    process_operators(last_read_block)
    process_restaking_changes(last_read_block)

    PubSub.broadcast(Explorer.PubSub, "update_restakings", %{})

    {:noreply, %{state | restakings_last_read_block: latest_block_number}}
  end

  def handle_info(:next_batch_progress, state) do
    Logger.debug("handling block progress timer")
    remaining_time = ExplorerWeb.Helpers.get_next_scheduled_batch_remaining_time()

    PubSub.broadcast(Explorer.PubSub, "update_views", %{
      next_scheduled_batch_remaining_time_percentage:
        ExplorerWeb.Helpers.get_next_scheduled_batch_remaining_time_percentage(remaining_time),
      next_scheduled_batch_remaining_time: remaining_time
    })

    {:noreply, state}
  end

  # Reads and process last n blocks for new batches or batch changes
  def handle_info(:batches, state) do
    count = Map.get(state, :batches_count)
    read_block_qty = 8
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    read_from_block = max(0, latest_block_number - read_block_qty)

    Task.start(fn -> process_batches(read_from_block, latest_block_number) end)

    run_every_n_iterations = 8
    new_count = rem(count + 1, run_every_n_iterations)

    if new_count == 0 do
      Task.start(&process_unverified_batches/0)
    end

    PubSub.broadcast(Explorer.PubSub, "update_views", :block_age)

    {:noreply, %{state | batches_count: new_count}}
  end

  def handle_info(:aggregated_proofs, state) do
    # This task runs every hour
    # We read a bit more than 300 blocks (1hr) to make sure we don't lose any event
    read_block_qty = 310
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    read_from_block = max(0, latest_block_number - read_block_qty)

    Task.start(fn -> process_aggregated_proofs(read_from_block, latest_block_number) end)

    {:noreply, state}
  end

  def process_aggregated_proofs(from_block, to_block) do
    Logger.info("[Aggregated Proofs] Starting fetch from block #{from_block} to #{to_block}")

    case AlignedProofAggregationService.get_aggregated_proof_event(%{
           from_block: from_block,
           to_block: to_block
         }) do
      {:ok, []} ->
        Logger.info("[Aggregated Proofs] No events found in block range #{from_block}-#{to_block}")

      {:ok, proofs} ->
        Logger.info(
          "[Aggregated Proofs] Found #{length(proofs)} events in block range #{from_block}-#{to_block}"
        )

        process_aggregated_proof_events(proofs)

      {:error, reason} ->
        Logger.error(
          "[Aggregated Proofs] Failed to fetch events from block #{from_block} to #{to_block}: #{inspect(reason)}"
        )
    end
  end

  defp process_aggregated_proof_events(proofs) do
    proofs
    |> Enum.each(fn proof ->
      Logger.info(
        "[Aggregated Proofs] Processing proof at block #{proof.block_number}, merkle_root: #{proof.merkle_root}"
      )

      try do
        # Fetch blob data
        Logger.debug(
          "[Aggregated Proofs] Fetching blob data for versioned_hash: #{proof.blob_versioned_hash}"
        )

        blob_data = AlignedProofAggregationService.get_blob_data!(proof)

        # Decode blob to get proof hashes
        proof_hashes =
          AlignedProofAggregationService.decode_blob(
            to_charlist(String.replace_prefix(blob_data, "0x", ""))
          )

        Logger.info(
          "[Aggregated Proofs] Decoded #{length(proof_hashes)} proof hashes from blob"
        )

        # Get aggregator type
        aggregator = AlignedProofAggregationService.get_aggregator!(proof)

        Logger.debug(
          "[Aggregated Proofs] Aggregator type: #{inspect(aggregator)} for merkle_root: #{proof.merkle_root}"
        )

        # Store aggregated proof to db
        agg_proof =
          proof
          |> Map.merge(%{aggregator: aggregator})
          |> Map.merge(%{number_of_proofs: length(proof_hashes)})

        case AggregatedProofs.insert_or_update(agg_proof) do
          {:ok, %{id: id}} ->
            Logger.info(
              "[Aggregated Proofs] Stored aggregated proof id=#{id}, merkle_root: #{proof.merkle_root}, proofs_count: #{length(proof_hashes)}"
            )

            # Store each individual proof hash
            store_individual_proofs(id, proof_hashes, proof.merkle_root)

          {:error, reason} ->
            Logger.error(
              "[Aggregated Proofs] Failed to store aggregated proof merkle_root: #{proof.merkle_root}: #{inspect(reason)}"
            )
        end
      rescue
        error ->
          Logger.error(
            "[Aggregated Proofs] Error processing proof at block #{proof.block_number}, merkle_root: #{proof.merkle_root}: #{Exception.message(error)}"
          )

          Logger.debug(
            "[Aggregated Proofs] Stacktrace: #{Exception.format_stacktrace(__STACKTRACE__)}"
          )
      end
    end)

    Logger.info("[Aggregated Proofs] Finished processing #{length(proofs)} events")
  end

  defp store_individual_proofs(agg_proof_id, proof_hashes, merkle_root) do
    proof_hashes
    |> Enum.with_index()
    |> Enum.each(fn {hash, index} ->
      proof_hash = "0x" <> List.to_string(hash)

      case AggregationModeProof.insert_or_update(%{
             agg_proof_id: agg_proof_id,
             proof_hash: proof_hash,
             index: index
           }) do
        {:ok, _} ->
          :ok

        {:error, reason} ->
          Logger.error(
            "[Aggregated Proofs] Failed to store individual proof hash #{proof_hash} for merkle_root: #{merkle_root}: #{inspect(reason)}"
          )
      end
    end)
  end

  def process_batches(fromBlock, toBlock) do
    "Processing from block #{fromBlock} to block #{toBlock}..." |> Logger.debug()

    try do
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: fromBlock, toBlock: toBlock})
      |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
      # This function will avoid processing a batch taken by another process
      |> Enum.map(&process_batch_if_not_in_other_process/1)
    rescue
      error -> Logger.error("An error occurred during batch processing:\n#{inspect(error)}")
    end

    Logger.debug("Done processing from block #{fromBlock} to block #{toBlock}")
  end

  def process_batch_if_not_in_other_process(%BatchDB{} = batch) do
    "Starting batch: #{batch.merkle_root}" |> Logger.debug()
    # Don't process same twice concurrently
    # one lock for each batch
    case Mutex.lock(BatchMutex, {batch.merkle_root}) do
      {:error, :busy} ->
        "Batch already being processed: #{batch.merkle_root}" |> Logger.debug()
        nil

      {:ok, lock} ->
        "Processing batch: #{batch.merkle_root}" |> Logger.debug()

        with {:ok, updated_batch} <- Utils.process_batch(batch),
             {batch_changeset, proofs} <- Batches.generate_changesets(updated_batch),
             {:ok, _} <- Batches.insert_or_update(batch_changeset, proofs) do
          PubSub.broadcast(Explorer.PubSub, "update_views", %{
            eth_usd:
              case EthConverter.get_eth_price_usd() do
                {:ok, eth_usd_price} -> eth_usd_price
                {:error, _error} -> :empty
              end
          })
        else
          {:error, reason} ->
            Logger.error("Error processing batch #{batch.merkle_root}. Error: #{inspect(reason)}")

          # no changes in DB
          nil ->
            nil
        end

        "Done processing batch: #{batch.merkle_root}" |> Logger.debug()
        Mutex.release(BatchMutex, lock)
    end
  end

  defp process_unverified_batches() do
    "Verifying previous unverified batches..." |> Logger.debug()
    unverified_batches = Batches.get_unverified_batches()

    array_of_changest_tuples =
      unverified_batches
      |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
      |> Enum.reject(&is_nil/1)
      |> Enum.map(&Batches.generate_changesets/1)

    Enum.map(
      array_of_changest_tuples,
      fn {batch_changeset, proofs} ->
        Batches.insert_or_update(batch_changeset, proofs)
      end
    )
  end

  def process_quorum_strategy_changes() do
    "Processing strategy changes..." |> Logger.debug()
    AlignedLayerServiceManager.update_restakeable_strategies()
    Quorums.process_quorum_changes()
  end

  def process_operators(fromBlock) do
    "Processing operators..." |> Logger.debug()
    AVSDirectoryManager.process_and_store_operator_data(%{fromBlock: fromBlock})
  end

  def process_restaking_changes(read_from_block) do
    "Processing restaking changes..." |> Logger.debug()
    Restakings.process_restaking_changes(%{fromBlock: read_from_block})
  end
end
