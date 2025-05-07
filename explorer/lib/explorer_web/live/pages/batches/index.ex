defmodule ExplorerWeb.Batches.Index do
  alias Phoenix.PubSub
  require Logger
  import ExplorerWeb.BatchesTable
  use ExplorerWeb, :live_view

  @page_size 15

  @impl true
  def mount(params, _, socket) do
    current_page = get_current_page(params)

    batches =
      Batches.get_paginated_batches(%{page: current_page, page_size: @page_size})
      |> Helpers.enrich_batches()

    if connected?(socket), do: PubSub.subscribe(Explorer.PubSub, "update_views")

    remaining_time = Helpers.get_next_scheduled_batch_remaining_time()

    {:ok,
     assign(socket,
       current_page: current_page,
       batches: batches,
       next_scheduled_batch_remaining_time_percentage:
         Helpers.get_next_scheduled_batch_remaining_time_percentage(remaining_time),
       next_scheduled_batch_remaining_time: remaining_time,
       last_page: Batches.get_last_page(@page_size),
       page_title: "Batches"
     )}
  end

  @impl true
  def handle_info(_, socket) do
    current_page = socket.assigns.current_page

    batches =
      Batches.get_paginated_batches(%{page: current_page, page_size: @page_size})
      |> Helpers.enrich_batches()

    remaining_time = Helpers.get_next_scheduled_batch_remaining_time()

    {:noreply,
     assign(socket,
       batches: batches,
       next_scheduled_batch_remaining_time_percentage:
         Helpers.get_next_scheduled_batch_remaining_time_percentage(remaining_time),
       next_scheduled_batch_remaining_time: remaining_time,
       last_page: Batches.get_last_page(@page_size)
     )}
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

  embed_templates("*")
end
