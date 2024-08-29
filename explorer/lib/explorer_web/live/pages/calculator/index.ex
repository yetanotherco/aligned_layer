defmodule ExplorerWeb.Calculator.Index do
  use ExplorerWeb, :live_view

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col space-y-3 px-1 text-foreground max-w-[27rem] sm:max-w-3xl md:max-w-5xl mx-auto">
      <.card_preheding>
        Calculator
      </.card_preheding>
    </div>
    """
  end
end
