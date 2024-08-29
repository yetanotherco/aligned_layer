defmodule OperatorVersionTracker do
  require Logger
  @tracker_api_url Application.compile_env(:explorer, :tracker_api_url)

  def get_operators_version() do
    get_operators_version(@tracker_api_url)
  end

  defp get_operators_version(nil), do: %{}
  defp get_operators_version(""), do: %{}

  defp get_operators_version(url) do
    clean_url = String.trim_trailing(url, "/")

    case HTTPoison.get("#{clean_url}/versions") do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body
        |> parse_as_map()

      {:ok, %HTTPoison.Response{status_code: _, body: _}} ->
        Logger.debug("Operator versions not found.")
        %{}

      {:error, reason} ->
        "Error while fetching operator versions." |> Logger.error()
        reason |> Logger.error()
        %{}

      [] ->
        "Empty response received while while fetching operator versions." |> Logger.debug()
        %{}
    end
  end

  def parse_as_map(body) do
    body
    |> Jason.decode!()
    |> Enum.reduce(%{}, fn %{"address" => address, "version" => version}, acc ->
      Map.put(acc, address, version)
    end)
  end

  def get_operator_version(address) do
    get_operator_version(@tracker_api_url, address)
  end

  defp get_operator_version(nil, _address), do: nil
  defp get_operator_version("", _address), do: nil

  defp get_operator_version(url, address) do
    clean_url = String.trim_trailing(url, "/")

    case HTTPoison.get("#{clean_url}/versions/#{address}") do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body
        |> parse_version()

      {:ok, %HTTPoison.Response{status_code: _, body: _}} ->
        Logger.debug("Operator version not found.")
        nil

      {:error, _reason} ->
        "Error while fetching operator version. Address: #{address}." |> Logger.error()
        nil

      [] ->
        "Empty response received while fetching operator version. Address: #{address}."
        |> Logger.error()

        nil
    end
  end

  def parse_version(body) do
    body
    |> Jason.decode!()
    |> Map.get("version")
  end
end
