defmodule LabeledProgressBarComponent do
  use ExplorerWeb, :live_component

  attr(:percent_progress, :float, required: true)
  attr(:label, :string, required: true)

  @impl true
  def render(assigns) do
    ~H"""
    <div class="w-full relative weight-700 rounded-lg">
      <div class="w-full bg-accent/30 rounded-2xl">
        <p class="ml-2 normal-case text-center relative text-accent-foreground font-bold z-10">
          <%= @label %>
        </p>
      </div>
      <div
        class="top-0 left-0 h-full bg-accent p-1 rounded-2xl absolute transition-all"
        style={"width: #{@percent_progress}%"}
      >
      </div>
    </div>
    """
  end

  def update_assigns(assigns, socket) do
    progress = assigns[:percent_progress] || 0
    label = assigns[:label] || ""
    {:ok, assign(socket, percent_progress: progress, label: label)}
  end
end
