defmodule Scripts.FetchOldBatches do
  require Logger

  # This Script is to fetch old batches from the blockchain
  # and insert them into the Ecto database

  def run(from, to) do
    "Running fetch_old_events" |> Logger.debug()
    fetch_old_events(from, to)
  end

  def fetch_old_events(fromBlock, toBlock) do
    "Fetching old events, from #{fromBlock} to #{toBlock}" |> Logger.debug()
    chunk_size = 500 # do in smaller chunks, if there are too many blocks to process
    chunkify(fromBlock, toBlock, chunk_size) |> Enum.each(&make_request/1)
    "✅ Done fetching old events" |> Logger.debug()
  end

  defp chunkify(start_num, end_num, chunk_size) do
    Stream.iterate(start_num, &(&1 + chunk_size))
    |> Stream.take_while(&(&1 <= end_num))
    |> Enum.map(fn chunk_start ->
      {chunk_start, min(chunk_start + chunk_size - 1, end_num)}
    end)
  end

  defp make_request({fromBlock, toBlock}) do
    "Making old batches request from #{fromBlock} to #{toBlock}" |> Logger.debug()
    try do
      Explorer.Periodically.process_batches(fromBlock, toBlock)
    rescue
      error -> "An error occurred during batch processing*:\n#{inspect(error)}" |> Logger.error()
    end
  end

end
