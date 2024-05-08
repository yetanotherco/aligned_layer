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
    last_task_id = AlignedLayerServiceManager.get_latest_task_index() #TODO show this value in front
    avs_directory = AlignedLayerServiceManager.get_avs_directory() #TODO show this value in front

    # task_responses = AlignedLayerServiceManager.get_task_responses()

    "a" |> IO.inspect()
    last_task_hash = AlignedLayerServiceManager.get_tx_hash(last_task_id)
    # AlignedLayerServiceManager.get_task_responses() |> IO.inspect()

    { :ok, assign(socket, last_task_id: last_task_id, avs_directory: avs_directory, last_task_hash: last_task_hash) }

  end

end
