defmodule ExplorerWeb.Restake.Index do
  use ExplorerWeb, :live_view

  @impl true
  def handle_info(_, socket) do
    total_stake =
      socket.assigns.restake.strategy_address
      |> Strategies.get_total_staked()

    restaked_amount_eth =
      total_stake |> EthConverter.wei_to_eth(2)

    {_, restaked_amount_usd} =
      total_stake
      |> EthConverter.wei_to_usd(0)

    {:noreply,
     assign(socket,
       restaked_amount_eth: restaked_amount_eth,
       restaked_amount_usd: restaked_amount_usd
     )}
  end

  @impl true
  def mount(%{"address" => address}, _, socket) do
    restake = Strategies.get_by_strategy_address(address)

    restaked_amount_eth = restake.total_staked |> EthConverter.wei_to_eth(2)

    {_, restaked_amount_usd} =
      restake.total_staked
      |> EthConverter.wei_to_usd(0)

    if connected?(socket), do: Phoenix.PubSub.subscribe(Explorer.PubSub, "update_restakings")

    {:ok,
     assign(socket,
       restake: restake,
       restaked_amount_eth: restaked_amount_eth,
       restaked_amount_usd: restaked_amount_usd,
       page_title: "Restake #{address}"
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 px-1 text-foreground max-w-[27rem] sm:max-w-3xl md:max-w-5xl mx-auto capitalize">
      <.card_preheding>
        Restaked Asset Details
      </.card_preheding>
      <.card
        class="px-4 py-5 min-h-fit flex flex-col"
        inner_class="font-semibold inline-flex flex-col text-base gap-y-2 text-muted-foreground [&>div>p]:text-foreground [&>p]:text-foreground [&>a]:text-foreground [&>p]:break-all [&>*]:font-normal [&>div]:flex [&>div]:flex-col [&>div]:lg:flex-row [&>div>h3]:basis-1/4"
      >
        <img
          src={~s"/images/restakes/#{@restake.symbol |> String.downcase()}.webp"}
          alt={@restake.name}
          class="py-2 text-xs truncate text-center self-start size-28 object-scale-down rounded-xl"
        />
        <div>
          <h3>
            Name:
          </h3>
          <p>
            <%= @restake.name %>
          </p>
        </div>
        <div>
          <h3>
            Symbol:
          </h3>
          <p>
            <%= @restake.symbol %>
          </p>
        </div>
        <div>
          <h3>
            Total Restaked:
          </h3>
          <div class="flex flex-column">
            <p>
              <%= @restaked_amount_eth |> Helpers.format_number() %> ETH
             <span class="text-gray-500"> (<%= @restaked_amount_usd |> Helpers.format_number() %> USD)</span>
            </p>
          </div>
        </div>
        <div class="break-all">
          <h3>
            Strategy Address:
          </h3>
          <.a
            href={"#{Helpers.get_eigenlayer_explorer_url()}/restake/#{@restake.symbol}"}
            class="text-foreground"
            target="_blank"
            rel="noopener noreferrer"
          >
            <%= @restake.strategy_address %>
            <.tooltip>
              View on EigenLayer Explorer
            </.tooltip>
          </.a>
        </div>
        <div class="break-all">
          <h3>
            Token Address:
          </h3>
          <%= if @restake.token_address != "0x" do %>
            <.a
              href={"#{Helpers.get_etherescan_url()}/address/#{@restake.token_address}"}
              class="text-foreground"
              target="_blank"
              rel="noopener noreferrer"
            >
              <%= @restake.token_address %>
              <.tooltip>
                View on Etherscan
              </.tooltip>
            </.a>
          <% else %>
            <p>N/A</p>
          <% end %>
        </div>
      </.card>
    </div>
    """
  end
end
