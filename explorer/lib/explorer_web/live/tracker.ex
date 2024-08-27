defmodule OperatorVersionTracker do
  require Logger
  @tracker_api_url Application.compile_env(:explorer, :tracker_api_url)

  # /versions endpoint
  def get_operators_version() do
    case HTTPoison.get("#{@tracker_api_url}/versions") do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body
        |> parse_as_map()

      {:error, reason} ->
        reason |> Logger.error()
    end
  end

  def parse_as_map(body) do
    body
    |> Jason.decode!()
    |> Enum.reduce(%{}, fn %{"address" => address, "version" => version}, acc ->
      Map.put(acc, address, version)
    end)
  end

  # /versions/:address endpoint
  def get_operator_version(address) do
    case HTTPoison.get("#{@tracker_api_url}/versions/#{address}") do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        body
        |> parse_version()

      {
        :ok,
        %HTTPoison.Response{status_code: _, body: _}
      } ->
        Logger.debug("Operator version not found for address: #{address}")
        nil

      {:error, reason} ->
        reason |> Logger.error()
    end
  end

  def parse_version(body) do
    body
    |> Jason.decode!()
    |> Map.get("version")
  end
end
