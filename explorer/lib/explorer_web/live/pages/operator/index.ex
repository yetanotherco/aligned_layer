defmodule ExplorerWeb.Operator.Index do
  use ExplorerWeb, :live_view

  @impl true
  def mount(%{"address" => address}, _, socket) do
    operator = Operators.get_operator_by_address(address)

    restaked_amount_eth =
      Restakings.get_aggregated_restake_by_operator(operator.id)
      |> Map.get(:total_stake)
      |> Decimal.to_integer()
      |> EthConverter.wei_to_eth(2)

    {:ok,
     assign(socket,
       operator: operator,
       restaked_amount_eth: restaked_amount_eth,
       page_title: operator.name
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 px-1 text-foreground max-w-[27rem] sm:max-w-3xl md:max-w-5xl mx-auto capitalize">
      <.card_preheding class="text-4xl sm:text-5xl font-bold font-foreground">
        Operator Details
      </.card_preheding>
      <.card
        class="px-4 py-5 min-h-fit flex flex-col"
        inner_class="font-semibold inline-flex flex-col text-base gap-y-2 text-muted-foreground [&>div>p]:text-foreground [&>p]:text-foreground [&>a]:text-foreground [&>*]:break-all [&>*]:font-normal"
      >
        <div class="flex flex-col md:flex-row gap-x-6 ">
          <img
            alt={@operator.name}
            class="rounded-full size-24 object-scale-down"
            src={@operator.logo_link}
          />
          <div class="hover:[&>a]:text-foreground [&>a]:pr-3 [&>a]:text-sm leading-7">
            <h1 class="text-2xl font-bold text-foreground">
              <%= @operator.name %>
            </h1>
            <p class="break-words">
              <%= @operator.description %>
            </p>
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
        Address:
        <p>
          <%= @operator.address %>
        </p>
        <div class="flex gap-x-2">
          <h3>
            Total Restaked:
          </h3>
          <p>
            <%= @restaked_amount_eth %> ETH
          </p>
        </div>
      </.card>
    </div>
    """
  end
end
