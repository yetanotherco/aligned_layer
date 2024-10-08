# Frontend Utils
defmodule ExplorerWeb.Helpers do
  def shorten_hash(hash, decimals \\ 6) do
    case String.length(hash) do
      n when n < decimals -> hash
      _ -> "#{String.slice(hash, 0, decimals)}...#{String.slice(hash, -4, 4)}"
    end
  end

  def convert_number_to_shorthand(number) when number >= 1_000_000 do
    "#{div(number, 1_000_000)}M"
  end

  def convert_number_to_shorthand(number) when number >= 10_000 do
    "#{div(number, 10_000)}k"
  end

  def convert_number_to_shorthand(number) when number >= 1_000 do
    "#{div(number, 1_000)}k"
  end

  def convert_number_to_shorthand(number) when number >= 0 do
    "#{number}"
  end

  def convert_number_to_shorthand(_number), do: "Invalid number"

  def parse_timestamp(timestamp) do
    %{hour: hour, minute: minute, second: second, day: day, month: month, year: year} = timestamp

    formatted_hour = pad_leading_zero(hour)
    formatted_minute = pad_leading_zero(minute)
    formatted_second = pad_leading_zero(second)
    formatted_day = pad_leading_zero(day)
    formatted_month = format_month(month)

    "#{formatted_hour}:#{formatted_minute}:#{formatted_second} (UTC) - #{formatted_month} #{formatted_day}, #{year}"
  end

  def parse_timeago(timestamp) do
    diff_seconds = DateTime.utc_now() |> DateTime.diff(timestamp)

    cond do
      diff_seconds < 60 -> "Just now"
      diff_seconds < 3600 -> format_minutes(diff_seconds)
      diff_seconds < 86400 -> format_hours(diff_seconds)
      true -> format_days(diff_seconds)
    end
  end

  defp format_minutes(seconds) do
    minutes = div(seconds, 60)
    pluralize(minutes, "min")
  end

  defp format_hours(seconds) do
    hours = div(seconds, 3600)
    pluralize(hours, "hr")
  end

  defp format_days(seconds) do
    days = div(seconds, 86400)
    pluralize(days, "day")
  end

  defp pluralize(1, unit), do: "1 #{unit} ago"
  defp pluralize(count, unit), do: "#{count} #{unit}s ago"

  def format_month(num) do
    case num do
      1 -> "Jan"
      2 -> "Feb"
      3 -> "Mar"
      4 -> "Apr"
      5 -> "May"
      6 -> "Jun"
      7 -> "Jul"
      8 -> "Aug"
      9 -> "Sep"
      10 -> "Oct"
      11 -> "Nov"
      12 -> "Dec"
      _ -> ""
    end
  end

  def format_number(number) do
    Numbers.format_number(number)
  end

  defp pad_leading_zero(value) do
    Integer.to_string(value) |> String.pad_leading(2, "0")
  end

  @doc """
  Get the EigenLayer Explorer URL based on the environment.
  - `holesky` -> https://holesky.eigenlayer.xyz
  - `mainnet` -> https://app.eigenlayer.xyz
  - `default` -> http://localhost:4000
  """
  def get_eigenlayer_explorer_url() do
    prefix = System.get_env("ENVIRONMENT")

    case prefix do
      "holesky" -> "https://holesky.eigenlayer.xyz"
      "mainnet" -> "https://app.eigenlayer.xyz"
      _ -> "http://localhost:4000"
    end
  end

  @doc """
  Get the Etherscan URL based on the environment.
  - `holesky` -> https://holesky.etherscan.io
  - `mainnet` -> https://etherscan.io
  - `default` -> http://localhost:4000
  """
  def get_etherescan_url() do
    prefix = System.get_env("ENVIRONMENT")

    case prefix do
      "mainnet" -> "https://etherscan.io"
      "holesky" -> "https://holesky.etherscan.io"
      _ -> "http://localhost:4000"
    end
  end

  def binary_to_hex_string(binary) do
    Utils.binary_to_hex_string(binary)
  end
end

# Backend utils
defmodule Utils do
  require Logger

  def string_to_bytes32(hex_string) do
    # Remove the '0x' prefix
    hex =
      case hex_string do
        "0x" <> _ -> String.slice(hex_string, 2..-1//1)
        _ -> raise "Invalid hex string, missing '0x' prefix"
      end

    # Convert the hex string to a binary
    case Base.decode16(hex, case: :mixed) do
      {:ok, binary} -> binary
      _ -> raise "Invalid hex string"
    end
  end

  def hex_string_to_int(hex_string) do
    hex_string |> String.replace_prefix("0x", "") |> String.to_integer(16)
  end

  def binary_to_hex_string(nil), do: "0x"
  def binary_to_hex_string(<<>>), do: "0x"

  def binary_to_hex_string(binary) do
    hex_string = binary |> Base.encode16(case: :lower)
    "0x" <> hex_string
  end

  def get_last_n_items(events, n) when is_list(events) and is_integer(n) and n >= 0 do
    events
    |> Enum.reverse()
    |> Enum.take(n)
    |> Enum.reverse()
  end

  def fetch_eigen_operator_metadata(url) do
    case Finch.build(:get, url) |> Finch.request(Explorer.Finch) do
      {:ok, %Finch.Response{status: 200, body: body}} ->
        case Jason.decode(body) do
          {:ok, json} ->
            {:ok, EigenOperatorMetadataStruct.map_to_struct(json)}

          {:error, reason} ->
            {:error, reason}
        end

      {:ok, %Finch.Response{status: status_code}} ->
        {:error, {:http_error, status_code}}

      {:error, reason} ->
        {:error, {:http_error, reason}}
    end
  end

  def random_id(prefix) do
    prefix <> "_" <> (:crypto.strong_rand_bytes(8) |> Base.url_encode64(padding: false))
  end
end
