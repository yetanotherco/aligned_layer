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
        page_title: Utils.shorten_hash(merkle_root),
        eth_usd_price: :empty
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
         proofs: :empty,
         eth_usd_price: :empty
       )}
  end

  @impl true
  def handle_info(%{eth_usd: eth_usd_price} = _params, socket) do
    eth_price = EthConverter.wei_to_eth(socket.assigns.current_batch.cost_per_proof)

    updated_socket =
      socket
      |> assign(current_batch: Batches.get_batch(%{merkle_root: socket.assigns.merkle_root}))
      |> then(fn updated_socket ->
        case eth_usd_price do
          :empty ->
            updated_socket

          _ ->
            assign(updated_socket,
              eth_usd_price: EthConverter.multiply_eth_by_usd(eth_price, eth_usd_price)
            )
        end
      end)

    {:noreply, updated_socket}
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
    case Proofs.get_proofs_from_batch(%{merkle_root: merkle_root}) do
      proofs when is_list(proofs) ->
        Enum.map(proofs, fn proof -> "0x" <> Base.encode16(proof.proof_hash, case: :lower) end)

      _ ->
        nil
    end
  end

  embed_templates "*"
end
