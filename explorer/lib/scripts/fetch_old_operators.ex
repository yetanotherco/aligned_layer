defmodule Scripts.FetchOldOperators do

  # This Script is to fetch old operators from the blockchain activity
  # and insert them into the Ecto database

  def run(fromBlock) do
    "running fetch_old_operators" |> IO.inspect()
    AVSDirectoryManager.process_operator_data(%{fromBlock: fromBlock})
    "done running fetch_old_operators" |> IO.inspect()
  end

end
