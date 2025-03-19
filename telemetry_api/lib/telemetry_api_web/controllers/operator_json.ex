defmodule TelemetryApiWeb.OperatorJSON do
  alias TelemetryApi.Operators.Operator

  @doc """
  Renders a list of operators.
  """
  def index(%{operators: operators}) do
    for(operator <- operators, do: data(operator))
  end

  @doc """
  Renders a single operator.
  """
  def show(%{operator: operator}) do
    data(operator)
  end

  defp data(%Operator{} = operator) do
    %{
      address: operator.address,
      id: operator.id,
      stake: operator.stake,
      name: operator.name,
      version: operator.version,
      status: operator.status,
      eth_rpc_url: operator.eth_rpc_url,
      eth_rpc_url_fallback: operator.eth_rpc_url_fallback,
      eth_ws_url: operator.eth_ws_url,
      eth_ws_url_fallback: operator.eth_ws_url_fallback
    }
  end

  @doc """
  Renders a list of operators with only public data.
  """
  def index_public(%{operators: operators}) do
    for(operator <- operators, do: data_public(operator))
  end

  @doc """
  Renders a single operator with only public data.
  """
  def show_public(%{operator: operator}) do
    data_public(operator)
  end

  defp data_public(%Operator{} = operator) do
    %{
      address: operator.address,
      id: operator.id,
      stake: operator.stake,
      name: operator.name,
      version: operator.version
    }
  end
end
