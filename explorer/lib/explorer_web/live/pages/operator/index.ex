defmodule ExplorerWeb.Operator.Index do
  use ExplorerWeb, :live_view

  @impl true
  def mount(params, _, socket) do
    address = params["address"]
    operator = Operators.get_operator_by_address(address)
    {:ok, assign(socket, operator: operator, page_title: "Operators")}
  end
end
