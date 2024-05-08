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
    {status, last_task_id} = AlignedLayerServiceManager.latest_task_index_plus_one() |> Ethers.call()
    case status do
      :ok -> Logger.debug("Latest task index: #{last_task_id}")
      :error -> raise("Error fetching latest task index")
    end

    {status, avs_directory} = AlignedLayerServiceManager.avs_directory() |> Ethers.call()
    case status do
      :ok -> Logger.debug("AVS directory #{avs_directory}")
      :error -> raise("Error fetching latest task index")
    end
    { :ok, assign(socket, last_task_id: last_task_id) }

  end

end
