defmodule ExplorerWeb.Operator.Index do
  use ExplorerWeb, :live_view

  @impl true
  def handle_info(_, socket) do
    restaked_amount_eth = socket.assigns.operator.total_stake |> EthConverter.wei_to_eth(2)
    {_, restaked_amount_usd} = socket.assigns.operator.total_stake |> EthConverter.wei_to_usd(0)

    restakes_by_operator = Restakings.get_restakes_by_operator_id(socket.assigns.operator.id)

    weight = Operators.get_operator_weight(socket.assigns.operator) |> Numbers.show_percentage()

    {:noreply,
     assign(socket,
       restaked_amount_eth: restaked_amount_eth,
       restaked_amount_usd: restaked_amount_usd,
       restakes_by_operator: restakes_by_operator,
       weight: weight
     )}
  end

  @impl true
  def mount(%{"address" => address}, _, socket) do
    operator = Operators.get_operator_by_address(address)

    restaked_amount_eth = operator.total_stake |> EthConverter.wei_to_eth(2)
    {_, restaked_amount_usd} = operator.total_stake |> EthConverter.wei_to_usd(0)

    restakes_by_operator = Restakings.get_restakes_by_operator_id(operator.id)

    weight = Operators.get_operator_weight(operator) |> Numbers.show_percentage()

    operator_version = OperatorVersionTracker.get_operator_version(address)

    if connected?(socket), do: Phoenix.PubSub.subscribe(Explorer.PubSub, "update_restakings")

    {:ok,
     assign(socket,
       operator: operator,
       operator_id: operator.id |> Helpers.binary_to_hex_string(),
       restaked_amount_eth: restaked_amount_eth,
       restaked_amount_usd: restaked_amount_usd,
       restakes_by_operator: restakes_by_operator,
       weight: weight,
       operator_version: operator_version,
       page_title: operator.name
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 px-1 text-foreground max-w-[27rem] sm:max-w-3xl md:max-w-5xl mx-auto capitalize">
      <.card_preheding>
        Operator Details
      </.card_preheding>
      <.card
        class="px-4 py-5 min-h-fit flex flex-col"
        inner_class="font-semibold inline-flex flex-col text-base gap-y-2 text-muted-foreground [&>div>p]:text-foreground [&>p]:text-foreground [&>a]:text-foreground [&>p]:break-all [&>*]:font-normal [&>div]:flex [&>div]:flex-col [&>div]:lg:flex-row [&>div>h3]:basis-1/4"
      >
        <div class="flex flex-col md:flex-row gap-x-6 gap-y-2.5">
          <img
            alt={@operator.name}
            class="rounded-full size-24 object-scale-down"
            src={@operator.logo_link}
          />
          <div class="leading-7 flex flex-col gap-y-1.5 text-pretty">
            <h1 class="text-2xl font-bold text-foreground">
              <%= @operator.name %>
            </h1>
            <p>
              <%= @operator.description %>
            </p>
            <div class="flex flex-row gap-x-2.5 hover:[&>a]:text-foreground [&>a]:text-sm">
              <.a href={@operator.website} target="_blank" rel="noopener">
                Website
              </.a>
              <.a href={@operator.twitter} target="_blank" rel="noopener">
                X/Twitter
              </.a>
              <.a
                href={
          "#{Helpers.get_eigenlayer_explorer_url()}/operator/#{@operator.address}"
          }
                target="_blank"
                rel="noopener"
              >
                EigenLayer Profile
              </.a>
            </div>
          </div>
        </div>
        <.divider class="my-2 sm:mt-5 sm:mb-3" />
        <div class="break-all">
          <h3>
            Id:
          </h3>
          <p>
            <%= @operator_id %>
            <.live_component
              module={CopyToClipboardButtonComponent}
              text_to_copy={@operator_id}
              id={"copy_#{@operator_id}"}
              class="inline-flex"
            />
          </p>
        </div>
        <div class="break-all">
          <h3>
            Address:
          </h3>
          <p>
            <%= @operator.address %>
            <.live_component
              module={CopyToClipboardButtonComponent}
              text_to_copy={@operator.address}
              id={"copy_#{@operator.address}"}
              class="inline-flex"
            />
          </p>
        </div>
        <%= if @operator_version != nil do %>
          <div>
            <h3>
              Version:
            </h3>
            <.badge class="text-xs px-1.5 normal-case" variant="secondary">
              <%= @operator_version %>
            </.badge>
          </div>
        <% end %>
        <div>
          <h3>
            Total Restaked:
          </h3>
          <p>
            <%= @restaked_amount_eth %> ETH
            <span class="text-gray-500">(<%= @restaked_amount_usd |> Helpers.format_number() %> USD)</span>
          </p>
        </div>
        <div>
          <h3>
            Concentration Restaked:
          </h3>
          <p>
            <%= @weight %>
          </p>
        </div>
        <div>
          <h3>
            Restakes:
          </h3>
          <%= if @restakes_by_operator != [] do %>
            <div class="flex flex-col gap-y-2 basis-3/4">
              <%= for %{strategy: strategy, restaking: restaking} <- @restakes_by_operator do %>
                <div class="flex text-foreground gap-x-3 lg:pr-2">
                  <p class="font-semibold md:basis-1/5">
                    <%= EthConverter.wei_to_eth(restaking.stake, 2) |> Helpers.format_number() %> ETH
                  </p>
                  <p>
                    <%= strategy.name %>
                    <span class="text-xs text-muted-foreground"><%= strategy.symbol %></span>
                  </p>
                </div>
              <% end %>
            </div>
          <% else %>
            <p>
              No restakes found.
            </p>
          <% end %>
        </div>
      </.card>
    </div>
    """
  end
end
