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
        AlignedLayerServiceManager.get_new_batch_events(page_size * current_page)
        |> Enum.map(&AlignedLayerServiceManager.extract_new_batch_event_info/1)
        |> Enum.map(&AlignedLayerServiceManager.cross_event_with_response/1)
        |> Enum.reverse()

      # TODO read the data pointers from the new data

      schedule_work() # Reschedule once more
      {:noreply, state}
  end

  defp schedule_work() do
      "IN SCHEDULE_WORK" |> IO.inspect()
      Process.send_after(self(), :work, 10 * 1000) # 10 seconds
  end

end
