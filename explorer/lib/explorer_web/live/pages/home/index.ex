defmodule ExplorerWeb.Home.Index do
  require Logger
  use ExplorerWeb, :live_view

  def handle_event("search_batch", %{"batch" => batch_params}, socket) do
    batch_merkle_root = Map.get(batch_params, "merkle_root")
    is_batch_merkle_root_valid = String.match?(batch_merkle_root, ~r/^0x[a-fA-F0-9]+$/)

    if not is_batch_merkle_root_valid do
      {:noreply,
       socket
       |> assign(batch_merkle_root: batch_merkle_root)
       |> put_flash(
         :error,
         "Please enter a valid proof batch hash, these should be hex values (0x69...)."
       )}
    else
      {:noreply, redirect(socket, to: "/batches/#{batch_merkle_root}")}
    end
  end

  def mount(_, _, socket) do
    verified_batches = get_verified_batches_count()

    operators_registered = get_operators_registered()

    latest_batches =
      AlignedLayerServiceManager.get_new_batch_events(%{amount: 5})
      |> Enum.map(fn event -> NewBatchEvent.extract_merkle_root(event) end)
      |> Enum.reverse()

    amount_of_proofs = Explorer.Repo.aggregate(Batches, :sum, :amount_of_proofs)

    {:ok,
     assign(socket,
       verified_batches: verified_batches,
       operators_registered: operators_registered,
       latest_batches: latest_batches,
       amount_of_proofs: amount_of_proofs,
       page_title: "Welcome"
     )}
    rescue
      e in Mint.TransportError ->
        case e do
          %Mint.TransportError{reason: :econnrefused} ->
            {
              :ok,
              assign(socket,
                verified_batches: :empty,
                operators_registered: :empty,
                latest_batches: :empty,
                amount_of_proofs: :empty,
                page_title: "Error connection refused, couldn't connect to RPC"
              )
            }
          _ ->
            IO.puts("Other transport error: #{inspect(e)}")
        end
      e -> raise e
  end

  defp get_verified_batches_count() do
    AlignedLayerServiceManager.get_batch_verified_events()
    |> (fn
          {:ok, nil} -> 0
          {:ok, list} -> Enum.count(list)
          {:error, error} -> raise error
        end).()
  end

  # tail-call recursion
  defp count_operators_registered(list), do: sum_operators_registered(list, 0)
  defp sum_operators_registered([], val), do: val

  defp sum_operators_registered([head | tail], val),
    do: sum_operators_registered(tail, evaluate_operator(head, val))

  defp evaluate_operator(event, val) do
    # registered or unregistered
    case event.data |> hd() == 1 do
      true -> val + 1
      false -> val - 1
    end
  end

  def get_operators_registered() do
    AVSDirectory.get_operator_status_updated_events()
    |> (fn {status, data} when status == :ok -> count_operators_registered(data) end).()
  end

  embed_templates "*"
end
