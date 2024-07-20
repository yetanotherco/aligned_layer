defmodule ExplorerWeb.Operators.Index do
  use ExplorerWeb, :live_view

  @impl true
  def mount(_, _, socket) do
    {:ok, assign(socket, page_title: "Operators")}
  end

  embed_templates "*"
end
