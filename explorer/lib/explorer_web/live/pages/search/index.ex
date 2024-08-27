defmodule ExplorerWeb.Search.Index do
  use ExplorerWeb, :live_view

  def mount(%{"hash" => hash}, _session, socket) do
    case Proofs.get_batches_containing_proof(hash) do
      [] ->
        {:ok, push_navigate(socket, to: ~p"/batches/#{hash}")}

      results ->
        {:ok, assign(socket, page_title: "Search Results", results: results, hash: hash)}
    end
  end

  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 text-foreground px-1 sm:max-w-lg md:max-w-3xl lg:max-w-5xl mx-auto capitalize">
      <.card_preheding>
        Search Results for "<%= assigns.hash |> Helpers.shorten_hash() %>"
      </.card_preheding>
      <%= if @results != nil or @results != [] do %>
        <.table id="results" rows={@results}>
          <:col :let={result} label="Batch Hash" class="text-left">
            <.link
              navigate={~p"/batches/#{result}"}
              class="flex justify-between group group-hover:text-foreground/80"
            >
              <span class="items-center group-hover:text-foreground/80 hidden md:inline">
                <%= result %>
              </span>
              <span class="items-center group-hover:text-foreground/80 md:hidden">
                <%= result |> Helpers.shorten_hash(12) %>
              </span>
              <.right_arrow />
              <.tooltip>
                <%= result %>
              </.tooltip>
            </.link>
          </:col>
        </.table>
      <% else %>
        <.card_background class="overflow-x-auto min-h-[38.45rem] flex flex-col items-center justify-center gap-2">
          <p class="text-lg text-muted-foreground">No batches found.</p>
        </.card_background>
      <% end %>
    </div>
    """
  end
end
