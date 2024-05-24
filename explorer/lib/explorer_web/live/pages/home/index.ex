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
       |> put_flash(:error, "#{batch_merkle_root} is not a valid merkle root. Please enter a hex value.")}
    else
      {:noreply, redirect(socket, to: "/batches/#{batch_merkle_root}")}
    end
  end

  def mount(_, _, socket) do
    # last_task_id = AlignedLayerServiceManager.get_latest_task_index()

    verified_batches = get_verified_batches_count()

    shorthand_verified_batches = Utils.convert_number_to_shorthand(verified_batches)

    operators_registered = get_operators_registered()

    {:ok,
     assign(socket,
       verified_batches: shorthand_verified_batches,
       operators_registered: operators_registered
     )}
  end

  defp get_verified_batches_count() do
    AlignedLayerServiceManager.get_batch_verified_events() |>
      (fn
        {:ok, list} -> Enum.count(list)
        {:error, _} -> 0
    end).()
  end

  # TODO: refactor to new arquitecture
  # new arquitecture no longer applies, all verified batches are true. false batches are not responded
  # defp get_verified_batches_count_by_status() do
  #   AlignedLayerServiceManager.get_batch_verified_events()
  #   |> get_verified_tasks_count_by_status
  # end

  # # tail-call recursion
  # defp get_verified_tasks_count_by_status(list), do: sum_status(list, [0, 0])
  # defp sum_status([], [a, b]), do: [a, b]
  # defp sum_status([head | tail], [a, b]), do: sum_status(tail, evaluate_event(head, a, b))

  # defp evaluate_event(event, a, b) do
  #   case event.data |> hd() |> elem(1) do
  #     true -> [a + 1, b]
  #     false -> [a, b + 1]
  #   end
  # end

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
