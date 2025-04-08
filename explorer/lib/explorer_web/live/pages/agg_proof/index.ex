defmodule ExplorerWeb.AggProof.Index do
  require Logger
  use ExplorerWeb, :live_view

  @impl true
  def mount(%{"proof_number" => proof_number}, _, socket) do
    agg_proof =
      AggregatedProofs.get_aggregated_proof_by_number(proof_number)

    {
      :ok,
      assign(
        socket,
        agg_proof: agg_proof,
        proof_hashes: :empty
      )
    }
  end

  @impl true
  def handle_event("show_proofs", _value, socket) do
    proofs = AggregationModeProof.get_all_proof_hashes(socket.assigns.agg_proof.number)
    {:noreply, assign(socket, proof_hashes: proofs)}
  end

  @impl true
  def handle_event("hide_proofs", _value, socket) do
    {:noreply, assign(socket, proof_hashes: :empty)}
  end
end
