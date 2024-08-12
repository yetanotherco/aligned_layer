defmodule ExplorerWeb.Assets.Index do
  use ExplorerWeb, :live_view

  @impl true
  def mount(_, _, socket) do
    {:ok, assign(socket, page_title: "Restaked Assets")}
  end

  @impl true
  def handle_params(_params, _url, socket) do
    assets = Strategies.get_all_strategies()

    dbg(assets)

    {:noreply, assign(socket, assets: assets)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 text-foreground px-1 sm:max-w-lg md:max-w-3xl lg:max-w-5xl mx-auto capitalize">
      <.card_preheding>Assets</.card_preheding>
      <.table id="assets" rows={@assets}>
        <:col :let={asset} label="Token" class="text-left">
          <%= asset.name %>
        </:col>
        <:col :let={asset} label="Total ETH Restaked">
          <%= if asset.total_staked != nil do %>
            <%= asset.total_staked %>
          <% else %>
            N/A
          <% end %>
        </:col>
      </.table>
    </div>
    """
  end
end
