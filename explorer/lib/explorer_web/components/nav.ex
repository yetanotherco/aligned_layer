defmodule NavComponent do
  use ExplorerWeb, :live_component

  def get_networks(current_network) do
    Helpers.get_aligned_networks()
    |> Enum.filter(fn {name, _link} ->
      case current_network do
        # Filter dev networks if we are in mainnet or holesky
        "Mainnet" -> name in ["Mainnet", "Holesky"]
        "Holesky" -> name in ["Mainnet", "Holesky"]
        _ -> true
      end
    end)
    |> Enum.map(fn {name, link} ->
      {name, "window.location.href='#{link}'"}
    end)
  end

  @impl true
  def mount(socket) do
    {:ok,
     assign(socket,
       latest_release: ReleasesHelper.get_latest_release()
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <nav
      class={
        classes([
          "flex fixed justify-center items-center w-full",
          "border-b border-foreground/10 backdrop-blur-lg backdrop-saturate-200"
        ])
      }
      style="z-index: 1"
    >
      <div
        class={classes(["gap-5  mx-4 top-0 p-3 z-50", "flex justify-between items-center w-full"])}
        style="max-width: 1200px;"
      >
        <div class="gap-x-6 flex">
          <.link
            class="hover:scale-105 transform duration-150 active:scale-95 text-3xl"
            navigate={~p"/"}
          >
            🟩 <span class="sr-only">Aligned Explorer Home</span>
          </.link>
          <div class={["items-center gap-5 hidden lg:inline-flex"]}>
            <.link
              class={
                active_view_class(assigns.socket.view, [
                  ExplorerWeb.Batches.Index,
                  ExplorerWeb.Batch.Index
                ])
              }
              navigate={~p"/batches"}
            >
              Batches
            </.link>
            <%= if !ExplorerWeb.Helpers.is_mainnet() do %>
                <.link
                class={
                  active_view_class(@socket.view, [
                    ExplorerWeb.AggProofs.Index,
                    ExplorerWeb.AggProof.Index
                  ])
                }
                navigate={~p"/aggregated_proofs"}
                >
                Aggregation
                </.link>
             <% end %>
            <.nav_links_dropdown
              title="Restaking"
              class={
                active_view_class(assigns.socket.view, [
                  ExplorerWeb.Operators.Index,
                  ExplorerWeb.Operator.Index,
                  ExplorerWeb.Restakes.Index,
                  ExplorerWeb.Restake.Index
                ])
              }
              links={[
                {"Operators", ~p"/operators",
                 active_view_class(assigns.socket.view, [
                   ExplorerWeb.Operators.Index,
                   ExplorerWeb.Operator.Index
                 ])},
                {"Tokens", ~p"/restaked",
                 active_view_class(assigns.socket.view, [
                   ExplorerWeb.Restakes.Index,
                   ExplorerWeb.Restake.Index
                 ])}
              ]}
            />
          </div>
        </div>
        <div style="max-width: 600px; width: 100%;">
          <.live_component module={SearchComponent} id="nav_search" />
        </div>
        <div class="items-center gap-4 font-semibold leading-6 text-foreground/80 flex [&>a]:hidden lg:[&>a]:inline-block">
          <.link class="hover:text-foreground" target="_blank" href="https://docs.alignedlayer.com">
            Docs
          </.link>
          <.link
            class="hover:text-foreground"
            target="_blank"
            href="https://github.com/yetanotherco/aligned_layer"
          >
            GitHub
          </.link>
          <DarkMode.button theme={@theme} />
          <.badge :if={@latest_release != nil} class="hidden md:inline">
            <%= @latest_release %>
            <.tooltip>
              Latest Aligned version
            </.tooltip>
          </.badge>
          <.hover_dropdown_selector
            current_value={Helpers.get_current_network_from_host(@host)}
            variant="accent"
            options={get_networks(Helpers.get_current_network_from_host(@host))}
            icon="hero-cube-transparent-micro"
          />
          <button
            class="lg:hidden z-50"
            id="menu-toggle"
            phx-click={toggle_menu()}
            aria-label="Toggle hamburger menu"
          >
            <.icon name="hero-bars-3" class="toggle-open" />
            <.icon name="hero-x-mark" class="toggle-close hidden" />
          </button>
          <div
            id="menu-overlay"
            class="fixed inset-0 bg-background/90 z-40 hidden min-h-dvh animate-in fade-in"
            phx-click={toggle_menu()}
          >
            <div class="h-full flex flex-col gap-y-10 text-2xl justify-end items-center p-12">
              <.badge :if={@latest_release != nil}>
                <%= @latest_release %>
              </.badge>
              <.link
                class={
                  classes([
                    active_view_class(assigns.socket.view, [
                      ExplorerWeb.Batches.Index,
                      ExplorerWeb.Batch.Index
                    ]),
                    "text-foreground/80 hover:text-foreground font-semibold"
                  ])
                }
                navigate={~p"/batches"}
              >
                Batches
              </.link>
              <%= if !ExplorerWeb.Helpers.is_mainnet() do %>
                <.link
                  class={
                    classes([
                      active_view_class(assigns.socket.view, [
                        ExplorerWeb.AggregatedProofs.Index,
                        ExplorerWeb.AggregatedProof.Index
                      ]),
                      "text-foreground/80 hover:text-foreground font-semibold"
                    ])
                  }
                  navigate={~p"/aggregated_proofs"}
                >
                  Aggregation
                </.link>
              <% end %>
              <.link
                class="hover:text-foreground"
                target="_blank"
                href="https://docs.alignedlayer.com"
              >
                Docs
              </.link>
              <.link
                class="hover:text-foreground"
                target="_blank"
                href="https://github.com/yetanotherco/aligned_layer"
              >
                GitHub
              </.link>
            </div>
          </div>
        </div>
      </div>
    </nav>
    """
  end

  @doc """
    Renders a dropdown on hover component with links.
  """
  attr(:title, :list, doc: "the selector title")
  attr(:class, :list, doc: "class for selector")
  attr(:links, :string, doc: "the links to render: (name, link, class)")

  def nav_links_dropdown(assigns) do
    ~H"""
    <div class="relative group">
      <div class="flex items-center gap-2">
        <p class={classes(["cursor-default", @class])}><%= @title %></p>
      </div>

      <div
        class="opacity-0 pointer-events-none absolute transition-all w-full group-hover:opacity-100 group-hover:pointer-events-auto  pt-2"
        style="min-width: 150px;"
      >
        <div class="p-5 w-full bg-card border border-muted-foreground/30 rounded-lg flex flex-col justify-center items-center gap-5">
          <%= for {name, route, class} <- @links do %>
            <.link
              class={
                classes([
                  "group/link text-card-foreground w-full flex items-center justify-between",
                  class
                ])
              }
              navigate={route}
            >
              <%= name %>
            </.link>
          <% end %>
        </div>
      </div>
    </div>
    """
  end

  def toggle_menu() do
    JS.toggle(to: "#menu-overlay")
    |> JS.toggle(to: ".toggle-open")
    |> JS.toggle(to: ".toggle-close")
  end

  defp active_view_class(current_view, target_views) do
    if current_view in target_views,
      do: "text-green-500 font-bold",
      else: "text-foreground/80 hover:text-foreground font-semibold"
  end
end
