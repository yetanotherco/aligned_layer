defmodule Explorer.Periodically do
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(state) do
    schedule_work()
    {:ok, state}
  end

  def handle_info({:work}, state) do
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    try do
      read_from_block = max(0, latest_block_number - 3600) # read last 3600 blocks, for redundancy
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: read_from_block, toBlock: latest_block_number})
      |> Enum.map(&AlignedLayerServiceManager.find_if_batch_was_responded/1)
      |> Enum.map(&Utils.extract_batch_data_pointer_info/1)
      |> Enum.map(&Batches.cast_to_batches/1)
      |> Enum.map(&Map.from_struct/1)
      |> Enum.map(fn batch -> Ecto.Changeset.cast(%Batches{}, batch, [:merkle_root, :amount_of_proofs, :is_verified]) end)
      |> Enum.map(fn changeset ->
        case Explorer.Repo.get_by(Batches, merkle_root: changeset.changes.merkle_root) do
          nil -> Explorer.Repo.insert(changeset)
          existing_batch ->
            if existing_batch.is_verified != changeset.changes.is_verified do # catches changes of state
              updated_changeset = Ecto.Changeset.change(existing_batch, changeset.changes)
              Explorer.Repo.update(updated_changeset)
            end
        end
      end)
    rescue
      error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
    end

    schedule_work() # Reschedule once more
    {:noreply, state}
  end

  defp schedule_work() do
    Process.send_after(self(), {:work}, 5 * 1000) # n seconds
  end

end
