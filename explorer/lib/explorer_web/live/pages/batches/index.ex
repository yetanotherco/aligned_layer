defmodule ExplorerWeb.Batches.Index do
  alias Phoenix.PubSub
  require Logger
  use ExplorerWeb, :live_view

  @page_size 15

  @impl true
  def mount(params, _, socket) do
    current_page = get_current_page(params)

    batches = Batches.get_paginated_batches(%{page: current_page, page_size: @page_size})

    PubSub.subscribe(Explorer.PubSub, "update_views")

    {:ok, assign(socket, current_page: current_page, batches: batches, page_title: "Batches")}
  end

  @impl true
  def handle_info(_, socket) do
    IO.puts("Received update for batches from PubSub")

    current_page = socket.assigns.current_page

    batches = Batches.get_paginated_batches(%{page: current_page, page_size: @page_size})

    {:noreply, assign(socket, batches: batches)}
  end

  @impl true
  def handle_event("change_page", %{"page" => page}, socket) do
    {:noreply, push_navigate(socket, to: ~p"/batches?page=#{page}")}
  end

  defp get_current_page(params) do
    case params |> Map.get("page") do
      nil ->
        1

      page ->
        case Integer.parse(page) do
          {number, _} ->
            if number < 1, do: 1, else: number

          :error ->
            1
        end
    end
  end

  embed_templates "*"
end
