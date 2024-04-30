defmodule ExplorerWeb.TaskController do
  use ExplorerWeb, :controller

  def task(conn, %{"id" => id}) do
    #Test, returns the name of the ERC20 token
    # import MyERC20Token
    # MyERC20Token.get_erc20_name() |> elem(1) |> IO.puts()

    #Returns the EigenLayer AVSDirectory contract.
    # data = AlignedLayerServiceManager.avs_directory() |> Ethers.call()

    #Returns the AlignedLayer "meaning" value
    # data = AlignedLayerServiceManager.get_meaning() |> Ethers.call() |> IO.puts()


    # Returns AlignedLayer latestTaskNum
    # data = AlignedLayerServiceManager.latest_task_num() |> Ethers.call() |> IO.puts()

    #Returns AlignedLayer is_aggregator -> bool
    # data = AlignedLayerServiceManager.is_aggregator("0x703E7dE5F528fA828f3BE726802B2092Ae7deb2F") |> Ethers.call()

    #Returns AlignedLayer task content
    newTaskEvent = AlignedLayerServiceManager.get_task(String.to_integer(id))

    ret = if newTaskEvent |> elem(0) == :ok do
      IO.puts("Task found")
      # newTaskEvent |> elem(1) |> IO.inspect()
      newTaskEvent |> elem(1)
    else
      IO.puts("No task found")
      "No task found"
    end

    newRespondedEvent = AlignedLayerServiceManager.get_responded(String.to_integer(id))


    render(conn, :task, message: ":)", id: id, task: ret)
  end
end
