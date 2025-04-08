defmodule ExplorerWeb.AggProofsTable do
  use Phoenix.Component
  use ExplorerWeb, :live_component

  attr(:agg_proofs, :list, required: true)

  def agg_proofs_table(assigns) do
    ~H"""
    <.table id="agg_proofs" rows={@proofs}>
      <:col :let={proof} label="Merkle root" class="text-left">
        <.link navigate={~p"/aggregated_proofs/#{proof.number}"}>
          <span class="inline-flex gap-x-3 items-center group-hover:text-foreground/80">
            <%= Helpers.shorten_hash(proof.merkle_root, 6) %>
            <.right_arrow />
            <.tooltip>
              <%= proof.merkle_root %>
            </.tooltip>
          </span>
        </.link>
      </:col>
      <:col :let={proof} label="Status">
        <.dynamic_badge_for_agg_proof status={proof.status} />
      </:col>
      <:col :let={proof} label="Age">
        <span class="md:px-0" title={proof.age}>
          <%= proof.age %>
        </span>
      </:col>
      <:col :let={proof} label="Block Number">
        <%= proof.block_number |> Helpers.format_number() %>
      </:col>

      <:col :let={proof} label="Number of proofs">
        <%= proof.number_of_proofs |> Helpers.format_number() %>
      </:col>
    </.table>
    """
  end
end
