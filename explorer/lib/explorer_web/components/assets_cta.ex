defmodule AssetsCTAComponent do
  use ExplorerWeb, :live_component

  @impl true
  def update(assigns, socket) do
    {:ok, assign(socket, assigns)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <header>
      <.card_background class="min-h-24 flex flex-col md:flex-row gap-y-1 justify-between p-4">
        <.link navigate={~p"/operators"} class="flex-1 flex flex-col justify-start gap-0.5 group">
          <div class="text-muted-foreground font-semibold flex gap-2 items-center">
            <h2>
              Registered Active Operators
            </h2>
            <.right_arrow />
          </div>
          <span class={["text-4xl font-bold slashed-zero"]}>
            <%= @operators_registered %>
          </span>
          <.tooltip>
            View all active operators
          </.tooltip>
        </.link>
        <.link navigate={~p"/restaked"} class="flex-1 flex flex-col justify-start gap-0.5 group">
          <div class="text-muted-foreground font-semibold flex gap-2 items-center">
            <h2>
              Total Restaked
            </h2>
            <.right_arrow />
          </div>
            <div class="flex items-center justify-between flex-wrap">
                <span class="text-4xl font-bold slashed-zero">
                <%= @total_staked_usd |> Helpers.format_number() %> USD
                </span>
                <p class="text-s slashed-zero text-gray-500 mt-2">
                  (<%= @total_staked_eth |> Helpers.format_number() %> ETH)
                </p>
            </div>
          <.tooltip>
            View all restaked assets
          </.tooltip>
        </.link>
        <div class="" />
      </.card_background>
    </header>
    """
  end
end
