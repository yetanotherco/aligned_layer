defmodule TelemetryApiWeb.TraceJSON do
  @doc """

  """
  def show_merkle(%{merkle_root: merkle_root}) do
    %{
      merkle_root: merkle_root
    }
  end

  @doc """

  """
  def show_operator(%{operator_id: operator_id}) do
    %{
      operator_id: operator_id
    }
  end
end
