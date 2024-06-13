defmodule ExplorerWeb.Batches.Index do
  require Logger
  use ExplorerWeb, :live_view

  def mount(params, _, socket) do
    current_page = get_current_page(params)

    page_size = 12

    batches = Batches.get_latest_batches(%{amount: page_size * current_page})

    {:ok, assign(socket, current_page: current_page, batches: batches, page_title: "Batches")}
  end

  def get_current_page(params) do
    case params |> Map.get("page") do
      nil ->
        1

      page ->
        number = page |> Integer.parse() |> elem(0)
        if number < 1, do: 1, else: number
    end
  end

  embed_templates "*"
end
