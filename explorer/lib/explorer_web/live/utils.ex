# Frontend Utils
defmodule ExplorerWeb.Helpers do
  def shorten_hash(hash, decimals \\ 6) do
    case String.length(hash) do
      n when n < decimals -> hash
      _ -> "#{String.slice(hash, 0, decimals)}...#{String.slice(hash, -4, 4)}"
    end
  end

  def convert_number_to_shorthand(number) when number >= 1_000_000_000 do
    formatted_number = Float.round(number / 1_000_000_000, 2)
    "#{remove_trailing_zeros(formatted_number)}B"
  end

  def convert_number_to_shorthand(number) when number >= 1_000_000 do
    formatted_number = Float.round(number / 1_000_000, 2)
    "#{remove_trailing_zeros(formatted_number)}M"
  end

  def convert_number_to_shorthand(number) when number >= 1_000 do
    "#{div(number, 1_000)}k"
  end

  def convert_number_to_shorthand(number) when number >= 0 do
    "#{number}"
  end

  def convert_number_to_shorthand(_number), do: "Invalid number"

  defp remove_trailing_zeros(number) do
    if Float.ceil(number) == Float.floor(number) do
      Integer.to_string(trunc(number))
    else
      Float.to_string(Float.round(number, 2))
    end
  end

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
    Returns a list of available AlignedLayer networks with their names and explorer URLs.
  """
  def get_aligned_networks() do
    [
      {"Mainnet", "https://explorer.alignedlayer.com"},
      {"Holesky", "https://holesky.explorer.alignedlayer.com"},
      {"Stage", "https://stage.explorer.alignedlayer.com"},
      {"Devnet", "http://localhost:4000/"}
    ]
  end

  def get_current_network_from_host(host) do
    case host do
      "explorer.alignedlayer.com" -> "Mainnet"
      "holesky.explorer.alignedlayer.com" -> "Holesky"
      "stage.explorer.alignedlayer.com" -> "Stage"
      _ -> "Devnet"
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

  def get_aligned_contracts_addresses() do
    aligned_config_file = System.get_env("ALIGNED_CONFIG_FILE")
    {_, config_json_string} = File.read(aligned_config_file)
    Jason.decode!(config_json_string) |> Map.get("addresses")
  end

  def binary_to_hex_string(binary) do
    Utils.binary_to_hex_string(binary)
  end

  def is_stale?(batch) do
    ttl = Utils.batch_ttl_minutes()

    DateTime.add(batch.submission_timestamp, ttl, :minute)
    |> DateTime.before?(DateTime.utc_now())
  end

  def get_batch_status(batch) do
    cond do
      not batch.is_valid -> :invalid
      batch.is_verified -> :verified
      is_stale?(batch) -> :stale
      true -> :pending
    end
  end

  def get_next_scheduled_batch_remaining_time() do
    interval = Utils.scheduled_batch_interval()
    latest_batch = Batches.get_latest_verified_batch()

    cond do
      latest_batch != nil ->
        latest_batch.submission_timestamp
        |> DateTime.add(interval, :minute)
        |> DateTime.diff(DateTime.utc_now(), :minute)
        |> max(0)

      true ->
        interval
    end
  end

  def get_next_scheduled_batch_remaining_time_percentage(remaining_time) do
    interval = Utils.scheduled_batch_interval()
    100 * (interval - remaining_time) / interval
  end

  def enrich_batches(batches) do
    batches
    |> Enum.map(fn batch ->
      batch
      |> Map.merge(%{
        age: batch.submission_timestamp |> parse_timeago(),
        status: batch |> get_batch_status
      })
    end)
  end
end

# Backend utils
defmodule Utils do
  require Logger

  @max_batch_size (case System.fetch_env("MAX_BATCH_SIZE") do
                     # empty env var
                     {:ok, ""} -> 268_435_456
                     {:ok, value} -> String.to_integer(value)
                     # error
                     _ -> 268_435_456
                   end)

  @batcher_submission_gas_cost Application.compile_env(:explorer, :batcher_submission_gas_cost)
  @aggregator_gas_cost Application.compile_env(:explorer, :aggregator_gas_cost)
  @aggregator_fee_percentage_multiplier Application.compile_env(:explorer, :aggregator_fee_percentage_multiplier)
  @percentage_divider Application.compile_env(:explorer, :percentage_divider)
  @additional_submission_gas_cost_per_proof Application.compile_env(:explorer, :additional_submission_gas_cost_per_proof)

  def scheduled_batch_interval() do
    default_value = 10
    case System.get_env("SCHEDULED_BATCH_INTERVAL_MINUTES") do
      nil ->
        Logger.warning("SCHEDULED_BATCH_INTERVAL_MINUTES .env var is not set, using default value: #{default_value}")
        default_value
      value ->
        try do
          String.to_integer(value)
        rescue
          ArgumentError ->
            Logger.warning("Invalid SCHEDULED_BATCH_INTERVAL_MINUTES .env var: #{value}, using default value: #{default_value}")
            default_value
        end
    end
  end

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

  def calculate_proof_hashes(deserialized_batch) do
    deserialized_batch
    |> Enum.map(fn s3_object ->
      ExKeccak.hash_256(:erlang.list_to_binary(s3_object["proof"]))
    end)
  end

  defp stream_handler({:headers, headers}, acc) do
    {_, batch_size} = List.keyfind(headers, "content-length", 0, {nil, "0"})
    check_batch_size(String.to_integer(batch_size), acc)
  end

  defp stream_handler({:status, 200}, acc), do: {:cont, acc}

  defp stream_handler({:status, status_code}, _acc),
    do: {:halt, {:error, {:http_error, status_code}}}

  defp stream_handler({:data, chunk}, {acc_body, acc_size}) do
    new_size = acc_size + byte_size(chunk)
    check_batch_size(new_size, {acc_body <> chunk, new_size})
  end

  defp check_batch_size(size, acc) do
    if size > @max_batch_size do
      {:halt, {:error, {:invalid, :body_too_large}}}
    else
      {:cont, acc}
    end
  end

  def fetch_batch_data_pointer(batch_data_pointer) do
    case Finch.build(:get, batch_data_pointer)
         |> Finch.stream_while(Explorer.Finch, {"", 0}, &stream_handler(&1, &2)) do
      {:ok, {:error, reason}} ->
        {:error, reason}

      {:ok, {body, _size}} ->
        cond do
          is_json?(body) ->
            case Jason.decode(body) do
              {:ok, json} ->
                {:ok, json}

              {:error, reason} ->
                {:error, {:json_decode, reason}}
            end

          is_cbor?(body) ->
            case CBOR.decode(body) do
              {:ok, cbor_data, _} ->
                {:ok, cbor_data}

              {:error, reason} ->
                {:error, {:cbor_decode, reason}}
            end

          true ->
            Logger.error("Unknown S3 object format")
            {:error, {:invalid, :unknown_format}}
        end

      {:error, reason} ->
        {:error, {:http_error, reason}}
    end
  end

  defp is_json?(body) do
    case Jason.decode(body) do
      {:ok, _} ->
        true

      {:error, _} ->
        false
    end
  end

  defp is_cbor?(body) do
    case CBOR.decode(body) do
      {:ok, _, _} ->
        true

      {:error, _} ->
        false

      _other ->
        false
    end
  end

  def batch_ttl_minutes() do
    System.get_env("BATCH_TTL_MINUTES")
    |> String.to_integer()
  end

  def process_batch(%BatchDB{} = batch) do
    case get_proof_hashes(batch) do
      {:ok, proof_hashes} ->
        {:ok, add_proof_hashes_to_batch(batch, proof_hashes)}

      {:error, {:invalid, reason}} ->
        Logger.error("Invalid batch content for #{batch.merkle_root}: #{inspect(reason)}")
        # Returning something ensures we avoid attempting to fetch the invalid data again.
        updated_batch =
          batch
          |> Map.put(:is_valid, false)
          |> add_proof_hashes_to_batch([<<0>>])

        {:ok, updated_batch}

      {:error, reason} ->
        {:error, reason}
    end
  end

  defp add_proof_hashes_to_batch(batch, proof_hashes) do
    batch
    |> Map.put(:proof_hashes, proof_hashes)
    |> Map.put(:amount_of_proofs, Enum.count(proof_hashes))
  end

  defp get_proof_hashes(%BatchDB{} = batch) do
    Logger.debug("Extracting batch's proofs info: #{batch.merkle_root}")
    # only get from s3 if not already in DB
    case Proofs.get_proofs_from_batch(%{merkle_root: batch.merkle_root}) do
      nil ->
        Logger.debug("Fetching from S3")

        batch_content = batch.data_pointer |> Utils.fetch_batch_data_pointer()

        case batch_content do
          {:ok, batch_content} ->
            proof_hashes =
              batch_content
              |> Utils.calculate_proof_hashes()

            {:ok, proof_hashes}

          {:error, reason} ->
            {:error, reason}
        end

      proof_hashes ->
        # already processed and stored the S3 data
        Logger.debug("Fetching from DB")
        {:ok, proof_hashes}
    end
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

  def constant_batch_submission_gas_cost() do
    trunc(
      @aggregator_gas_cost * @aggregator_fee_percentage_multiplier /
      @percentage_divider +
      @batcher_submission_gas_cost
    )
  end

  def additional_batch_submission_gas_cost_per_proof() do
    @additional_submission_gas_cost_per_proof
  end
end
