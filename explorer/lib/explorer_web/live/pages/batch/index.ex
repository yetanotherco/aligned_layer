defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  @impl true
  def mount(params, _, socket) do
    merkle_root = params["merkle_root"]

    Phoenix.PubSub.subscribe(Explorer.PubSub, "update_views")

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
        proof_hashes: :empty,
        network: System.get_env("ENVIRONMENT"),
        site_url: System.get_env("PHX_HOST"),
        page_title: Utils.shorten_hash(merkle_root)
      )
    }
  rescue
    _ ->
      {:ok,
       socket
       |> assign(
         merkle_root: :empty,
         current_batch: :empty,
         newBatchInfo: :empty,
         batchWasResponded: :empty,
         proof_hashes: :empty,
         proofs: :empty
       )}
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

  @impl true
  def handle_event("show_proofs", _value, socket) do
    {:noreply, assign(socket, proof_hashes: get_proofs(socket.assigns.merkle_root))}
  end

  @impl true
  def handle_event("hide_proofs", _value, socket) do
    {:noreply, assign(socket, proof_hashes: :empty)}
  end

  defp get_proofs(merkle_root) do
    Proofs.get_proofs_from_batch(%{merkle_root: merkle_root})
    |> Enum.map(fn proof -> "0x" <> Base.encode16(proof.proof_hash, case: :lower) end)
  end

  embed_templates "*"
end
