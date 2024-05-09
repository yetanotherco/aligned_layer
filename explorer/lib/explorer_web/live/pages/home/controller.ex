defmodule ExplorerWeb.Home.Controller do
  require Logger
  use ExplorerWeb, :live_view

  def handle_event("search_task", %{"task" => task_params}, socket) do
    task_id = Map.get(task_params, "id")
    is_task_id_valid = String.match?(task_id, ~r/^\d+$/)

    if not is_task_id_valid do
      {:noreply,
       socket
       |> assign(task_id: task_id)
       |> put_flash(:error, "#{task_id} is not a valid task ID. Please enter a numeric value.")}
    else
      {:noreply, redirect(socket, to: "/tasks/#{task_id}")}
    end
  end

  def mount(_, _, socket) do
    last_task_id = AlignedLayerServiceManager.get_latest_task_index()
    avs_directory = AlignedLayerServiceManager.get_avs_directory()

    last_task_hash = AlignedLayerServiceManager.get_tx_hash(last_task_id)
    last_task_response = AlignedLayerServiceManager.get_task_response(last_task_id)

    # tasks_verified = get_verified_tasks_count()
    [tasks_true, tasks_false] = get_verified_tasks_count_by_status()

    operators_registered = get_operators_registered()

    {:ok,
     assign(socket,
       last_task_id: last_task_id,
       last_task_hash: last_task_hash,
       tasks_verified: tasks_true + tasks_false,
       tasks_true: tasks_true,
       tasks_false: tasks_false,
       last_task_response: last_task_response,
       operators_registered: operators_registered,
       avs_directory: avs_directory
     )}
  end

  # defp get_verified_tasks_count() do
  #   AlignedLayerServiceManager.get_task_responded_events() |> (fn {x, y} when x==:ok -> Enum.count(y) end).()
  # end

  defp get_verified_tasks_count_by_status() do
    AlignedLayerServiceManager.get_task_responded_events()
      |> elem(1)
      |> get_verified_tasks_count_by_status
  end

  defp get_verified_tasks_count_by_status(list), do: sum_status(list, [0, 0]) # tail-call recursion
  defp sum_status([], [a, b]), do: [a, b]
  defp sum_status([head | tail], [a, b]), do: sum_status(tail, evaluate_event(head, a, b))
  defp evaluate_event(event, a, b) do
    case event.data |> hd() |> elem(1) do
      true -> [a + 1, b]
      false -> [a, b + 1]
    end
  end

  defp count_operators_registered(list), do: sum_operators_registered(list, 0) # tail-call recursion
  defp sum_operators_registered([], val), do: val
  defp sum_operators_registered([head | tail], val), do: sum_operators_registered(tail, evaluate_operator(head, val))
  defp evaluate_operator(event, val) do
    case event.data |> hd() == 1 do # registered or unregistered
      true -> val + 1
      false -> val - 1
    end
  end

  def get_operators_registered() do
    AVSDirectory.get_operator_status_updated_events() |> (fn {status, data} when status == :ok -> count_operators_registered(data) end).()
  end
end
