defmodule TelemetryApi.Utils do
  @moduledoc """
  Some utility functions
  """

  @doc """
  Fetches the provided url and returns a json decoded map

  ## Examples

      iex> fetch_json_data(url)
      {:ok, data}

      iex> fetch_json_data(url)
      {:error, message}
  """
  def fetch_json_data(url) do
    with {:ok, %HTTPoison.Response{status_code: 200, body: body}} <- HTTPoison.get(url) do
      Jason.decode(body)
    else
      {:ok, %HTTPoison.Response{status_code: status_code}} ->
        {:error, "Request failed with status #{status_code}"}

      {:error, %HTTPoison.Error{reason: reason}} ->
        {:error, "HTTP request failed: #{reason}"}
    end
  end

  @doc """
  Unwraps inner element status

  ## Examples

      iex> error_message = "Error found on list"
      iex> list = [{:ok, 2}, {:ok, 3}]
      iex> check_list_status(list, error_message)
      {:ok, list}

      iex> list = [{:ok, 2}, {:ok, 3}, {:error, "this is an error"}]
      iex> check_list_status(list, error_message)
      {:error, "Error found on list"}
  """
  def check_list_status(list, error_message) do
    case Enum.find(list, fn {status, _} -> status == :error end) do
      nil ->
        {:ok, Enum.map(list, fn {:ok, value} -> value end)}

      {:error, _} ->
        {:error, error_message}
    end
  end
end
