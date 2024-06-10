defmodule Explorer.Periodically do
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{})
  end

  def init(state) do
    schedule_work()
    {:ok, state}
  end

  #previous version, wip is refactor to new db with more params
  # def handle_info({:work, last_read_block}, state) do
  #   latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
  #   try do
  #     read_from_block = last_read_block - 8 #for redundancy
  #     AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: read_from_block, toBlock: latest_block_number})
  #     |> Enum.map(&AlignedLayerServiceManager.find_if_batch_was_responded/1)
  #     |> Enum.map(&Utils.extract_batch_data_pointer_info/1)
  #     |> Enum.map(&Batches.cast_to_batches/1)
  #     |> Enum.map(&Map.from_struct/1)
  #     |> Enum.map(fn batch -> Ecto.Changeset.cast(%Batches{}, batch, [:merkle_root, :amount_of_proofs, :is_verified]) end)
  #     |> Enum.map(fn changeset ->
  #       case Explorer.Repo.get_by(Batches, merkle_root: changeset.changes.merkle_root) do
  #         nil -> Explorer.Repo.insert(changeset)
  #         existing_batch ->
  #           if existing_batch.is_verified == false and changeset.changes.is_verified == true do
  #             updated_changeset = Ecto.Changeset.change(existing_batch, changeset.changes)
  #             Explorer.Repo.update(updated_changeset)
  #           end
  #       end
  #     end)
  #   rescue
  #     error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
  #   end

  #   schedule_work(latest_block_number) # Reschedule once more
  #   {:noreply, state}
  # end

  #wip new db version:
  def handle_info({:work}, state) do
    latest_block_number = AlignedLayerServiceManager.get_latest_block_number()
    try do
      "inserting batches..." |> IO.inspect()
      read_from_block = max(0, latest_block_number - 3600) # read last 3600 blocks, for redundancy
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: read_from_block, toBlock: latest_block_number})
      |> Enum.map(&AlignedLayerServiceManager.extract_batch_response/1)
      |> Enum.map(&Utils.extract_amount_of_proofs/1)
      |> Enum.map(&Batches.generate_changeset/1)
      # |> IO.inspect()
      |> Enum.map(&Ecto.Multi.insert_or_update/1)
      # |> Enum.map(fn changeset ->
      #   case Explorer.Repo.get_by(Batches, merkle_root: changeset.changes.merkle_root) do
      #     nil -> Explorer.Repo.insert(changeset)
      #     existing_batch ->
      #       if existing_batch.is_verified != changeset.changes.is_verified do #TODO add reorg handling (rewrite if tx hash or block num changed)
      #         updated_changeset = Ecto.Changeset.change(existing_batch, changeset.changes)
      #         Explorer.Repo.update(updated_changeset)
      #       end
      #   end
      # end)
      # |> IO.inspect()
      IO.inspect("Batches inserted")
    rescue
      error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
    end

    schedule_work() # Reschedule once more
    {:noreply, state}
  end

  defp schedule_work() do
    seconds = 5 # edit to modify process frequency
    Process.send_after(self(), {:work}, seconds * 1000)
  end

end
