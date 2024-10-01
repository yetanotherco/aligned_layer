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
    case HTTPoison.get(url) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        {:ok, Jason.decode!(body)}

      {:ok, %HTTPoison.Response{status_code: status_code}} ->
        {:error, "Request failed with status #{status_code}"}

      {:error, %HTTPoison.Error{reason: reason}} ->
        {:error, "HTTP request failed: #{reason}"}
    end
  end

  @doc """
  Unwraps inner element status 

  ## Examples

      iex> list = [{:ok, 2}, {:ok, 3}] 
      iex> clean_list_errors(list, error_message)
      {:ok, list}

      iex> list = [{:ok, 2}, {:ok, 3}, {:error, "this is an error"}] 
      iex> message = "Error found on list"
      iex> clean_list_errors(list, error_message)
      {:error, "Error found on list"}
  """
  def clean_list_errors(list, error_message) do
    case Enum.find(list, fn {status, _} -> status == :error end) do
      nil ->
        {:ok, Enum.map(list, fn {:ok, value} -> value end)}

      {:error, _} ->
        {:error, error_message}
    end
  end
end
