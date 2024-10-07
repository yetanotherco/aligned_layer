defmodule TelemetryApi.UrlsFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `TelemetryApi.Urls` context.
  """

  @doc """
  Generate a operator.
  """
  def operator_fixture(attrs \\ %{}) do
    {:ok, operator} =
      attrs
      |> Enum.into(%{
        address: "some address",
        version: "some version"
      })
      |> TelemetryApi.Urls.create_operator()

    operator
  end
end
