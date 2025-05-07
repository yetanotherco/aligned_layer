defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  defp set_empty_values(socket) do
    Logger.info("Setting empty values")

    socket
    |> assign(
      merkle_root: :empty,
      current_batch: :empty,
      newBatchInfo: :empty,
      batchWasResponded: :empty,
      proof_hashes: :empty,
      proofs: :empty,
      eth_usd_price: :empty
    )
  end

  @impl true
  def mount(%{"merkle_root" => merkle_root}, _, socket) do
    if connected?(socket), do: Phoenix.PubSub.subscribe(Explorer.PubSub, "update_views")

    current_batch =
      case Batches.get_batch(%{merkle_root: merkle_root}) do
        nil ->
          :empty

        %Batches{fee_per_proof: fee_per_proof} = batch ->
          {_, fee_per_proof_usd} = EthConverter.wei_to_usd_sf(fee_per_proof, 2)

          %{
            batch
            | fee_per_proof: EthConverter.wei_to_eth(fee_per_proof)
          }
          |> Map.merge(%{
            fee_per_proof_usd: fee_per_proof_usd,
            status: batch |> Helpers.get_batch_status
          })
      end

    {
      :ok,
      assign(socket,
        merkle_root: merkle_root,
        current_batch: current_batch,
        proof_hashes: :empty,
        network: System.get_env("ENVIRONMENT"),
        site_url: System.get_env("PHX_HOST"),
        page_title: Helpers.shorten_hash(merkle_root)
      )
    }
  rescue
    _ ->
      {:ok,
        set_empty_values(socket)
        |> put_flash(:error, "Something went wrong, please try again later.")
    }
  end

  @impl true
  def handle_info(_params, socket) do
    new_batch =
      case Batches.get_batch(%{merkle_root: socket.assigns.merkle_root}) do
        nil ->
          :empty

        %{fee_per_proof: fee_per_proof} = batch ->
          {_, fee_per_proof_usd} = EthConverter.wei_to_usd_sf(fee_per_proof, 2)

          %{
            batch
            | fee_per_proof: EthConverter.wei_to_eth(fee_per_proof)
          }
          |> Map.merge(%{
            fee_per_proof_usd: fee_per_proof_usd,
            status: batch |> Helpers.get_batch_status
          })

      end

    {
      :noreply,
      assign(
        socket,
        current_batch: new_batch
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
    case Proofs.get_proofs_from_batch(%{merkle_root: merkle_root}) do
      proofs when is_list(proofs) ->
        Enum.map(proofs, fn proof -> "0x" <> Base.encode16(proof.proof_hash, case: :lower) end)

      _ ->
        nil
    end
  end
end
