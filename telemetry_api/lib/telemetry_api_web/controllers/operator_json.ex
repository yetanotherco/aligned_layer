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
      id: operator.id,
      address: operator.address,
      version: operator.version
    }
  end
end
