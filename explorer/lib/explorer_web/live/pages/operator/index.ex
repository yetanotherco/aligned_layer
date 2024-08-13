defmodule ExplorerWeb.Operator.Index do
  use ExplorerWeb, :live_view

  @impl true
  def mount(%{"address" => address}, _, socket) do
    operator = Operators.get_operator_by_address(address)

    Operators.get_operator_weight(operator) |> dbg

    restaked_amount_eth = operator.total_stake |> EthConverter.wei_to_eth(2)

    restakes_by_operator = Restakings.get_restakes_by_operator_id(operator.id)

    {:ok,
     assign(socket,
       operator: operator,
       restaked_amount_eth: restaked_amount_eth,
       restakes_by_operator: restakes_by_operator,
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
          "#{Utils.get_eigenlayer_explorer_url()}/operator/#{@operator.address}"
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
        <div>
          <h3>
            Address:
          </h3>
          <p>
            <%= @operator.address %>
          </p>
        </div>
        <div>
          <h3>
            Total Restaked:
          </h3>
          <p>
            <%= @restaked_amount_eth %> ETH
          </p>
        </div>
        <div>
          <h3>
            Restakes:
          </h3>
          <%= if @restakes_by_operator != [] do %>
            <div class="flex flex-col gap-y-2">
              <%= for %{strategy: strategy, restaking: restaking} <- @restakes_by_operator do %>
                <div class="flex text-foreground">
                  <%= strategy.name %> (<%= strategy.symbol %>) <%= EthConverter.wei_to_eth(
                    restaking.stake,
                    3
                  ) %> ETH
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
