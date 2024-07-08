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
    seconds = 12 # once per block
    :timer.send_interval(seconds * 1000, :work) # send every n seconds
  end

  def handle_info(:work, count) do
    # Reads and process last n blocks for new batches or batch changes
      read_block_qty = 8
      latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
      read_from_block = max(0, latest_block_number - read_block_qty)

    Task.start(fn -> process_blocks_from_to(read_from_block, latest_block_number) end)

    # Gets previous unverified batches and checks if they were verified
      run_every_n_iterations = 8
      new_count = rem(count + 1, run_every_n_iterations)
      if new_count == 0 do
        Task.start(&process_unverified_batches/0)
      end

    {:noreply, new_count}
  end

  def process_blocks_from_to(fromBlock, toBlock) do
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
        "Batch already being processed: #{batch.merkle_root}" |> IO.inspect
        nil

      {:ok, lock} ->
        "Processing batch: #{batch.merkle_root}" |> IO.inspect
        batch
          |> Utils.extract_info_from_data_pointer
          |> Batches.generate_changeset
          |> Batches.insert_or_update
          |> case do
            {:ok, _} ->
              IO.puts("Broadcasting update_views")
              PubSub.broadcast(Explorer.PubSub, "update_views", %{})
            {:error, error} ->
              IO.puts("Some error in DB operation, not broadcasting update_views")
              IO.inspect(error)
            nil -> nil #no changes in DB

          end

        "Done processing batch: #{batch.merkle_root}" |> IO.inspect
        Mutex.release(BatchMutex, lock)
    end
  end

  defp process_unverified_batches() do
    "verifying previous unverified batches..." |> IO.inspect()
    unverified_batches = Batches.get_unverified_batches()
    unverified_batches
      |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
      |> Enum.reject(&is_nil/1)
      |> Enum.map(&Batches.generate_changeset/1)
      |> Enum.map(&Batches.insert_or_update/1)
  end

end
