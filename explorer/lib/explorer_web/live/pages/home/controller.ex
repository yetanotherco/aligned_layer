defmodule ExplorerWeb.Home.Controller do
  require Logger
  use ExplorerWeb, :live_view

  def handle_event("search_task", %{"task" => task_params}, socket) do
    task_id = Map.get(task_params, "id")
    is_task_id_valid = String.match?(task_id, ~r/^\d+$/)

    if not is_task_id_valid do
      {:noreply, assign(socket, error: "Invalid task ID")}
    else
      {:noreply, redirect(socket, to: "/tasks/#{task_id}")}
    end
  end

  def mount(_, _, socket) do
    last_task_id = AlignedLayerServiceManager.get_latest_task_index()
    avs_directory = AlignedLayerServiceManager.get_avs_directory()

    last_task_hash = AlignedLayerServiceManager.get_tx_hash(last_task_id)
    last_task_response = AlignedLayerServiceManager.get_task_response(last_task_id)

    tasks_verified = get_verified_tasks_count()
    [tasks_true, tasks_false] = get_verified_tasks_count_by_status()

    {:ok,
     assign(socket,
       last_task_id: last_task_id,
       last_task_hash: last_task_hash,
       tasks_verified: tasks_verified,
       tasks_true: tasks_true,
       tasks_false: tasks_false,
       last_task_response: last_task_response,
       avs_directory: avs_directory
     )}
  end

  defp get_verified_tasks_count() do
    
    1
  end

  defp get_verified_tasks_count_by_status do
    [1, 0]
  end
end
