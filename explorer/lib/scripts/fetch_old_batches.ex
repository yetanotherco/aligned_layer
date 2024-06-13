defmodule Scripts.FetchOldBatches do

  # This Script is to fetch old batches from the blockchain
  # and insert them into the Ecto database

  def run(from, to) do
    "running fetch_old_events" |> IO.inspect()
    fetch_old_events(from, to)
  end

  def fetch_old_events(fromBlock, toBlock) do
    "fetching old events, from #{fromBlock} to #{toBlock}" |> IO.inspect()
    chunk_size = 3 #do in smaller chunks, if there are too many blocks to process
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
    "Making old batches request" |> IO.inspect()
    "from #{fromBlock} to #{toBlock}" |> IO.inspect()
    try do
      Explorer.Periodically.process_from_to_blocks(fromBlock, toBlock)
    rescue
      error -> IO.puts("An error occurred during batch processing*:\n#{inspect(error)}")
    end
  end

end
