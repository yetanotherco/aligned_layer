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
    :timer.send_interval(one_second * seconds_in_an_hour, :aggregated_proofs)
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
    # This runs every 1hr, so reading 300 means going back exactly one hour
    # We add a few blocks more to make sure we don't lose anything
    read_block_qty = 310
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    read_from_block = max(0, latest_block_number - read_block_qty)

    process_aggregated_proofs(read_from_block, latest_block_number)
  end

  def process_aggregated_proofs(from_block, to_block) do
    "Processing aggregated proofs" |> Logger.debug()

    aggregated_proofs =
      AlignedProofAggregationService.get_aggregated_proof_event(%{
        from_block: from_block,
        to_block: to_block
      })
      |> Enum.map(fn x ->
        Map.merge(
          x,
          %{
            blob_data: AlignedProofAggregationService.get_blob_data_from_versioned_hash(x)
          }
        )
      end)

    # Split the blob data in chunks of 32 to get the number of leaves (number of proofs) in the aggregated proof
    ## TODO fix this parsing
    proofs_leaves =
      Enum.map(aggregated_proofs, fn x -> chunk_every(x.blob_data, 32) end)

    # Store aggregated proofs to db
    aggregated_proofs
    |> Enum.zip(proofs_leaves)
    |> Enum.map(fn %{agg_proof, leaves} ->
      Map.merge(agg_proof, %{number_of_proofs: length(leaves)})
      |> Enum.each(fn x -> AggregatedProof.insert_or_update(x) end)
    end)

    # Store each individual proof
    aggregated_proofs
    |> Enum.zip(proofs_leaves)
    |> Enum.map(fn %{agg_proof, leaves} ->
      Enum.each(leaves, fn leaf ->
        AggregatedProof.insert_proof(%{
          aggregated_proof_number: agg_proof.number,
          proof_hash: leaf
        })
      end)
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
