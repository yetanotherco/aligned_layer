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
      AlignedProofAggregationService.EventFilters.new_aggregated_proof(nil)
      |> Ethers.get_logs(fromBlock: fromBlock, toBlock: toBlock)

    case events do
      {:ok, []} ->
        []

      {:ok, list} ->
        Enum.map(list, fn x ->
          data = x |> Map.get(:data)
          topics_raw = x |> Map.get(:topics_raw)
          block_number = x |> Map.get(:block_number)
          tx_hash = x |> Map.get(:transaction_hash)

          {
            :ok,
            %{
              number: topics_raw |> Enum.at(1),
              status: data |> Enum.at(0),
              merkle_root: data |> Enum.at(1),
              blob_versioned_hash: data |> Enum.at(2),
              block_number: block_number,
              tx_hash: tx_hash
            }
          }
        end)

      {:error, reason} ->
        raise("Error fetching events: #{Map.get(reason, "message")}")
    end
  end

  def get_blob_data_from_versioned_hash(aggregated_proof) do
    case BeaconClient.fetch_blob_by_versioned_hash(
           aggregated_proof.block_number,
           aggregated_proof.blob_versioned_hash
         ) do
      {:ok, data} -> data.blob
      {:error, reason} -> {:error, reason}
    end
  end
end
