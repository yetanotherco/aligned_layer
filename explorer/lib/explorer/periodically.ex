defmodule Explorer.Periodically do
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(state) do
    schedule_work()
    {:ok, state}
  end

  #wip new db version:
  def handle_info({:work}, state) do
    # This reads and process last n blocks for new batches or batch changes
    read_block_qty = 8 # There is a new batch every 4-5 blocks
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    read_from_block = max(0, latest_block_number - read_block_qty)
    process_from_to_blocks(read_from_block, latest_block_number)

    # It gets previous unverified batches and checks if they were verified
    # This is to avoid having the -3600 because it takes 3-4-5 seconds to fetch each batch from S3
    # And would take even longer for heavier proof batches, for example SP1
    process_unverified_batches()

    schedule_work() # Reschedule once more
    {:noreply, state}
  end

  defp schedule_work() do
    seconds = 1 # edit to modify process frequency
    Process.send_after(self(), {:work}, seconds * 1000)
  end

  def process_from_to_blocks(fromBlock, toBlock) do
    "Processing from block #{fromBlock} to block #{toBlock}..." |> IO.inspect()
    try do
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: fromBlock, toBlock: toBlock})
      |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
      |> Enum.map(&Utils.extract_amount_of_proofs/1)
      |> Enum.map(&Batches.generate_changeset/1)
      |> Enum.map(&Batches.insert_or_update/1)
    rescue
      error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
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
