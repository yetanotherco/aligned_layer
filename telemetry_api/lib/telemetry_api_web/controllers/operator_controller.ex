defmodule TelemetryApiWeb.OperatorController do
  use TelemetryApiWeb, :controller

  alias TelemetryApi.Operators
  alias TelemetryApi.Operators.Operator

  action_fallback TelemetryApiWeb.FallbackController

  def index(conn, _params) do
    operators = Operators.list_operators()
    render(conn, :index, operators: operators)
  end

  def create(conn, operator_params) do
    with {:ok, %Operator{} = operator} <- Operators.create_operator(operator_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", ~p"/api/operators/#{operator}")
      |> render(:show, operator: operator)
    end
  end

  def show(conn, %{"id" => id}) do
    operator = Operators.get_operator!(id)
    render(conn, :show, operator: operator)
  end

  # def update(conn, %{"id" => id, "operator" => operator_params}) do
  #   operator = Operators.get_operator!(id)

  #   with {:ok, %Operator{} = operator} <- Operators.update_operator(operator, operator_params) do
  #     render(conn, :show, operator: operator)
  #   end
  # end

  # def delete(conn, %{"id" => id}) do
  #   operator = Operators.get_operator!(id)

  #   with {:ok, %Operator{}} <- Operators.delete_operator(operator) do
  #     send_resp(conn, :no_content, "")
  #   end
  # end
end
