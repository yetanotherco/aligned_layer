defmodule DelegationManager do
  require Logger

  @aligned_config_file System.get_env("ALIGNED_CONFIG_FILE")

  config_file_path =
    case @aligned_config_file do
      nil -> raise("ALIGNED_CONFIG_FILE not set in .env")
      file -> file
    end

  {status, config_json_string} = File.read(config_file_path)

  case status do
    :ok ->
      Logger.debug("Eigenlayer deployment file read successfully")

    :error ->
      raise(
        "Config file not read successfully, did you run make create-env? If you did,\n make sure Eigenlayer config file is correctly stored"
      )
  end

  @delegation_manager_address Jason.decode!(config_json_string)
                         |> Map.get("addresses")
                         |> Map.get("delegationManager")

  # @first_block (case @environment do
  #                 "devnet" -> 0
  #                 "holesky" -> 1_600_000
  #                 "mainnet" -> 20_020_000
  #                 _ -> raise("Invalid environment")
  #               end)

  use Ethers.Contract,
    abi_file: "lib/abi/DelegationManager.json",
    default_address: @delegation_manager_address

  def get_delegation_manager_address() do
    @delegation_manager_address
  end

  def get_operator_url(operator_address) do
    DelegationManager.EventFilters.operator_metadata_uri_updated(operator_address)
      |> Ethers.get_logs(fromBlock: 0)
      |> case do
        {:ok, data} -> List.last(data).data |> hd() # most recent entry

        {:error, reason} ->
          IO.inspect("Error getting operator url")
          IO.inspect(reason)

        other ->
          IO.inspect("Unexpected response:")
          IO.inspect(other)
      end
  end


end
