defmodule TelemetryApiWeb.OperatorControllerTest do
  use TelemetryApiWeb.ConnCase

  import TelemetryApi.UrlsFixtures

  alias TelemetryApi.Urls.Operator

  @create_attrs %{
    version: "some version",
    address: "some address"
  }
  @update_attrs %{
    version: "some updated version",
    address: "some updated address"
  }
  @invalid_attrs %{version: nil, address: nil}

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all operators", %{conn: conn} do
      conn = get(conn, ~p"/api/operators")
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create operator" do
    test "renders operator when data is valid", %{conn: conn} do
      conn = post(conn, ~p"/api/operators", operator: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, ~p"/api/operators/#{id}")

      assert %{
               "id" => ^id,
               "address" => "some address",
               "version" => "some version"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, ~p"/api/operators", operator: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update operator" do
    setup [:create_operator]

    test "renders operator when data is valid", %{conn: conn, operator: %Operator{id: id} = operator} do
      conn = put(conn, ~p"/api/operators/#{operator}", operator: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, ~p"/api/operators/#{id}")

      assert %{
               "id" => ^id,
               "address" => "some updated address",
               "version" => "some updated version"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, operator: operator} do
      conn = put(conn, ~p"/api/operators/#{operator}", operator: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete operator" do
    setup [:create_operator]

    test "deletes chosen operator", %{conn: conn, operator: operator} do
      conn = delete(conn, ~p"/api/operators/#{operator}")
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, ~p"/api/operators/#{operator}")
      end
    end
  end

  defp create_operator(_) do
    operator = operator_fixture()
    %{operator: operator}
  end
end
