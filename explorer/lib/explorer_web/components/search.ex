defmodule SearchComponent do
  use ExplorerWeb, :live_component

  @impl true
  def handle_event("search_batch", %{"batch" => %{"merkle_root" => input_hash}}, socket) do
    input_hash
    |> (fn hash ->
          if String.match?(hash, ~r/^0x[a-fA-F0-9]+$/), do: {:ok, hash}, else: :invalid_hash
        end).()
    |> case do
      {:ok, hash} ->
        case Proofs.get_number_of_batches_containing_proof(hash) do
          0 -> {:noreply, push_navigate(socket, to: ~p"/batches/#{hash}")}
          _ -> {:noreply, push_navigate(socket, to: ~p"/search?q=#{hash}")}
        end

      :invalid_hash ->
        {:noreply,
         socket
         |> assign(batch_merkle_root: input_hash)
         |> put_flash!(:error, "Please enter a valid proof batch hash (0x69...).")}
    end
  end

  attr :class, :string, default: nil

  @impl true
  def render(assigns) do
    ~H"""
    <form
      phx-target={@myself}
      phx-submit="search_batch"
      class={
        classes([
          "relative flex items-center gap-2 z-10 px-5 sm:px-0 drop-shadow-sm max-w-md",
          @class
        ])
      }
    >
      <input
        phx-hook="SearchFocus"
        id={"input_#{assigns.id}"}
        class="pr-10 shadow-md flex h-10 w-full md:min-w-80 file:border-0 text-foreground file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed flex-1 rounded-md border border-foreground/20 bg-card px-4 py-2 text-sm font-medium transition-colors hover:bg-muted focus:outline-none focus:ring-1 disabled:pointer-events-none disabled:opacity-50 hover:text-foreground"
        type="search"
        placeholder="Enter batch or proof hash (cmd+K)"
        name="batch[merkle_root]"
      />
      <.button
        type="submit"
        class="absolute right-5 sm:right-1 top-0.5 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-transparent border-none size-10 shadow-none hover:bg-transparent text-muted-foreground"
      >
        <.icon name="hero-magnifying-glass-solid" class="size-5 hover:text-foreground" />
        <span class="sr-only">Search</span>
      </.button>
    </form>
    """
  end
end
