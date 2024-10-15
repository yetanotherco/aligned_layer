defmodule ExplorerWeb.Operators.Index do
  use ExplorerWeb, :live_view

  @impl true
  def handle_info(_, socket) do
    operators = Operators.get_operators_with_their_weights()
    total_staked = Restakings.get_restaked_amount_eth()
    operators_registered = Operators.get_amount_of_operators()

    {:noreply,
     assign(socket,
       operators: operators,
       total_staked: total_staked,
       operators_registered: operators_registered
     )}
  end

  @impl true
  def mount(_, _, socket) do
    if connected?(socket), do: Phoenix.PubSub.subscribe(Explorer.PubSub, "update_restakings")
    {:ok, assign(socket, page_title: "Operators")}
  end

  @impl true
  def handle_params(_params, _url, socket) do
    operators = Operators.get_operators_with_their_weights()
    total_staked = Restakings.get_restaked_amount_eth()
    operators_registered = Operators.get_amount_of_operators()
    operator_versions = OperatorVersionTracker.get_operators_version()

    {:noreply,
     assign(socket,
       operators: operators,
       total_staked: total_staked,
       operators_registered: operators_registered,
       operator_versions: operator_versions
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 text-foreground px-1 sm:max-w-lg md:max-w-3xl lg:max-w-5xl mx-auto capitalize">
      <.card_preheding>Operators</.card_preheding>
      <.live_component
        module={AssetsCTAComponent}
        id="operators_cta"
        total_staked={@total_staked}
        operators_registered={@operators_registered}
      />
      <%= if @operators != [] do %>
        <.table id="operators" rows={@operators}>
          <:col :let={operator} label="Name" class="[animation-delay: 3s]">
            <.link navigate={~p"/operators/#{operator.address}"} class="flex gap-x-2">
              <span class="inline-flex gap-x-3 col-span-2 items-center group-hover:text-foreground/80">
                <img
                  src={operator.logo_link}
                  alt={operator.name}
                  class="rounded-full size-5 object-scale-down"
                />
                <span>
                  <%= operator.name %>
                  <%= if @operator_versions[operator.address] != nil do %>
                    <.badge class="text-xs px-1.5" variant="secondary">
                      <%= @operator_versions[operator.address] %>
                    </.badge>
                  <% end %>
                </span>
                <.right_arrow />
                <.tooltip class="py-2 px-2.5 rounded-2xl">
                  <span class="font-semibold text-muted-foreground">Id:</span> <%= operator.id
                  |> Helpers.binary_to_hex_string() %>
                  <br />
                  <span class="font-semibold text-muted-foreground">Address:</span> <%= operator.address %>
                </.tooltip>
              </span>
            </.link>
          </:col>
          <:col :let={operator} label="Restake Concentration">
            <%= operator.weight |> Numbers.show_percentage() %>
          </:col>
          <:col :let={operator} label="Total ETH Restaked">
            <%= operator.total_stake |> EthConverter.wei_to_eth(2) |> Helpers.format_number() %> ETH
          </:col>
          <:col :let={operator} label="Status">
            <.dynamic_badge_boolean status={operator.is_active} truthy_text="Active" falsy_text="Inactive" />
          </:col>
        </.table>
      <% else %>
        <.empty_card_background text="No operators found." />
      <% end %>
    </div>
    """
  end
end
