defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    merkle_root = params["merkle_root"]

    Phoenix.PubSub.subscribe(Explorer.PubSub, "update_views")

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

    IO.inspect("current_batch")
    IO.inspect(current_batch)

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
      {:ok, assign(socket, merkle_root: :empty, newBatchInfo: :empty, batchWasResponded: :empty, current_batch: :empty)}
  end

  def handle_info(_, socket) do
    IO.puts("Received batch update for #{socket.assigns.merkle_root} from PubSub")

    {
      :noreply,
      assign(
        socket,
        current_batch: Batches.get_batch(%{merkle_root: socket.assigns.merkle_root})
      )
    }
  end

  embed_templates "*"
end
