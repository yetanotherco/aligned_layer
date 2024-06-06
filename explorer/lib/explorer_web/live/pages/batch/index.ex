defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    merkle_root = params["merkle_root"]

    if merkle_root == nil do
      {
        :empty,
        assign(socket, newBatchEvent: :empty, batchWasResponded: :empty)
      }
    end

    newBatchInfo =
      case AlignedLayerServiceManager.get_new_batch_events(%{merkle_root: merkle_root}) do
        {:error, reason} ->
          Logger.error("batch detail error: ", reason)
          {:error, reason}

        {:empty, reason} ->
          Logger.info("batch returned empty: ", reason)
          :empty

        {_, []} ->
          :empty

        {:ok, event} ->
          event
      end

    batchWasResponded = AlignedLayerServiceManager.is_batch_responded(merkle_root)


    amount_of_proofs = AlignedLayerServiceManager.get_amount_of_proofs(newBatchInfo)

    {
      :ok,
      assign(socket,
        merkle_root: merkle_root,
        newBatchInfo: newBatchInfo,
        batchWasResponded: batchWasResponded,
        page_title: Utils.shorten_block_hash(merkle_root),
        amount_of_proofs: amount_of_proofs
      )
    }
  rescue
    _ ->
      # TODO handle different the 'without 0x prefix' error, for usability
      # ex.message == "Invalid hex string" or ex.message == "Invalid hex string, missing '0x' prefix" do
      {:ok, assign(socket, merkle_root: :empty, newBatchInfo: :empty, batchWasResponded: :empty)}
  end

  embed_templates "*"
end
