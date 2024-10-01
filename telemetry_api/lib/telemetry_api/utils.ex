defmodule TelemetryApi.Utils do
  use TelemetryApiWeb, :controller

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


  @doc """
  Returns json encoded error using http
  """
  def return_error(conn, message) do
    conn
      |> put_status(:bad_request)
      |> put_resp_content_type("application/json")
      |> send_resp(:bad_request, Jason.encode!(%{error: message}))
  end

  @doc """
  Validates the existance of a given list of keys in provided params map. 
  Extra keys will be filtered out.

  ## Examples

      iex> required_keys = ["hello", "bye"]
      iex> params = %{"hello": 4, "bye": 2, "dog": 100}
      iex> params_validation(required_keys, params)
      {:ok, %{"hello": 4, "bye": 2}}

      iex> required_keys = ["hello", "bye"]
      iex> params = %{"hello": 4}
      iex> params_validation(required_keys, params)
      {:error, string}
  """
  def params_validation(required_keys, params) do 
    # Check if all required keys are present
    missing_keys = Enum.filter(required_keys, &(!Map.has_key?(params, &1)))
    
    if Enum.empty?(missing_keys) do
      # Filter the params to only include the required keys
      filtered_params = Map.take(params, required_keys)
      {:ok, filtered_params}
    else
      {:error, "Missing required parameters: #{Enum.join(missing_keys, ", ")}"}
    end
  end

end
