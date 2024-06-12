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
      process_from_to_blocks(fromBlock, toBlock)
    rescue
      error -> IO.puts("An error occurred during batch processing*:\n#{inspect(error)}")
    end
  end

  # This is a copy of the function in periodically.ex
  # But it doesn't try to read from S3 and:
    # inserts amount of proofs = average
    # skips getting block timestamp
  def process_from_to_blocks(fromBlock, toBlock) do
    "Processing from block #{fromBlock} to block #{toBlock}..." |> IO.inspect()
    # TODO transform to Stream
    try do
      get_new_batch_events(%{fromBlock: fromBlock, toBlock: toBlock})
      |> Enum.map(&extract_batch_response/1)
      |> Enum.map(&Batches.generate_changeset/1)
      |> Enum.map(&Batches.insert_or_update/1)
    rescue
      error -> IO.puts("An error occurred during batch processing**:\n#{inspect(error)}")
    end
  end

  def extract_batch_response({_status, %NewBatchInfo{} = batch_creation}) do
    created_batch = batch_creation.new_batch
    %BatchDB{
      merkle_root: created_batch.batchMerkleRoot,
      data_pointer: created_batch.batchDataPointer,
      is_verified: true,
      submition_block_number: batch_creation.block_number,
      submition_transaction_hash: batch_creation.transaction_hash,
      submition_timestamp: batch_creation.block_timestamp,
      response_block_number: nil,
      response_transaction_hash: nil,
      response_timestamp: nil,
      amount_of_proofs: 502,
    }
  end

  def get_new_batch_events(%{fromBlock: fromBlock, toBlock: toBlock}) do
    "From block" |> IO.inspect()
    fromBlock |> IO.inspect()
    "To block" |> IO.inspect()
    toBlock |> IO.inspect()

    events =
      AlignedLayerServiceManager.EventFilters.new_batch(nil)
      |> Ethers.get_logs(fromBlock: fromBlock, toBlock: toBlock)

    case events do
      {:ok, []} -> []
      {:ok, list} -> Enum.map(list, &extract_new_batch_event_info/1)
      {:error, reason } -> raise("Error fetching events: #{Map.get(reason, "message")}")
    end
  end
  def extract_new_batch_event_info(event) do
    new_batch = AlignedLayerServiceManager.parse_new_batch_event(event)
    {:ok,
     %NewBatchInfo{
       address: event |> Map.get(:address),
       block_number: event |> Map.get(:block_number),
       block_timestamp: nil,
       transaction_hash: event |> Map.get(:transaction_hash),
       new_batch: new_batch
     }}
  end

end
