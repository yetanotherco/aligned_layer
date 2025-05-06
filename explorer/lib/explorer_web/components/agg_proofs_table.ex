defmodule ExplorerWeb.AggProofsTable do
  use Phoenix.Component
  use ExplorerWeb, :live_component

  attr(:agg_proofs, :list, required: true)

  def agg_proofs_table(assigns) do
    ~H"""
    <.table id="agg_proofs" rows={@proofs}>
      <:col :let={proof} label="Merkle root" class="text-left">
        <.link navigate={~p"/aggregated_proofs/#{proof.id}"}>
          <span class="inline-flex gap-x-3 items-center group-hover:text-foreground/80">
            <%= Helpers.shorten_hash(proof.merkle_root, 6) %>
            <.right_arrow />
            <.tooltip>
              <%= proof.merkle_root %>
            </.tooltip>
          </span>
        </.link>
      </:col>
      <:col :let={proof} label="Age">
        <span class="md:px-0" title={proof.age}>
          <%= proof.age %>
        </span>
      </:col>
      <:col :let={proof} label="Block Number">
        <%= proof.block_number |> Helpers.format_number() %>
      </:col>

      <:col :let={proof} label="Blob versioned hash" class="text-left">
        <.a href={
          "#{Helpers.get_blobscan_url()}/blob/#{proof.blob_versioned_hash}"}
          class="inline-flex gap-x-3 items-center group-hover:text-foreground/80 no-underline font-normal"
          >
          <span class="inline-flex gap-x-3 items-center group-hover:text-foreground/80">
            <%= Helpers.shorten_hash(proof.blob_versioned_hash, 6) %>
            <.tooltip>
              <%= proof.blob_versioned_hash %>
            </.tooltip>
          </span>
        </.a>
      </:col>

      <:col :let={proof} label="Number of proofs">
        <%= proof.number_of_proofs |> Helpers.format_number() %>
      </:col>

      <:col :let={proof} label="Aggregator">
        <%= case proof.aggregator do %>
          <% :sp1 -> %>
            <.sp1_badge />
          <% :risc0 -> %>
            <.risc0_badge />
          <% _ -> %>
            <span>Unknown</span>
        <% end %>
      </:col>
    </.table>
    """
  end

  defp sp1_badge(assigns) do
    ~H"""
    <div class="rounded-full p-1 px-5 border w-fit" style="border-color: #FE11C5">
        <p style="color: #FE11C5">SP1</p>
    </div>
    """
  end

  defp risc0_badge(assigns) do
    ~H"""
    <div class="rounded-full p-1 px-5 w-fit" style="background-color: #FEFF9D">
        <p class="text-black">RISC0</p>
    </div>
    """
  end
end
