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

  attr(:class, :string, default: nil)

  @impl true
  def render(assigns) do
    ~H"""
    <form
      phx-target={@myself}
      phx-submit="search_batch"
      class={
        classes([
          "relative flex items-center gap-2 sm:px-0 w-full",
          @class
        ])
      }
    >
      <input
        phx-hook="SearchFocus"
        id={"input_#{assigns.id}"}
        class="pr-10 w-full text-foreground rounded-lg border-foreground/20 bg-card focus:border-foreground/20 focus:ring-accent text-sm"
        type="search"
        placeholder="Search batch by batch hash or proof hash"
        name="batch[merkle_root]"
      />
      <.icon name="hero-magnifying-glass-solid" class="absolute right-3 text-foreground/20 size-5 hover:text-foreground" />
    </form>
    """
  end
end
