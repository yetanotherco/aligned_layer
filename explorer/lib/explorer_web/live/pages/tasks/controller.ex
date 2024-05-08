defmodule ExplorerWeb.Tasks.Controller do
  require Logger
  use ExplorerWeb, :live_view

  def handle_event("next_page", %{"task" => task_params}, socket) do
    # task_id = Map.get(task_params, "id")
    # is_task_id_valid = String.match?(task_id, ~r/^\d+$/)

    # if not is_task_id_valid do
    #   {:noreply, assign(socket, error: "Invalid task ID")}
    # else
    #   {:noreply, redirect(socket, to: "/tasks/#{task_id}")}
    # end
  end

  def mount(params, _, socket) do
    params |> IO.inspect() #%{"page" => "1", "size" => "1"}
    # page = Map.get(params, "page")
    # size = Map.get(params, "size")
    [page, size] = [Map.get(params, "page"), Map.get(params, "size")]
    page |> IO.inspect()
    size |> IO.inspect()

    {:ok, socket }
  end
end
