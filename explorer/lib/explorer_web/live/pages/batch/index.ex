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

    current_batch =
      case Batches.get_batch(%{merkle_root: merkle_root}) do
        nil -> :empty
        batch -> batch
      end


    {
      :ok,
      assign(socket,
        merkle_root: merkle_root,
        current_batch: current_batch,
        page_title: Utils.shorten_hash(merkle_root)
      )
    }
  rescue
    _ ->
      {:ok, assign(socket, merkle_root: :empty, newBatchInfo: :empty, batchWasResponded: :empty)}
  end

  embed_templates "*"
end
