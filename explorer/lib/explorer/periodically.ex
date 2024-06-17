defmodule Explorer.Periodically do
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(_) do
    :timer.send_interval(6000, :work) # send every 6 seconds, half of 1 block time
    {:ok, 1}
  end

  def handle_info(:work, count) do
    # Reads and process last n blocks for new batches or batch changes
    read_block_qty = 8 # There is a new batch every 4-5 blocks
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    read_from_block = max(0, latest_block_number - read_block_qty)
    Task.start(fn -> process_from_to_blocks(read_from_block, latest_block_number) end)

    # It gets previous unverified batches and checks if they were verified
    run_every_n_iterations = 10
    new_count = rem(count + 1, run_every_n_iterations)
    if new_count == 0 do
      Task.start(&process_unverified_batches/0)
    end

    {:noreply, new_count}
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
