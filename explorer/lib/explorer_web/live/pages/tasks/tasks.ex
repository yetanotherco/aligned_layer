defmodule ExplorerWeb.Tasks.Tasks do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    events = AlignedLayerServiceManager.get_tasks_created_events()
    # events |> IO.inspect()

    tasks = Enum.map(events, fn event -> event |> extract_task_data end)
    tasks |> IO.inspect()

    {:ok, assign(socket, tasks: tasks)}
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

  def prepend(value, list) do
    value |> IO.inspect()
    [value | list]
  end

  embed_templates "*"
end
