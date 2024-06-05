defmodule Explorer.Periodically do
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(state) do
    schedule_work(0)
    {:ok, state}
  end

  def handle_info({:work, last_read_block}, state) do
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    try do
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: last_read_block, toBlock: latest_block_number}) |>
      Enum.map(&AlignedLayerServiceManager.find_if_batch_was_responded/1) |>
      Enum.map(fn batch -> Utils.extract_batch_data_pointer_info(batch) end) |>
      Enum.map(&Batches.cast_to_batches/1) |>
      Enum.map(&Map.from_struct/1) |>
      Enum.map(fn batch -> Ecto.Changeset.cast(%Batches{}, batch, [:merkle_root, :amount_of_proofs, :is_verified]) end) |>
      Enum.map(fn changeset -> Explorer.Repo.insert(changeset) end)
    rescue
      error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
    end

    schedule_work(latest_block_number) # Reschedule once more
    {:noreply, state}
  end

  defp schedule_work(last_read_block) do
    Process.send_after(self(), {:work, last_read_block}, 5 * 1000) # 10 seconds
  end

end
