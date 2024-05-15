defmodule ExplorerWeb.Tasks.Tasks do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    current_page = get_current_page(params)
    # task_created_events = AlignedLayerServiceManager.get_tasks_created_events()
    # task_responded_events = AlignedLayerServiceManager.get_task_responded_events()

    page_size = 3
    from = (current_page-1) * page_size
    to = from + page_size - 1

    [task_created_events, task_responded_events] = AlignedLayerServiceManager.get_task_range(from, to)
    "[task_created_events, task_responded_events]" |> IO.inspect()
    [task_created_events, task_responded_events] |> IO.inspect()

    tasks_created_cross_tasks_responded = tasks_created_cross_tasks_responded(task_created_events, task_responded_events)
    "tasks_created_cross_tasks_responded" |> IO.inspect()
    tasks_created_cross_tasks_responded |> IO.inspect()
    # Enum.map(task_created_events, fn event -> event |> extract_task_data end)
      # |>
      # Enum.map(fn task_created -> check_if_task_responded(task_created, task_responded_events) end)


    {:ok, assign(socket, current_page: current_page, tasks: tasks_created_cross_tasks_responded)}
  end

  def tasks_created_cross_tasks_responded(task_created_events, task_responded_events) do
    Enum.map(task_created_events, fn event -> event |> extract_task_data end)
    |> Enum.map(fn task_created -> check_if_task_responded(task_created, task_responded_events) end)
  end

  def get_current_page(params) do
    case params |> Map.get("page") do
      nil -> 1
      page -> page |> Integer.parse() |> elem(0)
    end
  end

  def handle_event(event, params, socket) do
    current_page = case params |> Map.get("current_page") do
      nil -> 1
      page -> page
    end
    new_page = case event do
      "next_page" -> current_page |> (fn x -> x + 1 end).()
      "previous_page" -> current_page |> (fn x -> x - 1 end).()
    end
    {:noreply, redirect(socket, to: "/tasks?page=#{new_page}")}
  end

  def check_if_task_responded(task_created, task_responded_events) do
    task_response_event = Enum.find(task_responded_events, fn(event) -> match_event_id(event, task_created.taskId) end)
    case task_response_event do
      nil -> IO.puts("No task response found, id: #{task_created.taskId}")
      response ->
        Map.put(task_created, :proof_is_responded, true) |> Map.put(:proof_is_correct, response["data"] |> true_or_false() )
    end
  end

  def true_or_false(data) do
    case  String.slice(data, -1, 1) do
      "1" -> true
      "0" -> false
    end
  end

  def match_event_id(event, id) do
    padded_hex_string = Integer.to_string(id, 16) |> String.pad_leading(64, "0") |> String.downcase()
    idx = "0x" <> padded_hex_string

    case event["topics"] do
      [ _ | [ ^idx ] ] -> true
      rest -> false
    end
  end

  def extract_task_data(event) do
    %AlignedTaskPageItem{
      taskId: event |> elem(1) |> Map.get(:taskId),
      transaction_hash: event |> elem(1) |> Map.get(:transaction_hash),
      block_number: event |> elem(1) |> Map.get(:block_number),
      proof_is_responded: false,
      proof_is_correct: false
    }
  end

  embed_templates "*"
end
