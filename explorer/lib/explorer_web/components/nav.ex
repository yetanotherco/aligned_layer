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
    <nav class={
      classes([
        "flex fixed justify-center items-center w-full",
        "border-b border-foreground/10 backdrop-blur-lg backdrop-saturate-200"
      ])
    }
    style="z-index: 1"
    >
    <div class={classes(["gap-5  mx-4 top-0 p-3 z-50",
        "flex justify-between items-center w-full"])} style="max-width: 1200px;">
      <div class="gap-x-6 flex">
        <.link
          class="hover:scale-105 transform duration-150 active:scale-95 text-3xl"
          navigate={~p"/"}
        >
          🟩 <span class="sr-only">Aligned Explorer Home</span>
        </.link>
        <div class={["items-center gap-5 hidden md:inline-flex"]}>
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
          <.link
            class={
              active_view_class(assigns.socket.view, [
                ExplorerWeb.Operators.Index,
                ExplorerWeb.Operator.Index
              ])
            }
            navigate={~p"/operators"}
          >
            Operators
          </.link>
          <.link
            class={
              active_view_class(assigns.socket.view, [
                ExplorerWeb.Restakes.Index,
                ExplorerWeb.Restake.Index
              ])
            }
            navigate={~p"/restaked"}
          >
            Restaked
          </.link>
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
          class="md:hidden z-50"
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
            <.link
              class={
                active_view_class(assigns.socket.view, [
                  ExplorerWeb.Operators.Index,
                  ExplorerWeb.Operator.Index
                ])
              }
              navigate={~p"/operators"}
            >
              Operators
            </.link>
            <.link
              class={
                active_view_class(assigns.socket.view, [
                  ExplorerWeb.Restakes.Index,
                  ExplorerWeb.Restake.Index
                ])
              }
              navigate={~p"/restaked"}
            >
              Restaked
            </.link>
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
          </div>
        </div>
      </div>
      </div>
    </nav>
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
