defmodule ExplorerWeb.Task.Controller do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    # Returns AlignedLayer is_aggregator -> bool
    # data = AlignedLayerServiceManager.is_aggregator("0x703E7dE5F528fA828f3BE726802B2092Ae7deb2F") |> Ethers.call()

    # Returns AlignedLayer task content
    id = params["id"]

    newTaskEvent =
      case Integer.parse(id) do
        {task_id, _} -> AlignedLayerServiceManager.get_task_created_event(task_id)
        _ -> {:empty, "task_id must be an integer"}
      end

    task =
      case newTaskEvent do
        {:ok, value} -> value
        {_, _} -> :empty
      end

    # Returns AlignedLayer task response content
    newRespondedEvent =
      case Integer.parse(id) do
        {task_id, _} -> AlignedLayerServiceManager.get_task_responded_event(task_id)
        _ -> {:empty, "task_id must be an integer"}
      end

    taskResponse =
      case newRespondedEvent do
        {:ok, value} -> value
        {_, _} -> :empty
      end

    isTaskEmpty = task == :empty
    isTaskResponseEmpty = taskResponse == :empty

    {:ok,
     assign(socket,
       id: id,
       task: task,
       taskResponse: taskResponse,
       isTaskEmpty: isTaskEmpty,
       isTaskResponseEmpty: isTaskResponseEmpty
     )}
  end

  embed_templates "*"
end
