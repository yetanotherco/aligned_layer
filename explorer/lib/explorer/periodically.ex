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

    :timer.send_interval(one_second * 12, :batches) # every 12 seconds, once per block
    :timer.send_interval(one_second * seconds_in_an_hour, :restakings) # every 1 hour
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

    {:noreply, %{state | batches_count: new_count}}
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

        {batch_changeset, proofs} =
          batch
          |> NetworkStorage.extract_info_from_data_pointer()
          |> Batches.generate_changesets()

        Batches.insert_or_update(batch_changeset, proofs)
        |> case do
          {:ok, _} ->
            PubSub.broadcast(Explorer.PubSub, "update_views", %{
              eth_usd:
                case EthConverter.get_eth_price_usd() do
                  {:ok, eth_usd_price} -> eth_usd_price
                  {:error, _error} -> :empty
                end
            })

          {:error, error} ->
            Logger.error("Some error in DB operation, not broadcasting update_views: #{inspect(error)}")

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
