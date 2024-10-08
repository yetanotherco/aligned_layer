defmodule TelemetryApi.UrlsTest do
  use TelemetryApi.DataCase

  alias TelemetryApi.Urls

  describe "operators" do
    alias TelemetryApi.Urls.Operator

    import TelemetryApi.UrlsFixtures

    @invalid_attrs %{version: nil, address: nil}

    test "list_operators/0 returns all operators" do
      operator = operator_fixture()
      assert Urls.list_operators() == [operator]
    end

    test "get_operator!/1 returns the operator with given id" do
      operator = operator_fixture()
      assert Urls.get_operator!(operator.id) == operator
    end

    test "create_operator/1 with valid data creates a operator" do
      valid_attrs = %{version: "some version", address: "some address"}

      assert {:ok, %Operator{} = operator} = Urls.create_operator(valid_attrs)
      assert operator.version == "some version"
      assert operator.address == "some address"
    end

    test "create_operator/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Urls.create_operator(@invalid_attrs)
    end

    test "update_operator/2 with valid data updates the operator" do
      operator = operator_fixture()
      update_attrs = %{version: "some updated version", address: "some updated address"}

      assert {:ok, %Operator{} = operator} = Urls.update_operator(operator, update_attrs)
      assert operator.version == "some updated version"
      assert operator.address == "some updated address"
    end

    test "update_operator/2 with invalid data returns error changeset" do
      operator = operator_fixture()
      assert {:error, %Ecto.Changeset{}} = Urls.update_operator(operator, @invalid_attrs)
      assert operator == Urls.get_operator!(operator.id)
    end

    test "delete_operator/1 deletes the operator" do
      operator = operator_fixture()
      assert {:ok, %Operator{}} = Urls.delete_operator(operator)
      assert_raise Ecto.NoResultsError, fn -> Urls.get_operator!(operator.id) end
    end

    test "change_operator/1 returns a operator changeset" do
      operator = operator_fixture()
      assert %Ecto.Changeset{} = Urls.change_operator(operator)
    end
  end
end
