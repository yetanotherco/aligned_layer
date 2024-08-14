defmodule CopyToClipboardButtonComponent do
  use ExplorerWeb, :live_component

  @impl true
  def update(assigns, socket) do
    {:ok, assign(socket, text_to_copy: assigns.text_to_copy, class: assigns.class)}
  end

  @impl true
  def handle_event("copied", _params, socket) do
    text = socket.assigns.text_to_copy
    {:noreply, put_flash!(socket, :info, "Copied #{text} to clipboard!")}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <button
      class={[
        "flex items-center justify-center size-7 rounded-full bg-foreground/10 hover:bg-foreground/20 text-foreground/80 hover:text-foreground/100",
        @class
      ]}
      phx-hook="CopyToClipboard"
      data-clipboard-text={@text_to_copy}
      id={"copy-to-clipboard-" <> @text_to_copy}
      phx-target={@myself}
      phx-click="copied"
    >
      <.icon name="hero-clipboard" class="size-3" />
      <span class="sr-only">Copy to clipboard</span>
    </button>
    """
  end
end
