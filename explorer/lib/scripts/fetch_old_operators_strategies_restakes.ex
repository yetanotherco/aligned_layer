defmodule Scripts.FetchOldOperatorsStrategiesRestakes do

  # This Script is to fetch old operators from the blockchain activity
  # and insert them into the Ecto database

  def run(fromBlock) do
    dbg "fetching old quorum and strategy changes"
    Explorer.Periodically.process_quorum_strategy_changes()

    # Temporal solution to handle new quorums, until Eigenlayer implements emition of QuorumCreated event
    Quorums.handle_quorum(%Quorums{id: 0})

    dbg "fetching old operators changes"
    Explorer.Periodically.process_operators(fromBlock)

    dbg "fetching old restaking changes"
    Explorer.Periodically.process_restaking_changes(fromBlock)

    dbg "done"
  end

end
