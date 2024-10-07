defmodule Scripts.FetchOldOperatorsStrategiesRestakes do
  require Logger

  # This Script is to fetch old operators, strategies and restakes from the blockchain activity
  # and insert them into the Ecto database

  def run(fromBlock) do
    "Fetching old quorum and strategy changes" |> Logger.debug()
    Explorer.Periodically.process_quorum_strategy_changes()

    # Temporal solution to handle new quorums, until Eigenlayer implements emition of QuorumCreated event
    Quorums.handle_quorum(%Quorums{id: 0})

    "Fetching old operators changes" |> Logger.debug()
    Explorer.Periodically.process_operators(fromBlock)

    "Fetching old restaking changes" |> Logger.debug()
    Explorer.Periodically.process_restaking_changes(fromBlock)

    "Done" |> Logger.debug()
  end
end
