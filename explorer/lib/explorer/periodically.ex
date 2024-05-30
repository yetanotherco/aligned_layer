defmodule Explorer.Periodically do
  use GenServer

  def start_link(_) do
      GenServer.start_link(__MODULE__, %{})
  end

  def init(state) do
      schedule_work()
      {:ok, state}
  end

  def handle_info(:work, state) do
      batches =
        AlignedLayerServiceManager.get_new_batch_events() |>
        Enum.map(&AlignedLayerServiceManager.cross_event_with_response/1) |>
        Enum.map(fn batch -> Utils.extract_batch_data_pointer_info(batch) end) |>
        Enum.map(&Batches.cast_to_batches/1) |>
        Enum.map(&Map.from_struct/1) |>
        Enum.map(fn batch -> Ecto.Changeset.cast(%Batches{}, batch, [:merkle_root, :amount_of_proofs, :is_verified]) end) |>
        Enum.map(fn changeset -> Explorer.Repo.insert(changeset) end)

      "batches" |> IO.inspect()
      batches |> IO.inspect()

      schedule_work() # Reschedule once more
      {:noreply, state}
  end

  defp schedule_work() do
      "IN SCHEDULE_WORK" |> IO.inspect()
      Process.send_after(self(), :work, 5 * 1000) # 10 seconds
  end

end
