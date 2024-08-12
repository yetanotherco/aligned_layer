defmodule ExplorerWeb.Operators.Index do
  use ExplorerWeb, :live_view

  @impl true
  def handle_info(_, socket) do
    operators = Operators.get_operators()
    {:noreply, assign(socket, operators: operators)}
  end

  @impl true
  def mount(_, _, socket) do
    {:ok, assign(socket, page_title: "Operators")}
  end

  @impl true
  def handle_params(_params, _url, socket) do
    operators = Operators.get_operators()
    {:noreply, assign(socket, operators: operators)}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 text-foreground px-1 sm:max-w-lg md:max-w-3xl lg:max-w-5xl mx-auto capitalize">
      <.card_preheding>Operators</.card_preheding>
      <.table id="operators" rows={@operators}>
        <:col :let={operator} label="Name" class={"[animation-delay: 3s]"}>
          <.link navigate={~p"/operators/#{operator.address}"} class="flex gap-x-2">
            <span class="inline-flex gap-x-3 col-span-2 items-center group-hover:text-foreground/80">
              <img
                src={operator.logo_link}
                alt={operator.name}
                class="rounded-full size-5 object-scale-down"
              />
              <%= operator.name %>
              <.right_arrow />
              <.tooltip>
                <%= operator.address %>
              </.tooltip>
            </span>
          </.link>
        </:col>
        <:col :let={operator} label="Total ETH Restaked">
          <%= operator.total_stake
          |> Decimal.to_integer()
          |> EthConverter.wei_to_eth(2) %> ETH
        </:col>
        <:col :let={operator} label="Status">
          <.dynamic_badge status={operator.is_active} truthy_text="Active" falsy_text="Inactive" />
        </:col>
      </.table>
    </div>
    """
  end
end
