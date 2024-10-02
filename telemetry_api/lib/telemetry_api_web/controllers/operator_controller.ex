defmodule TelemetryApiWeb.OperatorController do
  use TelemetryApiWeb, :controller

  alias TelemetryApi.Operators
  alias TelemetryApi.Utils
  alias TelemetryApi.Operators.Operator

  action_fallback TelemetryApiWeb.FallbackController

  @create_params [
    "version",
    "signature",
    "eth_rpc_url",
    "eth_rpc_url_fallback",
    "eth_ws_url",
    "eth_ws_url_fallback"
  ]

  def index(conn, _params) do
    operators = Operators.list_operators()
    render(conn, :index, operators: operators)
  end

  def create(conn, params) do
    params = Map.take(params, @create_params) 
    with {:ok, %Operator{} = operator} <- Operators.update_operator(params) do
      conn
        |> put_status(:created)
        |> put_resp_header("location", ~p"/api/operators/#{operator}")
        |> render(:show, operator: operator)
    else
      {:error, message} ->
        Utils.return_error(conn, message)
      _ ->
        Utils.return_error(conn, "Unknown error while updating operator")
    end
  end

  def show(conn, %{"id" => address}) do
    case Operators.get_operator(address) do
      %Operator{} = operator ->
        render(conn, :show, operator: operator)

      nil ->
        conn
        |> put_status(:not_found)
        |> put_view(TelemetryApiWeb.ErrorJSON)
        |> render("404.json", %{})
    end
  end

  # defp update(conn, operator, operator_params) do
  #   with {:ok, %Operator{} = operator} <- Operators.update_operator(operator, operator_params) do
  #     conn
  #     |> put_status(:ok)
  #     |> render(:show, operator: operator)
  #   end
  # end

  # def delete(conn, %{"id" => id}) do
  #   operator = Operators.get_operator!(id)

  #   with {:ok, %Operator{}} <- Operators.delete_operator(operator) do
  #     send_resp(conn, :no_content, "")
  #   end
  # end
end
