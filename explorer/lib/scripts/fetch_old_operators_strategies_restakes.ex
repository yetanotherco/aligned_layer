defmodule Scripts.FetchOldOperatorsStrategiesRestakes do

  # This Script is to fetch old operators from the blockchain activity
  # and insert them into the Ecto database

  def run(fromBlock) do
    # "running fetch_old_operators" |> IO.inspect()
    # AVSDirectoryManager.process_operator_data(%{fromBlock: fromBlock})
    # "done running fetch_old_operators" |> IO.inspect()

    dbg "fetching old operators"
    Explorer.Periodically.process_operators(fromBlock)

    dbg "fetching old quorum strategy changes"
    Explorer.Periodically.process_quorum_strategy_changes()

    dbg "fetching old restaking changes"
    Explorer.Periodically.process_restaking_changes(fromBlock)

    dbg "done"
  end

end
