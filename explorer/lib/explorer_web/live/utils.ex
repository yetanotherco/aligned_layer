defmodule ExplorerWeb.Utils do
  def shorten_hash(hash) do
    case String.length(hash) do
      n when n < 6 -> hash
      _ -> "#{String.slice(hash, 0, 6)}...#{String.slice(hash, -4, 4)}"
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

  def format_number(number) when is_integer(number) do
    number
    |> Integer.to_string()
    |> String.reverse()
    |> String.graphemes()
    |> Enum.chunk_every(3)
    |> Enum.join(",")
    |> String.reverse()
  end

  defp pad_leading_zero(value) do
    Integer.to_string(value) |> String.pad_leading(2, "0")
  end
end

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

  def get_last_n_items(events, n) when is_list(events) and is_integer(n) and n >= 0 do
    events
    |> Enum.reverse()
    |> Enum.take(n)
    |> Enum.reverse()
  end

  def extract_amount_of_proofs_from_json({:ok, batch_json}) do
    batch_json |> Enum.count()
  end

  def extract_amount_of_proofs_from_json({:error, _}) do
    300
  end

  def fetch_batch_data_pointer(batch_data_pointer) do
    case Finch.build(:get, batch_data_pointer) |> Finch.request(Explorer.Finch) do
      {:ok, %Finch.Response{status: 200, body: body}} ->
        case Jason.decode(body) do
          {:ok, json} -> {:ok, json}
          {:error, reason} -> {:error, {:json_decode, reason}}
        end

      {:ok, %Finch.Response{status: status_code}} ->
        {:error, {:http_error, status_code}}

      {:error, reason} ->
        {:error, {:http_error, reason}}
    end
  end

  def extract_amount_of_proofs(%BatchDB{} = batch) do
    IO.inspect("Extracting amount of proofs for batch: #{batch.merkle_root}")
    # only get from s3 if not already in DB
    amount_of_proofs =
      case Batches.get_amount_of_proofs(%{merkle_root: batch.merkle_root}) do
        nil ->
          IO.inspect("Fetching from S3")

          batch.data_pointer
          |> Utils.fetch_batch_data_pointer()
          |> Utils.extract_amount_of_proofs_from_json()

        proofs ->
          IO.inspect("Fetching from DB")
          proofs
      end

    Map.put(batch, :amount_of_proofs, amount_of_proofs)
  end
end
