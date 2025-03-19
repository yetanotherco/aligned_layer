defmodule FooterComponent do
  use ExplorerWeb, :live_component

  @impl true
  def mount(socket) do
    {:ok,
     assign(socket,
       headers: [
         {"General",
          [
            {"Batches", "/batches"},
            {"Operators", "/operators"},
            {"Restake", "/restaked"}
          ]},
         {"Social",
          [
            {"Twitter", "https://x.com/alignedlayer"},
            {"Telegram", "https://t.me/aligned_layer"},
            {"Discord", "https://discord.gg/alignedlayer"},
            {"Youtube", "https://youtube.com/@alignedlayer"}
          ]},
         {"Developers",
          [
            {"Docs", "https://docs.alignedlayer.com/"},
            {"Supported verifiers",
             "https://docs.alignedlayer.com/architecture/0_supported_verifiers"},
            {"Whitepaper", "https://alignedlayer.com/whitepaper/"},
            {"Github", "https://github.com/yetanotherco/aligned_layer"}
          ]},
         {"Resources",
          [
            {"Blog", "https://blog.alignedlayer.com/"},
            {"Contact", "https://alignedlayer.com/contact/"}
          ]}
       ]
     )}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="w-full border-t border-foreground/10 backdrop-blur-lg backdrop-saturate-200">
      <div
        class="w-full flex justify-center flex-wrap p-5 pb-10 gap-5 m-auto"
        style="max-width: 75rem;"
      >
        <div class="hidden sm:inline-block flex-1">
          <.link
            class="text-xl font-medium hover:scale-105 transform duration-150 active:scale-95"
            navigate={~p"/"}
          >
            🟩 <span class="text-lg text-foreground ml-2">Aligned Layer</span>
          </.link>
        </div>

        <div>
          <div class="flex-1 flex flex-wrap gap-10 md:gap-32">
            <%= for {title, links} <- @headers do %>
              <div class="flex flex-col items-start gap-2">
                <h3 class="text-foreground font-bold text-lg"><%= title %></h3>
                <%= for {value, link} <- links do %>
                  <.link class="text-md text-foreground/80 hover:underline" href={link}>
                    <%= value %>
                  </.link>
                <% end %>
              </div>
            <% end %>
          </div>
        </div>
      </div>
    </div>
    """
  end
end
