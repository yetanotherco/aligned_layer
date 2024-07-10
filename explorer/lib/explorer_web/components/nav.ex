defmodule NavComponent do
  use ExplorerWeb, :live_component

  @impl true
  def render(assigns) do
    ~H"""
    <nav class="px-4 sm:px-6 lg:px-8 fixed top-0 w-full inline-flex justify-between p-3 border-b border-foreground/10 backdrop-blur-md backdrop-saturate-200 z-50">
      <div class="flex items-center gap-8 [&>a]:drop-shadow-md">
        <.link
          class="text-3xl hover:scale-105 transform duration-150 active:scale-95"
          navigate={~p"/"}
        >
          🟩 <span class="sr-only">Aligned Explorer</span>
        </.link>
        <.link class="text-foreground/80 hover:text-foreground font-semibold" navigate={~p"/batches"}>
          Batches
        </.link>
        <.live_component module={SearchComponent} id="nav_search" />
      </div>
      <div class="items-center gap-4 font-semibold leading-6 text-foreground/80 flex [&>a]:hidden sm:[&>a]:inline-block [&>a]:drop-shadow-md">
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
