defmodule ContractsComponent do
  use ExplorerWeb, :live_component

  attr :class, :string, default: nil

  @impl true
  def mount(socket) do
    {:ok,
     assign(socket,
       service_manager_address:
         AlignedLayerServiceManager.get_aligned_layer_service_manager_address(),
       batcher_payment_service_address:
         AlignedLayerServiceManager.get_batcher_payment_service_address(),
       network: System.get_env("ENVIRONMENT")
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class={["relative truncate", @class]}>
      <.card
        inner_class="text-base leading-9 flex flex-wrap sm:flex-row overflow-x-auto gap-x-2"
        title="Contract Addresses"
      >
        <.link
          href="https://docs.alignedlayer.com/guides/6_contract_addresses"
          class="absolute top-4 right-5 hover:underline font-medium text-muted-foreground capitalize text-sm"
          target="_blank"
          rel="noopener noreferrer"
        >
          View All <.icon name="hero-arrow-top-right-on-square-solid" class="size-3.5 mb-1" />
        </.link>
        <h3>
          <.icon name="hero-cpu-chip" class="size-4 mb-0.5" /> Service Manager:
        </h3>
        <.a
          href={"https://#{@network |> String.replace(~r/holesky/, "holesky.") |> String.replace(~r/mainnet/, "")}etherscan.io/address/#{@service_manager_address}"}
          class="hover:text-foreground/80"
          target="_blank"
          rel="noopener noreferrer"
        >
          <%= @service_manager_address %>
        </.a>
        <h3>
          <.icon name="hero-wallet" class="size-4 mb-0.5" /> Batcher Payment Service:
        </h3>
        <.a
          href={"https://#{@network |> String.replace(~r/holesky/, "holesky.") |> String.replace(~r/mainnet/, "")}etherscan.io/address/#{@batcher_payment_service_address}"}
          class="hover:text-foreground/80"
          target="_blank"
          rel="noopener noreferrer"
        >
          <%= @batcher_payment_service_address %>
        </.a>
      </.card>
    </div>
    """
  end
end
