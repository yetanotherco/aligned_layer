defmodule AlignedProofAggregationService do
  require Logger

  @aligned_config_file System.get_env("ALIGNED_PROOF_AGG_CONFIG_FILE")

  config_file_path =
    case @aligned_config_file do
      nil -> raise("ALIGNED_PROOF_AGG_CONFIG_FILE not set in .env")
      file -> file
    end

  {status, config_json_string} = File.read(config_file_path)

  case status do
    :ok ->
      Logger.debug("Aligned deployment file read successfully")

    :error ->
      raise(
        "Config file not read successfully, make sure your .env is correctly created, and make sure Eigenlayer config file is correctly stored"
      )
  end

  @contract_address Jason.decode!(config_json_string)
                    |> Map.get("addresses")
                    |> Map.get("alignedProofAggregationService")

  use Ethers.Contract,
    abi_file: "lib/abi/AlignedProofAggregationService.json",
    default_address: @contract_address

  def get_address() do
    @contract_address
  end

  def get_aggregated_proof_event(%{from_block: fromBlock, to_block: toBlock}) do
    events =
      AlignedProofAggregationService.EventFilters.aggregated_proof_verified(nil)
      |> Ethers.get_logs(fromBlock: fromBlock, toBlock: toBlock)

    case events do
      {:ok, []} ->
        {:ok, []}

      {:ok, list} ->
        {:ok,
         Enum.map(list, fn x ->
           data = x |> Map.get(:data)
           topics_raw = x |> Map.get(:topics_raw)
           block_number = x |> Map.get(:block_number)
           tx_hash = x |> Map.get(:transaction_hash)

           %{
             merkle_root:
               topics_raw
               |> Enum.at(1),
             blob_versioned_hash: "0x" <> Base.encode16(data |> Enum.at(0), case: :lower),
             block_number: block_number,
             block_timestamp: get_block_timestamp(block_number),
             tx_hash: tx_hash
           }
         end)}

      {:error, reason} ->
        {:error, reason}
    end
  end

  def get_block_timestamp(block_number) do
    case Ethers.Utils.get_block_timestamp(block_number) do
      {:ok, timestamp} -> DateTime.from_unix!(timestamp)
      {:error, error} -> raise("Error fetching block timestamp: #{error}")
    end
  end

  def get_blob_data!(aggregated_proof) do
    {:ok, block} =
      Explorer.EthClient.get_block_by_number(
        Explorer.Utils.decimal_to_hex(aggregated_proof.block_number)
      )

    parent_beacon_block_hash = Map.get(block, "parentBeaconBlockRoot")

    {:ok, beacon_block} =
      Explorer.BeaconClient.get_block_header_by_parent_hash(parent_beacon_block_hash)

    slot = Explorer.BeaconClient.get_block_slot(beacon_block)

    data =
      Explorer.BeaconClient.fetch_blob_by_versioned_hash!(
        slot,
        aggregated_proof.blob_versioned_hash
      )

    Map.get(data, "blob")
  end

  @doc """
  Decodes blob data represented as an ASCII charlist.
  """
  def decode_blob(blob_data), do: decode_blob(blob_data, [[]], 0, 0, 0)

  defp decode_blob([], acc, _current_count, _total_count, _i), do: acc

  defp decode_blob([head | tail], acc, current_count, total_count, i) do
    # Every 64 characters (or 32 bytes) there is a 00 for padding
    should_skip = rem(total_count, 64) == 0

    case should_skip do
      true ->
        [_head | tail] = tail
        decode_blob(tail, acc, current_count, total_count + 2, i)

      false ->
        acc = List.update_at(acc, i, fn chunk -> chunk ++ [head] end)

        case current_count + 1 < 64 do
          true ->
            decode_blob(tail, acc, current_count + 1, total_count + 1, i)

          false ->
            current_blob = Enum.at(acc, i)
            # 48 is 0 in ascii
            is_all_zeroes = Enum.all?(current_blob, fn x -> x == 48 end)

            ## If the hash is all zeroed, then there are no more hashes in the blob
            if is_all_zeroes do
              # Drop last limiter zeroed element
              Enum.drop(acc, -1)
            else
              decode_blob(tail, acc ++ [[]], 0, total_count + 1, i + 1)
            end
        end
    end
  end
end
