defmodule NavComponent do
  use ExplorerWeb, :live_component

  @impl true
  def render(assigns) do
    ~H"""
    <nav class={[
      "px-4 sm:px-6 lg:px-8 fixed top-0 p-3 z-50",
      "flex justify-between items-center w-full",
      "border-b border-foreground/10 backdrop-blur-lg backdrop-saturate-200"
    ]}>
      <div class="gap-x-6 inline-flex">
        <.link
          class="text-3xl hover:scale-105 transform duration-150 active:scale-95"
          navigate={~p"/"}
        >
          🟩 <span class="sr-only">Aligned Explorer Home</span>
        </.link>
        <div class={["items-center gap-8 [&>a]:drop-shadow-md", "hidden md:inline-flex"]}>
          <.link
            class="text-foreground/80 hover:text-foreground font-semibold"
            navigate={~p"/batches"}
          >
            Batches
          </.link>
          <.link
            class="text-foreground/80 hover:text-foreground font-semibold"
            navigate={~p"/operators"}
          >
            Operators
          </.link>
          <.link class="text-foreground/80 hover:text-foreground font-semibold" navigate={~p"/assets"}>
            Assets
          </.link>
        </div>
      </div>
      <.live_component module={SearchComponent} id="nav_search" />
      <div class="items-center gap-4 font-semibold leading-6 text-foreground/80 flex [&>a]:hidden lg:[&>a]:inline-block [&>a]:drop-shadow-md">
        <a class="hover:text-foreground" target="_blank" href="https://x.com/alignedlayer">
          @alignedlayer
        </a>
        <a
          class="hover:text-foreground"
          target="_blank"
          href="https://github.com/yetanotherco/aligned_layer"
        >
          GitHub
        </a>
        <DarkMode.button />
      </div>
    </nav>
    """
  end
end
