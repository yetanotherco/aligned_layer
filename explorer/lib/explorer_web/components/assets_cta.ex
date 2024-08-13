defmodule AssetsCTAComponent do
  use ExplorerWeb, :live_component

  @impl true
  def mount(socket) do
    total_staked = get_restaked_amount_eth()
    operators_registered = Operators.get_amount_of_operators()

    {:ok,
     assign(socket,
       total_staked: total_staked,
       operators_registered: operators_registered
     )}
  end

  defp get_restaked_amount_eth() do
    restaked_amount_wei =
      Restakings.get_aggregated_restakings()
      |> Map.get(:total_stake)

    case restaked_amount_wei do
      nil ->
        nil

      _ ->
        restaked_amount_wei
        |> EthConverter.wei_to_eth(2)
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <header>
      <.card_background class="min-h-24 flex flex-col md:flex-row gap-y-1 justify-between p-4">
        <.link navigate={~p"/operators"} class="flex flex-col justify-start gap-0.5 group">
          <div class="text-muted-foreground font-semibold flex gap-2 items-center">
            <h2>
              Total Operators
            </h2>
            <.right_arrow />
          </div>
          <span class={["text-4xl font-bold slashed-zero"]}>
            <%= @operators_registered %>
          </span>
          <.tooltip>
            View all operators
          </.tooltip>
        </.link>
        <.link navigate={~p"/restakes"} class="flex flex-col justify-start gap-0.5 group">
          <div class="text-muted-foreground font-semibold flex gap-2 items-center">
            <h2>
              Total Restaked
            </h2>
            <.right_arrow />
          </div>
          <span class={["text-4xl font-bold slashed-zero"]}>
            <%= @total_staked |> Helpers.format_number() %> ETH
          </span>
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
