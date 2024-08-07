defmodule Explorer.Periodically do
  alias Phoenix.PubSub
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(_) do
    send_work()
    {:ok, 0}
  end

  def send_work() do
    # once per block
    seconds = 12
    # send every n seconds
    :timer.send_interval(seconds * 1000, :work)
  end

  def handle_info(:work, count) do
    # Reads and process last n blocks for new batches or batch changes
    read_block_qty = 8
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    read_from_block = max(0, latest_block_number - read_block_qty)

    Task.start(fn -> process_batches(read_from_block, latest_block_number) end)

    run_every_n_iterations = 8
    new_count = rem(count + 1, run_every_n_iterations)
    if new_count == 0 do
      Task.start(&process_unverified_batches/0)
      Task.start(fn -> process_operators(read_from_block) end)
      Task.start(fn -> process_quorum_strategy_changes(read_from_block) end)
      Task.start(fn -> process_restaking_changes(read_from_block) end)
    end
    # process_operators(0)
    # process_quorum_strategy_changes()
    # process_restaking_changes(0)


    {:noreply, new_count}
  end

  def process_batches(fromBlock, toBlock) do
    "Processing from block #{fromBlock} to block #{toBlock}..." |> IO.inspect()

    try do
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: fromBlock, toBlock: toBlock})
      |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
      # This function will avoid processing a batch taken by another process
      |> Enum.map(&process_batch_if_not_in_other_process/1)
    rescue
      error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
    end

    IO.inspect("Done processing from block #{fromBlock} to block #{toBlock}")
  end

  def process_batch_if_not_in_other_process(%BatchDB{} = batch) do
    "Starting batch: #{batch.merkle_root}" |> IO.inspect()
    # Don't process same twice concurrently
    # one lock for each batch
    case Mutex.lock(BatchMutex, {batch.merkle_root}) do
      {:error, :busy} ->
        "Batch already being processed: #{batch.merkle_root}" |> IO.inspect()
        nil

      {:ok, lock} ->
        "Processing batch: #{batch.merkle_root}" |> IO.inspect()

        {batch_changeset, proofs} =
          batch
          |> Utils.extract_info_from_data_pointer()
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
            IO.puts("Some error in DB operation, not broadcasting update_views")
            IO.inspect(error)

          # no changes in DB
          nil ->
            nil
        end

        "Done processing batch: #{batch.merkle_root}" |> IO.inspect()
        Mutex.release(BatchMutex, lock)
    end
  end

  defp process_unverified_batches() do
    "Verifying previous unverified batches..." |> IO.inspect()
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

  def process_operators(fromBlock) do
    "Processing operators..." |> IO.inspect()
    AVSDirectoryManager.process_operator_data(%{fromBlock: fromBlock})
  end

  def process_quorum_strategy_changes() do
    "Processing strategy changes..." |> IO.inspect()
    AlignedLayerServiceManager.update_restakeable_strategies()
    Quorums.process_quorum_changes()
  end

  def process_restaking_changes(read_from_block) do
    "Processing restaking changes..." |> IO.inspect()
    Restakings.process_restaking_changes(%{fromBlock: read_from_block})
  end
end
