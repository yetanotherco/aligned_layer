defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    # Returns AlignedLayer is_aggregator -> bool
    # data = AlignedLayerServiceManager.is_aggregator("0x703E7dE5F528fA828f3BE726802B2092Ae7deb2F") |> Ethers.call()

    # Returns AlignedLayer task content
    "params" |> IO.inspect()
    params |> IO.inspect()

    merkle_root = params["merkle_root"]

    if merkle_root == nil do
      {:error, "merkle_root is required"}
      # TODO return empty
    end

    newBatchEvent =
      case AlignedLayerServiceManager.get_new_batch_events(merkle_root) do
        {:error, reason} -> {:error, reason}
        {:empty, reason} -> {:empty, reason}
        {_, []} -> {:empty, "No task found"}
        {:ok, event} -> {:ok, event}
      end

    batchWasResponded =
      case AlignedLayerServiceManager.is_batch_responded(merkle_root) do
        {:ok, [_, true]} -> true
        _ -> false
      end

    {:ok, assign(socket,
      newBatchEvent: newBatchEvent,
      batchWasResponded: batchWasResponded
    )}
  end

  embed_templates "*"
end
