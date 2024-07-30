defmodule ExplorerWeb.Operators.Index do
  use ExplorerWeb, :live_view

  @impl true
  def mount(_, _, socket) do
    {:ok, assign(socket, page_title: "Operators")}
  end

  @impl true
  def handle_params(_params, _url, socket) do
    operators = Operators.get_operators()
    {:noreply, assign(socket, operators: operators)}
  end

  embed_templates "*"
end
