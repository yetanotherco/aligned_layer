defmodule SearchComponent do
  require Logger
  use ExplorerWeb, :live_component

  @impl true
  def handle_event("search_batch", %{"search" => search}, socket) do
    search
    |> (fn hash ->
          if String.match?(hash, ~r/^0x[a-fA-F0-9]+$/), do: {:ok, hash}, else: :invalid_hash
        end).()
    |> case do
      {:ok, hash} ->
        cond do
          # See if the hash belongs to a proof in a batch
          # If so, redirect to search to show all the batches where this proofs exists
          Proofs.get_number_of_batches_containing_proof(hash) > 0 ->
            {:noreply, push_navigate(socket, to: ~p"/search?q=#{hash}")}

          # See if the hash belongs to the root of a batch
          Batches.get_batch(%{merkle_root: hash}) != nil ->
            {:noreply, push_navigate(socket, to: ~p"/batches/#{hash}")}

          # See if the hash belongs to an aggregated proof merkle root
          (proof = AggregatedProofs.get_newest_aggregated_proof_by_merkle_root(hash)) != nil ->
            {:noreply, push_navigate(socket, to: ~p"/aggregated_proofs/#{proof.id}")}

          # Finally, see if the hash belongs to a proof of an aggregated proof
          (proof = AggregationModeProof.get_newest_proof_by_hash(hash)) != nil ->
            {:noreply, push_navigate(socket, to: ~p"/aggregated_proofs/#{proof.agg_proof_id}")}

          # Otherwise inform the user nothing was found
          true ->
            {:noreply,
             socket
             |> put_flash!(:error, "No batch or proof was found with the provided hash.")}
        end

      :invalid_hash ->
        {:noreply,
         socket
         |> put_flash!(:error, "Please enter a valid hash (0x69...).")}
    end
  end

  attr(:class, :string, default: nil)

  @impl true
  def render(assigns) do
    ~H"""
    <form
      phx-target={@myself}
      phx-submit="search_batch"
      class={
        classes([
          "relative flex items-center gap-2 sm:px-0 w-full",
          @class
        ])
      }
    >
      <input
        phx-hook="SearchFocus"
        id={"input_#{assigns.id}"}
        class="pr-10 w-full text-foreground rounded-lg border-foreground/20 bg-card focus:border-foreground/20 focus:ring-accent text-sm"
        type="search"
        placeholder="Search by batch hash or proof hash"
        name="search"
      />
      <.icon name="hero-magnifying-glass-solid" class="absolute right-3 text-foreground/20 size-5 hover:text-foreground" />
    </form>
    """
  end
end
