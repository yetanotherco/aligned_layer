defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  @impl true
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
        proofs: :empty,
        network: System.get_env("ENVIRONMENT"),
        site_url: System.get_env("PHX_HOST"),
        page_title: Utils.shorten_hash(merkle_root)
      )
    }
  rescue
    _ ->
      {:ok, assign(socket, merkle_root: :empty, newBatchInfo: :empty, batchWasResponded: :empty, current_batch: :empty, proofs: :empty)}
  end

  @impl true
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

  # @Gian the load button should do something like the following:
  # def load_proofs() do
  #   proofs = Proofs.get_proofs_from_batch(%{merkle_root: @merkle_root})
  #   assign(socket,
  #     proofs: proofs
  #   )
  # end

  embed_templates "*"
end
