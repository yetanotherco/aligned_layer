defmodule ExplorerWeb.Batches.Index do
  alias Phoenix.PubSub
  require Logger
  use ExplorerWeb, :live_view

  @page_size 12

  def mount(params, _, socket) do
    current_page = get_current_page(params)

    batches = Batches.get_latest_batches(%{amount: @page_size * current_page})

    PubSub.subscribe(Explorer.PubSub, "update_batches")

    {:ok, assign(socket, current_page: current_page, batches: batches, page_title: "Batches")}
  end

  def handle_info(_, socket) do
    IO.puts("Received update for batches from PubSub")

    current_page = socket.assigns.current_page

    batches = Batches.get_latest_batches(%{amount: @page_size * current_page})

    {:noreply, assign(socket, batches: batches)}
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
