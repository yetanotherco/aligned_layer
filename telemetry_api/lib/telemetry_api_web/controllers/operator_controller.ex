defmodule TelemetryApiWeb.OperatorController do
  use TelemetryApiWeb, :controller

  alias TelemetryApi.Operators
  alias TelemetryApi.Operators.Operator

  action_fallback(TelemetryApiWeb.FallbackController)

  def index(conn, _params) do
    operators = Operators.list_operators()
    render(conn, :index, operators: operators)
  end

  def create_or_update(conn, operator_params) do
    with {:ok, %Operator{} = operator} <- Operators.update_operator_version(operator_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", ~p"/api/operators/#{operator}")
      |> render(:show, operator: operator)
    end
  end

  def show(conn, %{"id" => address}) do
    with {%Operator{} = operator} <- Operators.get_operator(%Operator{address: address}) do
      render(conn, :show, operator: operator)
    end
  end
end
