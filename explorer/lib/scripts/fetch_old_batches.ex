defmodule Scripts.FetchOldBatches do

  # This Script is to fetch old batches from the blockchain
  # and insert them into the Ecto database

  def run(from, to) do
    "running fetch_old_events" |> IO.inspect()
    fetch_old_events(from, to)
  end

  def fetch_old_events(fromBlock, toBlock) do
    "fetching old events, from #{fromBlock} to #{toBlock}" |> IO.inspect()
    chunk_size = 5000 #do in smaller chunks as to not request 100k events to rpc in one call
    chunkify(fromBlock, toBlock, chunk_size) |> Enum.each(&make_request/1)
    "done fetching old events" |> IO.inspect()
  end

  defp chunkify(start_num, end_num, chunk_size) do
    Stream.iterate(start_num, &(&1 + chunk_size))
    |> Stream.take_while(&(&1 <= end_num))
    |> Enum.map(fn chunk_start ->
      {chunk_start, min(chunk_start + chunk_size - 1, end_num)}
    end)
  end

  defp make_request({fromBlock, toBlock}) do
    "making request from #{fromBlock} to #{toBlock}" |> IO.inspect()
    try do
      read_from_block = fromBlock
      AlignedLayerServiceManager.get_new_batch_events(%{fromBlock: read_from_block, toBlock: toBlock})
      |> Enum.map(&transform_to_batch/1)
      |> Enum.map(&transfrom_to_batch_db/1)
      |> Enum.map(&Batches.cast_to_batches/1)
      |> Enum.map(&Map.from_struct/1)
      |> Enum.map(fn batch -> Ecto.Changeset.cast(%Batches{}, batch, [:merkle_root, :amount_of_proofs, :is_verified]) end)
      |> Enum.map(fn changeset ->
        Explorer.Repo.insert_or_update(changeset)
      end)
    rescue
      error -> IO.puts("An error occurred during batch processing:\n#{inspect(error)}")
    end
  end

  def transform_to_batch(%Ethers.Event{} = new_batch_event) do
    new_batch = AlignedLayerServiceManager.parse_new_batch_event(new_batch_event)
    %Batch{
      batch_merkle_root: new_batch.batchMerkleRoot,
      batch_data_pointer: new_batch.batchDataPointer,
      is_verified: true
    }
  end

  def transfrom_to_batch_db(%Batch{} = batch) do
    %BatchDB {
      batch_merkle_root: batch.batch_merkle_root,
      amount_of_proofs: 470,
      is_verified: batch.is_verified
    }
  end

end
