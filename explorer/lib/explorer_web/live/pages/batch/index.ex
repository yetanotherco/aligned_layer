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
      case AlignedLayerServiceManager.get_new_batch_events(merkle_root) do
        {:error, reason} -> {:error, reason}
        {:empty, reason} -> {:empty, reason}
        {_, []} -> {:empty, "No task found"}
        {:ok, event} -> {:ok, event}
      end

    batchWasResponded = AlignedLayerServiceManager.is_batch_responded(merkle_root)

    {
      :ok,
      assign(socket, newBatchInfo: newBatchInfo, batchWasResponded: batchWasResponded)
    }

  end

  embed_templates "*"
end
