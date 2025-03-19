defmodule ExplorerWeb.BatchesTable do
  use Phoenix.Component
  use ExplorerWeb, :live_component

  attr(:batches, :list, required: true)

  def batches_table(assigns) do
    ~H"""
    <.table id="batches" rows={@batches}>
      <:col :let={batch} label="Batch Hash" class="text-left">
        <.link navigate={~p"/batches/#{batch.merkle_root}"}>
          <span class="inline-flex gap-x-3 items-center group-hover:text-foreground/80">
            <%= Helpers.shorten_hash(batch.merkle_root, 6) %>
            <.right_arrow />
            <.tooltip>
              <%= batch.merkle_root %>
            </.tooltip>
          </span>
        </.link>
      </:col>
      <:col :let={batch} label="Status">
        <.dynamic_badge_for_batcher status={Helpers.get_batch_status(batch)} />
      </:col>
      <:col :let={batch} label="Age">
        <span class="md:px-0" title={batch.submission_timestamp}>
          <%= batch.submission_timestamp |> Helpers.parse_timeago() %>
        </span>
      </:col>
      <:col :let={batch} label="Block Number">
        <%= batch.submission_block_number |> Helpers.format_number() %>
      </:col>

      <:col :let={batch} label="Fee per proof">
        <%= case EthConverter.wei_to_usd_sf(batch.fee_per_proof, 2) do %>
          <% {:ok, usd} -> %>
            <%= "#{usd} USD" %>
          <% {:error, _} -> %>
            <%= "N/A" %>
        <% end %>
        <.tooltip>
          <%= case EthConverter.wei_to_eth(batch.fee_per_proof, 6) do %>
            <% nil -> %>
              <%= "N/A" %>
            <% eth -> %>
              <%= "~= #{eth} ETH" %>
          <% end %>
        </.tooltip>
      </:col>

      <:col :let={batch} label="Number of proofs">
        <%= batch.amount_of_proofs |> Helpers.format_number() %>
      </:col>
    </.table>
    """
  end
end
