defmodule ExplorerWeb.AggProofs.Index do
  require Logger
  import ExplorerWeb.AggProofsTable
  use ExplorerWeb, :live_view

  @page_size 15

  @impl true
  def mount(params, _, socket) do
    current_page = get_current_page(params)

    proofs =
      AggregatedProofs.get_paginated_proofs(%{
        page: current_page,
        page_size: @page_size
      })
      |> Enum.map(fn proof ->
        proof |> Map.merge(%{age: proof.block_timestamp |> Helpers.parse_timeago()})
      end)

    {
      :ok,
      assign(
        socket,
        proofs: proofs,
        current_page: current_page,
        last_page: AggregatedProofs.get_last_page(@page_size)
      )
    }
  end

  @impl true
  def handle_event("change_page", %{"page" => page}, socket) do
    {:noreply, push_navigate(socket, to: ~p"/aggregated_proofs?page=#{page}")}
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
end
