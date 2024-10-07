defmodule ExplorerWeb.Restakes.Index do
  use ExplorerWeb, :live_view

  @impl true
  def handle_info(_, socket) do
    assets = Strategies.get_all_strategies()
    total_staked = Restakings.get_restaked_amount_eth()
    operators_registered = Operators.get_amount_of_operators()

    {:noreply,
     assign(socket,
       assets: assets,
       total_staked: total_staked,
       operators_registered: operators_registered
     )}
  end

  @impl true
  def mount(_, _, socket) do
    if connected?(socket), do: Phoenix.PubSub.subscribe(Explorer.PubSub, "update_restakings")

    {:ok, assign(socket, page_title: "Restaked Assets")}
  end

  @impl true
  def handle_params(_params, _url, socket) do
    assets = Strategies.get_all_strategies()
    total_staked = Restakings.get_restaked_amount_eth()
    operators_registered = Operators.get_amount_of_operators()

    {:noreply,
     assign(socket,
       assets: assets,
       total_staked: total_staked,
       operators_registered: operators_registered
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 text-foreground px-1 sm:max-w-lg md:max-w-3xl lg:max-w-5xl mx-auto capitalize">
      <.card_preheding>Restaked Assets</.card_preheding>
      <.live_component
        module={AssetsCTAComponent}
        id="assets_cta"
        total_staked={@total_staked}
        operators_registered={@operators_registered}
      />
      <%= if @assets != [] do %>
        <.table id="assets" rows={@assets}>
          <:col :let={asset} label="Token" class="text-left">
            <.link
              navigate={~p"/restake/#{asset.strategy_address}"}
              class="flex gap-x-2 items-center group-hover:text-foreground/80"
            >
              <img
                src={~s"/images/restakes/#{asset.symbol |> String.downcase()}.webp"}
                alt={asset.name}
                class="size-5 rounded-full object-scale-down text-xs truncate text-center"
              />
              <%= if asset.name != "‎" do %>
                <%= asset.name %>
              <% else %>
                <%= asset.strategy_address %>
              <% end %>
              <p class="text-muted-foreground text-sm">
                <%= asset.symbol %>
              </p>
              <.right_arrow />
            </.link>
          </:col>
          <:col :let={asset} label="Total ETH Restaked">
            <%= if asset.total_staked != nil do %>
              <%= asset.total_staked |> EthConverter.wei_to_eth(3) |> Helpers.format_number() %>
            <% else %>
              N/A
            <% end %>
          </:col>
        </.table>
      <% else %>
        <.empty_card_background text="No restaked assets found." />
      <% end %>
    </div>
    """
  end
end
