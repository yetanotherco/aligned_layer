defmodule TelemetryApiWeb.OperatorController do
  use TelemetryApiWeb, :controller

  alias TelemetryApi.Operators
  alias TelemetryApi.Operators.Operator

  action_fallback(TelemetryApiWeb.FallbackController)

  def index(conn, _params) do
    operators = Operators.list_operators()
    render(conn, :index, operators: operators)
  end

  def create_or_update(conn, %{"version" => version, "signature" => signature} = attrs) do
    with {:ok, %Operator{} = operator} <- Operators.update_operator(version, signature, attrs) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", ~p"/api/operators/#{operator}")
      |> render(:show, operator: operator)
    end
  end

  def show(conn, %{"id" => address}) do
    with {:ok, %Operator{} = operator} <- Operators.get_operator(%{address: address}) do
      render(conn, :show, operator: operator)
    end
  end
end
