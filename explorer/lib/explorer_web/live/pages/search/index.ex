defmodule ExplorerWeb.Search.Index do
  use ExplorerWeb, :live_view

  @page_size 15

  @impl true
  def mount(%{"q" => hash}, _session, socket) do
    total_pages =
      Proofs.get_number_of_batches_containing_proof(hash)
      |> div(@page_size)
      |> Kernel.ceil()
      |> max(1)

    {:ok,
     assign(socket,
       page_title: "Search Results For #{hash |> Helpers.shorten_hash()}",
       hash: hash,
       total_pages: total_pages
     )}
  end

  @impl true
  def handle_params(params, _url, socket) do
    hash = params["q"]
    page_param = Integer.parse(params["page"] || "1")

    current_page =
      case page_param do
        {page, _} when page > 0 -> page
        _ -> 1
      end

    case Proofs.get_batches_containing_proof(hash, current_page, @page_size) do
      [] ->
        {:noreply, push_navigate(socket, to: ~p"/batches/#{hash}")}

      results ->
        {:noreply,
         assign(socket,
           page_title: "Search Results For #{hash |> Helpers.shorten_hash()}",
           results: results,
           current_page: current_page
         )}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 text-foreground px-1 sm:max-w-lg md:max-w-3xl lg:max-w-5xl mx-auto capitalize">
      <.card_preheding>
        Search Results for "<%= @hash |> Helpers.shorten_hash() %>"
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
        <div class="flex gap-x-2 justify-center items-center">
          <%= if @current_page != 1 do %>
            <.link patch={~p"/search?q=#{@hash}&page=#{@current_page - 1}"}>
              <.button
                icon="arrow-left-solid"
                icon_class="group-hover:-translate-x-1 transition-all duration-150"
                class="text-muted-foreground size-10 group"
              >
                <span class="sr-only">Previous Page</span>
              </.button>
            </.link>
          <% else %>
            <.button
              icon="arrow-left-solid"
              class="text-muted-foreground size-10 group pointer-events-none opacity-50"
              disabled
            >
              <span class="sr-only">Previous Page</span>
            </.button>
          <% end %>
          <p>
            <%= @current_page %> / <%= @total_pages %>
          </p>
          <%= if @current_page != @total_pages do %>
            <.link patch={~p"/search?q=#{@hash}&page=#{@current_page + 1}"}>
              <.button
                icon="arrow-right-solid"
                icon_class="group-hover:translate-x-1 transition-all duration-150"
                class="text-muted-foreground size-10 group"
              >
                <span class="sr-only">Next Page</span>
              </.button>
            </.link>
          <% else %>
            <.button
              icon="arrow-right-solid"
              class="text-muted-foreground size-10 group pointer-events-none opacity-50"
              disabled
            >
              <span class="sr-only">Next Page</span>
            </.button>
          <% end %>
        </div>
      <% else %>
        <.empty_card_background text="No matching batches found." />
      <% end %>
    </div>
    """
  end
end
