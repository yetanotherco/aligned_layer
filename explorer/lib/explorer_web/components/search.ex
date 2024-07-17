defmodule SearchComponent do
  use ExplorerWeb, :live_component

  @impl true
  def handle_event("search_batch", %{"batch" => batch_params}, socket) do
    batch_merkle_root = Map.get(batch_params, "merkle_root")
    is_batch_merkle_root_valid = String.match?(batch_merkle_root, ~r/^0x[a-fA-F0-9]+$/)

    if not is_batch_merkle_root_valid do
      {:noreply,
       socket
       |> assign(batch_merkle_root: batch_merkle_root)
       |> put_flash!(
         :error,
         "Please enter a valid proof batch hash, these should be hex values (0x69...)."
       )}
    else
      {:noreply, push_navigate(socket, to: ~p"/batches/#{batch_merkle_root}")}
    end
  end

  attr :class, :string, default: nil

  @impl true
  def render(assigns) do
    ~H"""
    <form
      phx-target={@myself}
      phx-submit="search_batch"
      class={[
        "relative flex items-center w-full max-w-md gap-2 z-10 px-5 sm:px-0 drop-shadow-sm",
        @class
      ]}
    >
      <input
        phx-hook="SearchFocus"
        id={"input_#{assigns.id}"}
        class="pr-10 shadow-md flex h-10 w-full file:border-0 text-foreground file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed flex-1 rounded-md border border-foreground/20 bg-card px-4 py-2 text-sm font-medium transition-colors hover:bg-foreground/10 focus:outline-none focus:ring-1 disabled:pointer-events-none disabled:opacity-50 hover:text-foreground"
        type="search"
        placeholder="Search by merkle root hash..."
        name="batch[merkle_root]"
      />
      <.button
        type="submit"
        class="absolute right-5 sm:right-1 -top-0.5 whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-transparent border-none text-muted-foreground hover:text-foreground size-10 rounded-full shadow-none hover:bg-transparent"
      >
        <.icon name="hero-magnifying-glass-solid" class="size-4" />
        <span class="sr-only">Search</span>
      </.button>
    </form>
    """
  end
end
