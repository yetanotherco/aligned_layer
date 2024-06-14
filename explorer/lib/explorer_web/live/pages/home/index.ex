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
      {:noreply, push_navigate(socket, to: "/batches/#{batch_merkle_root}")}
    end
  end

  def mount(_, _, socket) do
    verified_batches = Batches.get_amount_of_verified_batches()

    operators_registered = get_operators_registered()

    latest_batches =
      AlignedLayerServiceManager.get_new_batch_events(%{amount: 5})
      |> Enum.map(fn event -> NewBatchEvent.extract_merkle_root(event) end)
      |> Enum.reverse()

    submitted_proofs = Batches.get_amount_of_submitted_proofs()
    verified_proofs = Batches.get_amount_of_verified_proofs()

    {:ok,
     assign(socket,
       verified_batches: verified_batches,
       operators_registered: operators_registered,
       latest_batches: latest_batches,
       submitted_proofs: submitted_proofs,
       verified_proofs: verified_proofs,
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
              submitted_proofs: :empty,
              verified_proofs: :empty
            )
            |> put_flash(:error, "Could not connect to the backend, please try again later.")
          }

        _ ->
          IO.puts("Other transport error: #{inspect(e)}")
      end

    e ->
      raise e
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
