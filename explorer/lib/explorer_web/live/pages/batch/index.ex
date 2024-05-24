defmodule ExplorerWeb.Batch.Index do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    # Returns AlignedLayer is_aggregator -> bool
    # data = AlignedLayerServiceManager.is_aggregator("0x703E7dE5F528fA828f3BE726802B2092Ae7deb2F") |> Ethers.call()

    # Returns AlignedLayer task content
    "params" |> IO.inspect()
    params |> IO.inspect()

    merkle_root = params["merkle_root"]

    if merkle_root == nil do
      {:error, "merkle_root is required"}
      # TODO return empty
    end

    newBatchEvent =
      case AlignedLayerServiceManager.get_new_batch_events(merkle_root) do
        {:error, reason} -> {:error, reason}
        {:empty, reason} -> {:empty, reason}
        {_, []} -> {:empty, "No task found"}
        {:ok, event} -> {:ok, event}
      end

    batchResponded = AlignedLayerServiceManager.is_batch_responded(merkle_root)
    "batchResponded" |> IO.inspect()
    batchResponded |> IO.inspect()
    

    # # Returns AlignedLayer task response content
    # newRespondedEvent =
    #   case Integer.parse(id) do
    #     {task_id, _} -> AlignedLayerServiceManager.get_task_responded_event(task_id)
    #     _ -> {:empty, "task_id must be an integer"}
    #   end

    # taskResponse =
    #   case newRespondedEvent do
    #     {:ok, value} -> value
    #     {_, _} -> :empty
    #   end

    # isTaskEmpty = task == :empty
    # isTaskResponseEmpty = taskResponse == :empty

    {:ok, assign(socket,
      newBatchEvent: newBatchEvent
    )}
  end

  embed_templates "*"
end
