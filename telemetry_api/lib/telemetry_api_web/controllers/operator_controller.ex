defmodule TelemetryApiWeb.OperatorController do
  use TelemetryApiWeb, :controller

  alias TelemetryApi.Operators
  alias TelemetryApi.Operators.Operator

  action_fallback TelemetryApiWeb.FallbackController

  defp return_error(conn, message) do
    conn
    |> put_status(:bad_request)
    |> put_resp_content_type("application/json")
    |> send_resp(:bad_request, Jason.encode!(%{error: message}))
  end

  def index(conn, _params) do
    operators = Operators.list_operators()
    render(conn, :index, operators: operators)
  end

  def create(conn, operator_params) do
    case Operators.update_operator_version(operator_params) do
      {:ok, %Operator{} = operator} ->
        conn
        |> put_status(:created)
        |> put_resp_header("location", ~p"/api/operators/#{operator}")
        |> render(:show, operator: operator)

      {:error, message} ->
        return_error(conn, message)
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
end
