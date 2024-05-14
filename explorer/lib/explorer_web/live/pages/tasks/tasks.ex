defmodule ExplorerWeb.Tasks.Tasks do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    task_created_events = AlignedLayerServiceManager.get_tasks_created_events()
    task_responded_events = AlignedLayerServiceManager.get_task_responded_events()

    tasks_created_cross_tasks_responded =
      Enum.map(task_created_events, fn event -> event |> extract_task_data end)
      |>
      Enum.map(fn task_created -> check_if_task_responded(task_created, task_responded_events) end)

    {:ok, assign(socket, tasks: tasks_created_cross_tasks_responded)}
  end

  # def handle_event("next_page", %{"task" => task_params}, socket) do
  # def handle_event("next_page", _, socket) do
  #   # task_id = Map.get(task_params, "id")
  #   is_task_id_valid = String.match?(task_id, ~r/^\d+$/)

  #   if not is_task_id_valid do
  #     {:noreply, assign(socket, error: "Invalid task ID")}
  #   else
  #     {:noreply, redirect(socket, to: "/tasks/#{task_id}")}
  #   end
  # end

  def check_if_task_responded(task_created, task_responded_events) do
    task_response_event = Enum.find(task_responded_events, fn(event) -> match_event_id(event, task_created.taskId) end)
    case task_response_event do
      nil -> IO.puts("No task response found")
      response ->
        Map.put(task_created, :proof_is_responded, true) |> Map.put(:proof_is_correct, response.data |> hd() |> elem(1) )
    end
  end

  def match_event_id(event, id) do
    parsed = Integer.parse(id) |> elem(0)
    case event.topics do
      [ _ | [ ^parsed ] ] -> true
      _ -> false
    end
  end

  def extract_task_data(event) do
    %AlignedTaskPageItem{
      taskId: event |> Map.get(:topics) |> Enum.at(1) |> Integer.to_string,
      transaction_hash: event |> Map.get(:transaction_hash),
      block_number: event |> Map.get(:block_number),
      proof_is_responded: false,
      proof_is_correct: false
    }
  end

  embed_templates "*"
end
