defmodule ExplorerWeb.Tasks.Tasks do
  require Logger
  use ExplorerWeb, :live_view

  def mount(_params, _, socket) do
    events = AlignedLayerServiceManager.get_tasks_created_events()
    # events |> IO.inspect()

    "a" |> IO.inspect()
    list = []
    # Stream.map(events, fn event -> event |> Map.get(:topics) |> Enum.at(1) |> IO.inspect() end)
    Stream.map(events, fn event -> event |> Map.get(:topics) |> Enum.at(1) |> prepend(list) end) |> Stream.run()

    list |> IO.inspect()

    {:ok, socket}
  end

  def prepend(value, list) do
    value |> IO.inspect()
    [value | list]
  end

  embed_templates "*"
end
