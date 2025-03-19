defmodule ContractsComponent do
  use ExplorerWeb, :live_component

  attr(:class, :string, default: nil)
  attr(:host, :string, default: nil)

  @impl true
  def mount(socket) do
    addresses = Helpers.get_aligned_contracts_addresses()

    {:ok,
     assign(socket,
       contracts: [
         %{
           contract_name: "AlignedServiceManager",
           address: addresses["alignedLayerServiceManager"]
         },
         %{
           contract_name: "BatcherPaymentService",
           address: addresses["batcherPaymentService"]
         },
         %{
           contract_name: "BlsApkRegistry",
           address: addresses["blsApkRegistry"]
         },
         %{
           contract_name: "IndexRegistry",
           address: addresses["indexRegistry"]
         },
         %{
           contract_name: "OperatorStateRetriever",
           address: addresses["operatorStateRetriever"]
         },
         %{
           contract_name: "RegistryCoordinator",
           address: addresses["registryCoordinator"]
         },
         %{
           contract_name: "StakeRegistry",
           address: addresses["stakeRegistry"]
         }
       ]
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class={classes(["relative truncate", @class])}>
      <.card
        inner_class="text-base leading-9 flex flex-wrap sm:flex-row overflow-x-auto gap-x-2"
        title="Contract Addresses"
        subtitle={"All Aligned contracts addresses on #{Helpers.get_current_network_from_host(@host)}"}
      >
        <.link
          href="https://docs.alignedlayer.com/guides/6_contract_addresses"
          class="absolute top-4 right-5 hover:underline font-medium text-muted-foreground capitalize text-sm"
          target="_blank"
          rel="noopener noreferrer"
        >
          See more <.icon name="hero-arrow-top-right-on-square-solid" class="size-3.5 mb-1" />
        </.link>
        <div class="flex flex-col w-full">
          <%= for %{contract_name: contract_name, address: address} <- @contracts do %>
            <.contract contract_name={contract_name} address={address} />
          <% end %>
        </div>
      </.card>
    </div>
    """
  end

  attr(:contract_name, :string)
  attr(:address, :string)

  def contract(assigns) do
    ~H"""
    <div class="flex flex-wrap gap-x-3 w-full justify-between">
      <h3>
        <%= @contract_name %>
      </h3>
      <.a
        href={"#{Helpers.get_etherescan_url()}/address/#{@address}"}
        class="hover:text-foreground/80"
        target="_blank"
        rel="noopener noreferrer"
      >
        <%= @address %>
      </.a>
    </div>
    """
  end
end
