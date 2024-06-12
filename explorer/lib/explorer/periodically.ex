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
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    try do
      "inserting batches..." |> IO.inspect()
      read_from_block = max(0, latest_block_number - 5) # read last 3600 blocks, for redundancy
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: read_from_block, toBlock: latest_block_number})
      |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
      |> Enum.map(&Utils.extract_amount_of_proofs/1)
      |> Enum.map(&Batches.generate_changeset/1)
      |> Enum.map(&Batches.insert_or_update/1)
    rescue
      error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
    end

    # TODO finish this function. It gets previous unverified batches and checks if they were verified
    # This is to avoid having the -3600 because it takes 3-4-5 seconds to fetch each batch from S3
    # "verifying previous unverified batches..." |> IO.inspect()
    # unverified_batches = Batches.get_unverified_batches()
    # # IO.inspect("executed query, unverified_batches:")
    # # IO.inspect(unverified_batches)
    # unverified_batches
    #   |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
    #   |> Enum.reject(&is_nil/1)
    #   |> Enum.map(&Batches.generate_changeset/1)
    #   |> Enum.map(fn changeset -> Explorer.Repo.update(unverified_batches, changeset)end)
    #   #   |> Enum.map(&Explorer.Repo.update)

    schedule_work() # Reschedule once more
    {:noreply, state}
  end

  defp schedule_work() do
    seconds = 5 # edit to modify process frequency
    Process.send_after(self(), {:work}, seconds * 1000)
  end

end
