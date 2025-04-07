defmodule ExplorerWeb.AggProofs.Index do
  require Logger
  use ExplorerWeb, :live_view

  @impl true
  def mount(%{"proof_number" => proof_number}, _, socket) do
    agg_proof =
      Explorer.AggregatedProofs.get_aggregated_proof_by_number(proof_number)

    proofs = Explorer.AggregationModeProof.get_all_proof_hashes(proof_number)

    {
      :ok,
      assign(
        socket,
        agg_proof: agg_proof,
        proofs: proofs
      )
    }
  end
end
